use core::arch::asm;

const ENTRY_COUNT: usize = 512;
const PAGE_SHIFT_2M: usize = 21;
const PAGE_SHIFT_1G: usize = 30;

const DESC_VALID: u64 = 1 << 0;
const DESC_TABLE: u64 = 1 << 1;
const DESC_BLOCK: u64 = 0 << 1;

const ATTR_IDX_SHIFT: u64 = 2;
const ATTR_IDX_DEVICE: u64 = 0 << ATTR_IDX_SHIFT;
const ATTR_IDX_NORMAL: u64 = 1 << ATTR_IDX_SHIFT;
const ACCESS_FLAG: u64 = 1 << 10;
const INNER_SHAREABLE: u64 = 0b11 << 8;
const PXN: u64 = 1 << 53;
const UXN: u64 = 1 << 54;

const MAIR_DEVICE_NGNRNE: u64 = 0x00;
const MAIR_NORMAL_WB: u64 = 0xff;
const MAIR_VALUE: u64 = MAIR_DEVICE_NGNRNE | (MAIR_NORMAL_WB << 8);

const TCR_T0SZ_4GB: u64 = 32;
const TCR_IRGN0_WBWA: u64 = 0b01 << 8;
const TCR_ORGN0_WBWA: u64 = 0b01 << 10;
const TCR_SH0_INNER: u64 = 0b11 << 12;
const TCR_TG0_4K: u64 = 0b00 << 14;
const TCR_EPD1_DISABLE: u64 = 1 << 23;
const TCR_IPS_40BIT: u64 = 0b010 << 32;
const TCR_VALUE: u64 = TCR_T0SZ_4GB
    | TCR_IRGN0_WBWA
    | TCR_ORGN0_WBWA
    | TCR_SH0_INNER
    | TCR_TG0_4K
    | TCR_EPD1_DISABLE
    | TCR_IPS_40BIT;

#[repr(C, align(4096))]
struct PageTable([u64; ENTRY_COUNT]);

static mut L1_TABLE: PageTable = PageTable([0; ENTRY_COUNT]);
static mut LOW_1GB_L2_TABLE: PageTable = PageTable([0; ENTRY_COUNT]);

#[inline(always)]
const fn table_desc(addr: usize) -> u64 {
    (addr as u64) | DESC_VALID | DESC_TABLE
}

#[inline(always)]
const fn block_desc(addr: usize, attrs: u64) -> u64 {
    (addr as u64) | DESC_VALID | DESC_BLOCK | attrs
}

#[inline(always)]
const fn device_block_attrs() -> u64 {
    ATTR_IDX_DEVICE | ACCESS_FLAG | PXN | UXN
}

#[inline(always)]
const fn normal_block_attrs() -> u64 {
    ATTR_IDX_NORMAL | ACCESS_FLAG | INNER_SHAREABLE
}

unsafe fn build_identity_map() {
    L1_TABLE.0[0] = table_desc(core::ptr::addr_of!(LOW_1GB_L2_TABLE) as usize);

    for index in 0..ENTRY_COUNT {
        let phys = index << PAGE_SHIFT_2M;
        LOW_1GB_L2_TABLE.0[index] = block_desc(phys, device_block_attrs());
    }

    // 左闭右开区间 等价于 >= 1 && < 2
    for index in 1..2 {
        let phys = index << PAGE_SHIFT_1G;
        L1_TABLE.0[index] = block_desc(phys, normal_block_attrs());
    }
}

pub fn init() {
    unsafe {
        build_identity_map();

        let ttbr0 = core::ptr::addr_of!(L1_TABLE) as u64;
        let mut sctlr: u64;

        asm!(
            "dsb ishst",
            "msr mair_el1, {mair}",
            "msr tcr_el1, {tcr}",
            "msr ttbr0_el1, {ttbr0}",
            "isb",
            "tlbi vmalle1",
            "dsb ish",
            "isb",
            "mrs {sctlr}, sctlr_el1",
            mair = in(reg) MAIR_VALUE,
            tcr = in(reg) TCR_VALUE,
            ttbr0 = in(reg) ttbr0,
            sctlr = out(reg) sctlr,
            options(nostack)
        );

        sctlr |= 1 << 0;

        asm!(
            "msr sctlr_el1, {sctlr}",
            "isb",
            sctlr = in(reg) sctlr,
            options(nostack)
        );
    }
}