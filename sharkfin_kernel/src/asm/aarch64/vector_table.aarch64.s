.section ".text.vector_table"
.global vector_table
.balign 0x800

vector_table:
    b sync_handler
    .balign 0x80
    b irq_handler
    .balign 0x80
    b fiq_handler
    .balign 0x80
    b serror_handler
    .balign 0x80
    b sync_handler
    .balign 0x80
    b irq_handler
    .balign 0x80
    b fiq_handler
    .balign 0x80
    b serror_handler
    .balign 0x80
    b sync_handler
    .balign 0x80
    b irq_handler
    .balign 0x80
    b fiq_handler
    .balign 0x80
    b serror_handler
