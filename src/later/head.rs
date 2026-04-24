use core::arch::asm;

use crate::main;

unsafe extern "C" {
    static __boot_stack_top_linear: u8;
}

#[inline(never)]
#[no_mangle]
pub extern "C" fn jump_to_main() -> ! {
    unsafe {
        asm!(
            "adrp x9, {stack_top_sym}",
            "add x9, x9, :lo12:{stack_top_sym}",
            "mov sp, x9",
            "msr ttbr0_el1, xzr",
            "isb",
            "tlbi vmalle1",
            "dsb ish",
            "isb",
            "b {main_entry}",
            stack_top_sym = sym __boot_stack_top_linear,
            main_entry = sym main,
            options(noreturn)
        );
    }
}