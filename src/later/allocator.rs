use core::alloc::{GlobalAlloc, Layout};
use core::ptr::null_mut;
use core::sync::atomic::{AtomicBool, Ordering};

use crate::LINEAR_MAP_BASE;

const PAGE_SIZE: usize = 4096;
const DRAM_START: usize = 0x4000_0000;
const DRAM_END: usize = 0xa000_0000;
const PAGE_COUNT: usize = (DRAM_END - DRAM_START) / PAGE_SIZE;
const USIZE_BITS: usize = usize::BITS as usize;
const BITMAP_WORDS: usize = (PAGE_COUNT + USIZE_BITS - 1) / USIZE_BITS;

static mut BITMAP: [usize; BITMAP_WORDS] = [usize::MAX; BITMAP_WORDS];
static LOCKED: AtomicBool = AtomicBool::new(false);
static INITIALIZED: AtomicBool = AtomicBool::new(false);

unsafe extern "C" {
    static __kernel_linear_end: u8;
}

pub struct KernelAllocator;

#[global_allocator]
static GLOBAL_ALLOCATOR: KernelAllocator = KernelAllocator;

unsafe impl GlobalAlloc for KernelAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        match alloc_phys(layout) {
            Some(phys) => phys_to_virt(phys) as *mut u8,
            None => null_mut(),
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        if ptr.is_null() {
            return;
        }

        let virt = ptr as usize;
        if virt < LINEAR_MAP_BASE {
            return;
        }

        let _ = free_phys(virt - LINEAR_MAP_BASE, layout);
    }
}

struct LockGuard;

impl LockGuard {
    fn lock() -> Self {
        while LOCKED
            .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
            .is_err()
        {
            core::hint::spin_loop();
        }

        Self
    }
}

impl Drop for LockGuard {
    fn drop(&mut self) {
        LOCKED.store(false, Ordering::Release);
    }
}

#[inline(always)]
const fn align_up(value: usize, align: usize) -> usize {
    (value + align - 1) & !(align - 1)
}

#[inline(always)]
const fn phys_to_page(phys: usize) -> usize {
    (phys - DRAM_START) / PAGE_SIZE
}

#[inline(always)]
const fn page_to_phys(page: usize) -> usize {
    DRAM_START + page * PAGE_SIZE
}

#[inline(always)]
const fn phys_to_virt(phys: usize) -> usize {
    phys + LINEAR_MAP_BASE
}

#[inline(always)]
unsafe fn set_used(page: usize, used: bool) {
    let word = page / USIZE_BITS;
    let bit = page % USIZE_BITS;
    let mask = 1usize << bit;

    if used {
        BITMAP[word] |= mask;
    } else {
        BITMAP[word] &= !mask;
    }
}

#[inline(always)]
unsafe fn is_used(page: usize) -> bool {
    let word = page / USIZE_BITS;
    let bit = page % USIZE_BITS;

    (BITMAP[word] & (1usize << bit)) != 0
}

fn pages_for(layout: Layout) -> usize {
    let size = if layout.size() == 0 { 1 } else { layout.size() };
    align_up(size, PAGE_SIZE) / PAGE_SIZE
}

pub fn init() {
    let _guard = LockGuard::lock();

    unsafe {
        for word in 0..BITMAP_WORDS {
            BITMAP[word] = usize::MAX;
        }

        let kernel_end_linear = core::ptr::addr_of!(__kernel_linear_end) as usize;
        let kernel_end = align_up(kernel_end_linear - LINEAR_MAP_BASE, PAGE_SIZE);
        let first_free = if kernel_end < DRAM_START {
            DRAM_START
        } else {
            kernel_end
        };
        let first_free_page = phys_to_page(first_free);

        for page in first_free_page..PAGE_COUNT {
            set_used(page, false);
        }
    }

    INITIALIZED.store(true, Ordering::Release);
}

pub fn alloc_frame() -> Option<usize> {
    alloc_pages(1, PAGE_SIZE)
}

pub fn free_frame(phys: usize) -> bool {
    match Layout::from_size_align(PAGE_SIZE, PAGE_SIZE) {
        Ok(layout) => free_phys(phys, layout),
        Err(_) => false,
    }
}

pub fn alloc_phys(layout: Layout) -> Option<usize> {
    alloc_pages(pages_for(layout), layout.align().max(PAGE_SIZE))
}

pub fn free_phys(phys: usize, layout: Layout) -> bool {
    if !INITIALIZED.load(Ordering::Acquire) {
        return false;
    }

    if phys < DRAM_START || phys >= DRAM_END || phys % PAGE_SIZE != 0 {
        return false;
    }

    let pages = pages_for(layout);
    let start_page = phys_to_page(phys);
    if start_page + pages > PAGE_COUNT {
        return false;
    }

    let _guard = LockGuard::lock();

    unsafe {
        for page in start_page..start_page + pages {
            set_used(page, false);
        }
    }

    true
}

fn alloc_pages(pages: usize, align: usize) -> Option<usize> {
    if pages == 0 || !INITIALIZED.load(Ordering::Acquire) {
        return None;
    }

    let _guard = LockGuard::lock();
    let mut page = 0;

    unsafe {
        while page + pages <= PAGE_COUNT {
            let phys = page_to_phys(page);
            if phys & (align - 1) != 0 {
                page += 1;
                continue;
            }

            let mut free = true;
            for offset in 0..pages {
                if is_used(page + offset) {
                    page += offset + 1;
                    free = false;
                    break;
                }
            }

            if free {
                for allocated in page..page + pages {
                    set_used(allocated, true);
                }

                return Some(phys);
            }
        }
    }

    None
}

pub fn free_pages() -> usize {
    let _guard = LockGuard::lock();
    let mut count = 0;

    unsafe {
        for page in 0..PAGE_COUNT {
            if !is_used(page) {
                count += 1;
            }
        }
    }

    count
}
