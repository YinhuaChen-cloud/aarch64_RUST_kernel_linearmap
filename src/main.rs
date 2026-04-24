#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod start;

const PL011_BASE: usize = 0x0900_0000;
const UART_DR: *mut u32 = PL011_BASE as *mut u32;
const UART_FR: *const u32 = (PL011_BASE + 0x18) as *const u32;
const UART_FR_TXFF: u32 = 1 << 5;

#[no_mangle]
pub extern "C" fn rust_main() -> ! {
    for &byte in b"hello world\r\n" {
        uart_write(byte);
    }

    loop {
        core::hint::spin_loop();
    }
}

fn uart_write(byte: u8) {
    unsafe {
        while core::ptr::read_volatile(UART_FR) & UART_FR_TXFF != 0 {}
        core::ptr::write_volatile(UART_DR, byte as u32);
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo<'_>) -> ! {
    loop {
        core::hint::spin_loop();
    }
}
