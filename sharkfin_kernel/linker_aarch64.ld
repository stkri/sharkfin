ENTRY (_start)

SECTIONS {
    . = 0x80000;

    .text : {
        *(.text*)
    }

    .rodata : {
        *(.rodata*)
    }

    .data : {
        *(.data*)
    }

    .bss : {
        __bss_start = .;
        *(.bss*)
        *(COMMON)
        __bss_end = .;
    }

    . = ALIGN(16);
    . = . + 0x4000;
    __stack_top = .;

    . = ALIGN(4096);
    __page_tables_start = .;
    . = . + (515 * 4096);
    __page_tables_end = .;

    __heap_start = .;
}
