#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod exception;
mod mmu;
mod start;
mod uart;

#[no_mangle]
pub extern "C" fn rust_main() -> ! {
    exception::init();
    mmu::init();
    uart::puts(b"mmu enabled\r\n");
    uart::puts(b"read 0x8000_0000 -> expect translation fault\r\n");

    unsafe {
        core::ptr::read_volatile(0x8000_0000 as *const u64);
    }

    uart::puts(b"unexpected: access returned\r\n");

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
