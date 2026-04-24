OUTPUT_FORMAT(elf64-littleaarch64)
OUTPUT_ARCH(aarch64)
ENTRY(_start)

/* link.x 中不接受下划线 '_' 分隔数字 */
LINEAR_MAP_OFFSET = 0xffffffff00000000;

SECTIONS
{
  . = 0x40080000;

  .text : ALIGN(4096) {
    KEEP(*(.text._start))
    *(EXCLUDE_FILE (*liblater.rlib:*) .text .text.*)
  }

  .rodata : ALIGN(4096) {
    *(EXCLUDE_FILE (*liblater.rlib:*) .rodata .rodata.*)
  }

  .data : ALIGN(4096) {
    *(EXCLUDE_FILE (*liblater.rlib:*) .data .data.*)
  }

  .bss (NOLOAD) : ALIGN(4096) {
    *(EXCLUDE_FILE (*liblater.rlib:*) .bss .bss.*)
    *(COMMON)
  }

  .stack (NOLOAD) : ALIGN(16) {
    __boot_stack_bottom = .;
    . += 0x4000;
    __boot_stack_top = .;
    __boot_stack_top_linear = __boot_stack_top + LINEAR_MAP_OFFSET;
  }

  . = ALIGN(4096);
  . += LINEAR_MAP_OFFSET;

  .later.text : AT(ADDR(.later.text) - LINEAR_MAP_OFFSET) ALIGN(4096) {
    *liblater.rlib:(.text .text.*)
  }

  .later.rodata : AT(ADDR(.later.rodata) - LINEAR_MAP_OFFSET) ALIGN(4096) {
    *liblater.rlib:(.rodata .rodata.*)
  }

  .later.data : AT(ADDR(.later.data) - LINEAR_MAP_OFFSET) ALIGN(4096) {
    *liblater.rlib:(.data .data.*)
  }

  .later.bss (NOLOAD) : AT(ADDR(.later.bss) - LINEAR_MAP_OFFSET) ALIGN(4096) {
    *liblater.rlib:(.bss .bss.*)
  }

  /DISCARD/ : {
    *(.comment)
    *(.eh_frame*)
    *(.note*)
    *(.gnu*)
  }
}