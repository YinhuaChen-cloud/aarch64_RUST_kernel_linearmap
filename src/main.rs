use crate::uart;

#[no_mangle]
pub extern "C" fn main() -> ! {
    uart::puts(b"hello from main\r\n");

    loop {
        core::hint::spin_loop();
    }
}
