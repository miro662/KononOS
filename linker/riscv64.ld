SECTIONS {
    . = 0x80000000;
    .text.start : { *(.text.start) }
    .text : { *(.text) }
    .data : { *(.data) }
    .rodata : { *(.rodata) }
    .bss : { 
        _bss_start = .;
        *(.bss) 
        . = ALIGN(4096);
        _stack_end = .;
        . += 0x10000;
        _bss_end = .;
        _stack_start = .;
    }
}