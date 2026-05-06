#![no_std]

use core::alloc::Layout;

pub mod allocator;
pub mod head;
pub mod uart;

pub use head::jump_to_main;

const LINEAR_MAP_BASE: usize = 0xffff_ffff_0000_0000;

#[no_mangle]
pub extern "C" fn main() -> ! {
    uart::puts(b"hello from main\r\n");
    allocator::init();
    uart::puts(b"physical allocator initialized, free pages = ");
    uart::put_hex_u64(allocator::free_pages() as u64);
    uart::puts(b"\r\n");

    if let Some(frame) = allocator::alloc_frame() {
        uart::puts(b"alloc frame phys = ");
        uart::put_hex_u64(frame as u64);
        uart::puts(b"\r\n");
        allocator::free_frame(frame);
    }

    match Layout::from_size_align(8192, 4096) {
        Ok(layout) => {
            if let Some(phys) = allocator::alloc_phys(layout) {
                uart::puts(b"core::alloc 8KiB phys = ");
                uart::put_hex_u64(phys as u64);
                uart::puts(b"\r\n");
                allocator::free_phys(phys, layout);
            }
        }
        Err(_) => uart::puts(b"bad allocation layout\r\n"),
    }

    uart::puts(b"free pages after allocator test = ");
    uart::put_hex_u64(allocator::free_pages() as u64);
    uart::puts(b"\r\n");

    uart::puts(b"before clear ttbr0\r\n");

    uart::puts(b"after clear ttbr0\r\n");

    loop {
        core::hint::spin_loop();
    }
}