pub const EARLY_PL011_BASE: usize = 0x0900_0000;

const UART_DR_OFFSET: usize = 0x00;
const UART_FR_OFFSET: usize = 0x18;
const UART_FR_TXFF: u32 = 1 << 5;

#[inline(always)]
const fn uart_dr(base: usize) -> *mut u32 {
    (base + UART_DR_OFFSET) as *mut u32
}

#[inline(always)]
const fn uart_fr(base: usize) -> *const u32 {
    (base + UART_FR_OFFSET) as *const u32
}

#[inline(always)]
fn write_byte_at(base: usize, byte: u8) {
    unsafe {
        while core::ptr::read_volatile(uart_fr(base)) & UART_FR_TXFF != 0 {}
        core::ptr::write_volatile(uart_dr(base), byte as u32);
    }
}

pub fn early_puts(s: &[u8]) {
    for &byte in s {
        early_write_byte(byte);
    }
}

pub fn early_write_byte(byte: u8) {
    write_byte_at(EARLY_PL011_BASE, byte)
}

pub fn early_put_hex_u64(value: u64) {
    early_puts(b"0x");
    for shift in (0..16).rev() {
        let nibble = ((value >> (shift * 4)) & 0xf) as u8;
        let digit = match nibble {
            0..=9 => b'0' + nibble,
            _ => b'a' + (nibble - 10),
        };
        early_write_byte(digit);
    }
}