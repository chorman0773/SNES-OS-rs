
.section .text.init
_Reset:
    CLC
    XCE
    LDA $__stack_tail
    TAS
    JMP __reset_bios_startup