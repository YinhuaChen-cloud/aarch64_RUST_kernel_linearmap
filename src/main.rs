#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod mmu;
mod start;
mod uart;

#[no_mangle]
pub extern "C" fn rust_main() -> ! {
    mmu::init();
    uart::puts(b"hello world\r\n");

    loop {
        core::hint::spin_loop();
    }
}

// panic_handler: 意思是 panic 处理入口，当程序发生 panic! 时会调用这个函数
// 当这些情况发生时，会进来：
//     panic!("xxx")
//     unwrap() 失败
//     assert!() 失败
//     数组越界等某些检查失败
// -> ! : 这个函数 不会返回
#[panic_handler]
fn panic(_info: &PanicInfo<'_>) -> ! {
    loop {
        core::hint::spin_loop();
    }
}
