OUTPUT_FORMAT(elf64-littleaarch64)
OUTPUT_ARCH(aarch64)
ENTRY(_start)

KERNEL_PHYS_BASE = 0x40080000;
KERNEL_VIRT_OFFSET = 0xffffffff00000000;

SECTIONS
{
  . = KERNEL_PHYS_BASE;

  .boot.text : ALIGN(4096) {
    KEEP(*(.boot.text .boot.text.*))
  }

  .boot.rodata : ALIGN(4096) {
    KEEP(*(.boot.rodata .boot.rodata.*))
  }

  .boot.data : ALIGN(4096) {
    KEEP(*(.boot.data .boot.data.*))
  }

  .boot.bss (NOLOAD) : ALIGN(4096) {
    KEEP(*(.boot.bss .boot.bss.*))
  }

  .boot.stack (NOLOAD) : ALIGN(16) {
    __boot_stack_bottom = .;
    . += 0x4000;
    __boot_stack_top = .;
  }

  __boot_end = ALIGN(4096);
  . = __boot_end + KERNEL_VIRT_OFFSET;

  .text : AT(ADDR(.text) - KERNEL_VIRT_OFFSET) ALIGN(4096) {
    *(.text .text.*)
  }

  .rodata : AT(ADDR(.rodata) - KERNEL_VIRT_OFFSET) ALIGN(4096) {
    *(.rodata .rodata.*)
  }

  .data : AT(ADDR(.data) - KERNEL_VIRT_OFFSET) ALIGN(4096) {
    *(.data .data.*)
  }

  .bss (NOLOAD) : ALIGN(4096) {
    *(.bss .bss.*)
    *(COMMON)
  }

  /DISCARD/ : {
    *(.comment)
    *(.eh_frame*)
    *(.note*)
    *(.gnu*)
  }
}