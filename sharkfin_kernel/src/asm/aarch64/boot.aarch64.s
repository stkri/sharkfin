.section ".text.boot"
.global _start

_start:
    ldr x0, =__stack_top
    mov sp, x0

    ldr         x1,     =__bss_start
    ldr         x2,     =__bss_end

    .clear_bss:
        cmp                 x1, x2
        b.ge        .call_main_when_bss_empty
        str         xzr,    [x1]
        add         x1,     x1, #8
        b           .clear_bss

    .call_main_when_bss_empty:
        b           kernel_main