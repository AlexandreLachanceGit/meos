ENTRY(boot)

SECTIONS {
    . = 0x80200000; /* TODO: *NOT PLATFORM AGNOSTIC* Is there a way to make it dynamic? */

    .text :{
        KEEP(*(.text.boot));
        *(.text .text.*);
    }

    .rodata : ALIGN(8) {
        *(.rodata .rodata.*);
    }

    .data : ALIGN(8) {
        *(.data .data.*);
    }

    .bss : ALIGN(8) {
        __bss = .;
        *(.bss .bss.* .sbss .sbss.*);
        __bss_end = .;
    }

    . = ALIGN(16);
    . += 128 * 1024; /* 128KB stack */
    _STACK_PTR = .;

    . = ALIGN(4096);
    _KERNEL_END = .;
}
