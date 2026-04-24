#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod start;
mod uart;

#[no_mangle]
pub extern "C" fn rust_main() -> ! {
    uart::puts(b"hello world\r\n");

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
