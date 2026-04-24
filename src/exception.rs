use core::arch::{asm, global_asm};

use crate::uart;

const EXCEPTION_FRAME_SIZE: usize = core::mem::size_of::<ExceptionFrame>();

global_asm!(
    r#"
    .section .text.exceptions, "ax"
    .align 11
    .global __exception_vectors
__exception_vectors:
    b __vector_current_el_sp0_sync
    .balign 128
    b __vector_current_el_sp0_irq
    .balign 128
    b __vector_current_el_sp0_fiq
    .balign 128
    b __vector_current_el_sp0_serror
    .balign 128
    b __vector_current_el_spx_sync
    .balign 128
    b __vector_current_el_spx_irq
    .balign 128
    b __vector_current_el_spx_fiq
    .balign 128
    b __vector_current_el_spx_serror
    .balign 128
    b __vector_lower_el_a64_sync
    .balign 128
    b __vector_lower_el_a64_irq
    .balign 128
    b __vector_lower_el_a64_fiq
    .balign 128
    b __vector_lower_el_a64_serror
    .balign 128
    b __vector_lower_el_a32_sync
    .balign 128
    b __vector_lower_el_a32_irq
    .balign 128
    b __vector_lower_el_a32_fiq
    .balign 128
    b __vector_lower_el_a32_serror

    .macro save_and_return vector_id
    sub sp, sp, #288
    stp x0, x1, [sp, #0]
    stp x2, x3, [sp, #16]
    stp x4, x5, [sp, #32]
    stp x6, x7, [sp, #48]
    stp x8, x9, [sp, #64]
    stp x10, x11, [sp, #80]
    stp x12, x13, [sp, #96]
    stp x14, x15, [sp, #112]
    stp x16, x17, [sp, #128]
    stp x18, x19, [sp, #144]
    stp x20, x21, [sp, #160]
    stp x22, x23, [sp, #176]
    stp x24, x25, [sp, #192]
    stp x26, x27, [sp, #208]
    stp x28, x29, [sp, #224]
    str x30, [sp, #240]

    mov x9, #\vector_id
    str x9, [sp, #248]
    mrs x9, esr_el1
    str x9, [sp, #256]
    mrs x9, far_el1
    str x9, [sp, #264]
    mrs x9, elr_el1
    str x9, [sp, #272]
    mrs x9, spsr_el1
    str x9, [sp, #280]

    mov x0, sp
    bl rust_exception_handler

    ldr x9, [sp, #272]
    msr elr_el1, x9
    ldr x9, [sp, #280]
    msr spsr_el1, x9

    ldp x0, x1, [sp, #0]
    ldp x2, x3, [sp, #16]
    ldp x4, x5, [sp, #32]
    ldp x6, x7, [sp, #48]
    ldp x8, x9, [sp, #64]
    ldp x10, x11, [sp, #80]
    ldp x12, x13, [sp, #96]
    ldp x14, x15, [sp, #112]
    ldp x16, x17, [sp, #128]
    ldp x18, x19, [sp, #144]
    ldp x20, x21, [sp, #160]
    ldp x22, x23, [sp, #176]
    ldp x24, x25, [sp, #192]
    ldp x26, x27, [sp, #208]
    ldp x28, x29, [sp, #224]
    ldr x30, [sp, #240]
    add sp, sp, #288
    eret
    .endm

__vector_current_el_sp0_sync:
    save_and_return 0
__vector_current_el_sp0_irq:
    save_and_return 1
__vector_current_el_sp0_fiq:
    save_and_return 2
__vector_current_el_sp0_serror:
    save_and_return 3
__vector_current_el_spx_sync:
    save_and_return 4
__vector_current_el_spx_irq:
    save_and_return 5
__vector_current_el_spx_fiq:
    save_and_return 6
__vector_current_el_spx_serror:
    save_and_return 7
__vector_lower_el_a64_sync:
    save_and_return 8
__vector_lower_el_a64_irq:
    save_and_return 9
__vector_lower_el_a64_fiq:
    save_and_return 10
__vector_lower_el_a64_serror:
    save_and_return 11
__vector_lower_el_a32_sync:
    save_and_return 12
__vector_lower_el_a32_irq:
    save_and_return 13
__vector_lower_el_a32_fiq:
    save_and_return 14
__vector_lower_el_a32_serror:
    save_and_return 15
"#
);

unsafe extern "C" {
    static __exception_vectors: u8;
}

pub fn init() {
    unsafe {
        let vbar = core::ptr::addr_of!(__exception_vectors) as u64;
        asm!(
            "msr vbar_el1, {vbar}",
            "isb",
            vbar = in(reg) vbar,
            options(nostack, preserves_flags)
        );
    }
}

#[repr(C)]
pub struct ExceptionFrame {
    pub regs: [u64; 31],
    pub vector: u64,
    pub esr: u64,
    pub far: u64,
    pub elr: u64,
    pub spsr: u64,
}

#[no_mangle]
pub extern "C" fn rust_exception_handler(frame: &mut ExceptionFrame) {
    uart::puts(b"\r\nexception: ");
    put_vector_name(frame.vector);
    uart::puts(b"\r\n");

    uart::puts(b"esr_el1=");
    uart::put_hex_u64(frame.esr);
    uart::puts(b"\r\nfar_el1=");
    uart::put_hex_u64(frame.far);
    uart::puts(b"\r\nelr_el1=");
    uart::put_hex_u64(frame.elr);
    uart::puts(b"\r\nspsr_el1=");
    uart::put_hex_u64(frame.spsr);
    uart::puts(b"\r\n");

    if is_translation_fault(frame.esr) {
        uart::puts(b"decoded: translation fault\r\n");
    }

    #[cfg(exception_test)]
    if should_resume_after_test(frame) {
        uart::puts(b"test enabled: resume at next instruction\r\n");
        frame.elr = frame.elr.wrapping_add(4);
        return;
    }

    loop {
        core::hint::spin_loop();
    }
}

#[cfg(exception_test)]
fn should_resume_after_test(frame: &ExceptionFrame) -> bool {
    is_synchronous_vector(frame.vector) && is_translation_fault(frame.esr)
}

#[cfg(exception_test)]
fn is_synchronous_vector(vector: u64) -> bool {
    matches!(vector, 0 | 4 | 8 | 12)
}

fn put_vector_name(vector: u64) {
    match vector {
        0 => uart::puts(b"current_el_sp0_sync"),
        1 => uart::puts(b"current_el_sp0_irq"),
        2 => uart::puts(b"current_el_sp0_fiq"),
        3 => uart::puts(b"current_el_sp0_serror"),
        4 => uart::puts(b"current_el_spx_sync"),
        5 => uart::puts(b"current_el_spx_irq"),
        6 => uart::puts(b"current_el_spx_fiq"),
        7 => uart::puts(b"current_el_spx_serror"),
        8 => uart::puts(b"lower_el_a64_sync"),
        9 => uart::puts(b"lower_el_a64_irq"),
        10 => uart::puts(b"lower_el_a64_fiq"),
        11 => uart::puts(b"lower_el_a64_serror"),
        12 => uart::puts(b"lower_el_a32_sync"),
        13 => uart::puts(b"lower_el_a32_irq"),
        14 => uart::puts(b"lower_el_a32_fiq"),
        15 => uart::puts(b"lower_el_a32_serror"),
        _ => uart::puts(b"unknown"),
    }
}

fn is_translation_fault(esr: u64) -> bool {
    let ec = (esr >> 26) & 0x3f;
    let dfsc = esr & 0x3f;

    matches!(ec, 0x24 | 0x25) && matches!(dfsc, 0b000100..=0b000111)
}

const _: () = assert!(EXCEPTION_FRAME_SIZE == 288);