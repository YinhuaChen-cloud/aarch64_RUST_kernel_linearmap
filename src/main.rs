use crate::uart;

// 这是 Rust 里的一个属性，用来禁止编译器 “修改” 函数名字
#[no_mangle]
pub extern "C" fn main() -> ! {
    uart::puts(b"hello from main\r\n");
    uart::puts(b"before clear ttbr0\r\n");

    uart::puts(b"after clear ttbr0\r\n");

    loop {
        core::hint::spin_loop();
    }
}
