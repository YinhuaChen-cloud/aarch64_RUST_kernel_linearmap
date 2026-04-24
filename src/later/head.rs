use core::arch::asm;

use crate::{main, LINEAR_MAP_BASE};

unsafe extern "C" {
    static __boot_stack_top: u8;
}

pub extern "C" fn jump_to_main() -> ! {
    let stack_top = core::ptr::addr_of!(__boot_stack_top) as usize + LINEAR_MAP_BASE;

    unsafe {
        asm!(
            "mov sp, {stack_top}",
            "b {main_entry}",
            stack_top = in(reg) stack_top,
            main_entry = sym main,
            options(noreturn)
        );
    }
}