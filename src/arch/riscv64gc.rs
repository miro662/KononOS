use core::arch::global_asm;

/// A type identifing core, passed as only argument to kernel_main
pub type Hart = u64;

global_asm!{
    // put this in start section
    ".section .text.start",
    ".globl _start",

    // start point
    "_start:",

    // load stack address
    "la sp, _stack_start",

    // clear BSS section
    "1:",
    "la t1, _bss_start",
    "la t2, _bss_end",
    
    // a0 is expected to contain hartid
    "sd zero, (t1)",
    "addi t1, t1, 8",
    "bltu t1, t2, 1b",

    // launch kernel
    "tail kernel_main",
    ".previous",
}