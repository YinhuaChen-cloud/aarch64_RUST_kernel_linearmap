#![no_std]
#![no_main]

use core::arch::global_asm;
use core::panic::PanicInfo;

global_asm!(
    r#"
    .section .text._start, "ax"
    .global _start
_start:
    ldr x0, =__boot_stack_top
    mov sp, x0
    bl rust_main
1:
    wfe
    b 1b
"#
);

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
