OUTPUT_FORMAT(elf64-littleaarch64)
OUTPUT_ARCH(aarch64)
ENTRY(_start)

SECTIONS
{
  . = 0x40080000;

  .text : ALIGN(4096) {
    KEEP(*(.text._start))
    *(.text .text.*)
  }

  .rodata : ALIGN(4096) {
    *(.rodata .rodata.*)
  }

  .data : ALIGN(4096) {
    *(.data .data.*)
  }

  .bss (NOLOAD) : ALIGN(4096) {
    *(.bss .bss.*)
    *(COMMON)
  }

  .stack (NOLOAD) : ALIGN(16) {
    __boot_stack_bottom = .;
    . += 0x4000;
    __boot_stack_top = .;
  }

  /DISCARD/ : {
    *(.comment)
    *(.eh_frame*)
    *(.note*)
    *(.gnu*)
  }
}
