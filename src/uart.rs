const PL011_BASE: usize = 0x0900_0000;
const UART_DR: *mut u32 = PL011_BASE as *mut u32;
const UART_FR: *const u32 = (PL011_BASE + 0x18) as *const u32;
const UART_FR_TXFF: u32 = 1 << 5;

pub fn puts(s: &[u8]) {
    for &byte in s {
        write_byte(byte);
    }
}

pub fn write_byte(byte: u8) {
    unsafe {
        while core::ptr::read_volatile(UART_FR) & UART_FR_TXFF != 0 {}
        core::ptr::write_volatile(UART_DR, byte as u32);
    }
}

pub fn put_hex_u64(value: u64) {
    puts(b"0x");
    for shift in (0..16).rev() {
        let nibble = ((value >> (shift * 4)) & 0xf) as u8;
        let digit = match nibble {
            0..=9 => b'0' + nibble,
            _ => b'a' + (nibble - 10),
        };
        write_byte(digit);
    }
}
