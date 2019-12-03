.text

set_mprv:
    # NOTE: CLOBBERS a3
    li a3, 1
    slli a3, a3, 17
    csrs mstatus, a3
    jr ra

unset_mprv:
    # NOTE: CLOBBERS a3
    li a3, 1
    slli a3, a3, 17
    csrc mstatus, a3
    jr ra

.global copy8_from_sm
copy8_from_sm:
    # a0: dst
    # a1: src
    addi sp, sp, -8
    sd ra, 0(sp)
    ld t0, 0x00(a1)
    jal set_mprv
    sd t0, 0x00(a0)
    jal unset_mprv
    ld ra, 0(sp)
    addi sp, sp, 8
    jr ra
