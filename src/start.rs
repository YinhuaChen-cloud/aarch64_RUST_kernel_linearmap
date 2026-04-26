use core::arch::global_asm;

global_asm!(
    r#"
    .section .boot.text, "ax"
    .global _start
_start:
    ldr x0, =__boot_stack_top
    mov sp, x0
    bl init
1:
    wfe
    b 1b
"#
);
