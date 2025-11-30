.section ".text.boot"
.global _start

_start:
    ldr x0, =__stack_top
    mov sp, x0

    ldr         x1,     =__bss_start
    ldr         x2,     =__bss_end

    .clear_bss:
        cmp                 x1, x2
        b.ge        .set_el1_when_bss_empty
        str         xzr,    [x1]
        add         x1,     x1, #8
        b           .clear_bss

    .set_el1_when_bss_empty:
        ldr         x0,         =__stack_top
        msr         sp_el1,     x0

        mov         x0,         #(1 << 31)
        msr         hcr_el2,    x0

        mov         x0,         #0x3c5
        msr         spsr_el2,   x0

        adr         x0,         el1_entry
        msr         elr_el2,    x0

        eret

    el1_entry:
        mov         x0,         #(0x3 << 20)
        msr         cpacr_el1,  x0
        isb
        ldr         x0,         =vector_table
        msr         vbar_el1,   x0
        b           kernel_main
