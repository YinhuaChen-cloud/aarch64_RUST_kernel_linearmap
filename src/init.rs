#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod exception;
mod mmu;
mod start;
mod uart;

#[no_mangle]
pub extern "C" fn init() -> ! {
    exception::init();
    mmu::init();
    uart::puts(b"mmu enabled\r\n");

    #[cfg(translation_fault_test)]
    {
        uart::puts(b"translation fault test enabled\r\n");
        uart::puts(b"read 0x8000_0000 -> expect translation fault\r\n");

        unsafe {
            core::ptr::read_volatile(0x8000_0000 as *const u64);
        }

        uart::puts(b"returned from translation-fault handler\r\n");
    }

    #[cfg(dram_oob_test)]
    {
        uart::puts(b"dram out-of-range test enabled\r\n");
        uart::puts(b"read 0xa000_0000 -> verify actual fault type\r\n");

        unsafe {
            core::ptr::read_volatile(0xa000_0000 as *const u64);
        }

        uart::puts(b"returned from out-of-range handler\r\n");
    }

    #[cfg(not(any(translation_fault_test, dram_oob_test)))]
    uart::puts(b"exception test disabled\r\n");

    loop {
        core::hint::spin_loop();
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo<'_>) -> ! {
    loop {
        core::hint::spin_loop();
    }
}