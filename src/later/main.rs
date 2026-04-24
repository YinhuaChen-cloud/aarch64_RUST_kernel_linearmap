#![no_std]

pub mod uart;

const LINEAR_MAP_BASE: usize = 0xffff_ffff_0000_0000;

#[no_mangle]
pub extern "C" fn main() -> ! {
    uart::puts(b"hello from main\r\n");
    uart::puts(b"before clear ttbr0\r\n");

    uart::puts(b"after clear ttbr0\r\n");

    loop {
        core::hint::spin_loop();
    }
}