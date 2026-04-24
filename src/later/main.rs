#![no_std]

pub mod head;
pub mod uart;

pub use head::jump_to_main;

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