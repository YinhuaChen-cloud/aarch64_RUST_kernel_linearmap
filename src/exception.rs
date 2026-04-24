use core::arch::{asm, global_asm};

use crate::uart;

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

__vector_current_el_sp0_sync:
    mov x0, #0
    b rust_exception_handler
__vector_current_el_sp0_irq:
    mov x0, #1
    b rust_exception_handler
__vector_current_el_sp0_fiq:
    mov x0, #2
    b rust_exception_handler
__vector_current_el_sp0_serror:
    mov x0, #3
    b rust_exception_handler
__vector_current_el_spx_sync:
    mov x0, #4
    b rust_exception_handler
__vector_current_el_spx_irq:
    mov x0, #5
    b rust_exception_handler
__vector_current_el_spx_fiq:
    mov x0, #6
    b rust_exception_handler
__vector_current_el_spx_serror:
    mov x0, #7
    b rust_exception_handler
__vector_lower_el_a64_sync:
    mov x0, #8
    b rust_exception_handler
__vector_lower_el_a64_irq:
    mov x0, #9
    b rust_exception_handler
__vector_lower_el_a64_fiq:
    mov x0, #10
    b rust_exception_handler
__vector_lower_el_a64_serror:
    mov x0, #11
    b rust_exception_handler
__vector_lower_el_a32_sync:
    mov x0, #12
    b rust_exception_handler
__vector_lower_el_a32_irq:
    mov x0, #13
    b rust_exception_handler
__vector_lower_el_a32_fiq:
    mov x0, #14
    b rust_exception_handler
__vector_lower_el_a32_serror:
    mov x0, #15
    b rust_exception_handler
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

#[no_mangle]
pub extern "C" fn rust_exception_handler(vector: u64) -> ! {
    let esr: u64;
    let far: u64;
    let elr: u64;
    let spsr: u64;

    unsafe {
        asm!(
            "mrs {esr}, esr_el1",
            "mrs {far}, far_el1",
            "mrs {elr}, elr_el1",
            "mrs {spsr}, spsr_el1",
            esr = out(reg) esr,
            far = out(reg) far,
            elr = out(reg) elr,
            spsr = out(reg) spsr,
            options(nostack, nomem, preserves_flags)
        );
    }

    uart::puts(b"\r\nexception: ");
    put_vector_name(vector);
    uart::puts(b"\r\n");

    uart::puts(b"esr_el1=");
    uart::put_hex_u64(esr);
    uart::puts(b"\r\nfar_el1=");
    uart::put_hex_u64(far);
    uart::puts(b"\r\nelr_el1=");
    uart::put_hex_u64(elr);
    uart::puts(b"\r\nspsr_el1=");
    uart::put_hex_u64(spsr);
    uart::puts(b"\r\n");

    if is_translation_fault(esr) {
        uart::puts(b"decoded: translation fault\r\n");
    }

    loop {
        core::hint::spin_loop();
    }
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