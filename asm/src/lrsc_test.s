.global _start

.text
_start:
    # Load the base addr of the trace array
    li     x31, 0x00001000
    # Addr to test lr/sc
    li     x30, 0x00000ffc  # Main addr
    li     x29, 0x00000ff8  # Other addr
    # Load 2 random 32-bit numbers
    li     x1, 0xbcfec832
    li     x2, 0x51290ce3
    # --------------- LR -> SC ----------------
    # Same addr
    sw     x1, (x30)
    lr.w   x3, (x30)
    sw     x3, 0(x31)       # Should be 0xbcfec832
    sc.w   x4, x2, (x30)    # Should succeed
    sw     x4, 4(x31)       # Should be 0x00000000
    # Other addr
    lr.w   x3, (x30)
    sw     x3, 8(x31)       # Should be 0x51290ce3
    sc.w   x4, x2, (x29)    # Should fail
    sw     x4, 12(x31)      # Should be 0x00000001
    # ------------ LR -> SC -> SC -------------
    # Same addr
    lr.w   x3, (x30)
    sw     x3, 16(x31)      # Should be 0x51290ce3
    sc.w   x4, x1, (x30)    # Should succeed
    sc.w   x5, x2, (x30)    # Should fail
    sw     x4, 20(x31)      # Should be 0x00000000
    sw     x5, 24(x31)      # Should be 0x00000001
    # Other addr
    lr.w   x3, (x30)
    sw     x3, 28(x31)      # Should be 0xbcfec832
    sc.w   x4, x2, (x29)    # Should fail
    sc.w   x5, x2, (x30)    # Should fail
    sw     x4, 32(x31)      # Should be 0x00000001
    sw     x5, 36(x31)      # Should be 0x00000001
    # ------------ LR -> SW -> SC -------------
    # Same addr
    lr.w   x3, (x30)
    sw     x3, 40(x31)      # Should be 0xbcfec832
    sw     x2, (x30)        # Should invalidate
    sc.w   x4, x1, (x30)    # Should fail
    sw     x4, 44(x31)      # Should be 0x00000001
    # Other addr
    lr.w   x3, (x30)
    sw     x3, 48(x31)      # Should be 0x51290ce3
    sw     x1, (x29)        # Should not invalidate
    sc.w   x4, x1, (x30)    # Should succeed
    sw     x4, 52(x31)      # Should be 0x00000000
    # ------------ LR -> SH -> SC -------------
    # Same addr + 2
    addi   x28, x30, 2
    lr.w   x3, (x30)
    sw     x3, 56(x31)      # Should be 0xbcfec832
    sh     x2, (x28)        # Should invalidate
    sc.w   x4, x1, (x30)    # Should fail
    sw     x4, 60(x31)      # Should be 0x00000001
    # Other addr + 2
    addi   x27, x29, 2
    lr.w   x3, (x30)
    sw     x3, 64(x31)      # Should be 0x0ce3c832
    sh     x1, (x27)        # Should not invalidate
    sc.w   x4, x1, (x30)    # Should succeed
    sw     x4, 68(x31)      # Should be 0x00000000
    # ------------ LR -> SB -> SC -------------
    # Same addr + 3
    addi   x28, x30, 3
    lr.w   x3, (x30)
    sw     x3, 72(x31)      # Should be 0xbcfec832
    sb     x2, (x28)        # Should invalidate
    sc.w   x4, x1, (x30)    # Should fail
    sw     x4, 76(x31)      # Should be 0x00000001
    # Other addr + 3
    addi   x27, x29, 3
    lr.w   x3, (x30)
    sw     x3, 80(x31)      # Should be 0xe3fec832
    sb     x1, (x27)        # Should not invalidate
    sc.w   x4, x1, (x30)    # Should succeed
    sw     x4, 84(x31)      # Should be 0x00000000
    # ------------ LR -> LR -> SC -------------
    # Other addr
    lr.w   x3, (x30)
    sw     x3, 88(x31)      # Should be 0xbcfec832
    lr.w   x3, (x29)
    sc.w   x4, x2, (x30)    # Should fail
    sw     x4, 92(x31)      # Should be 0x00000001
    lw     x3, (x30)
    sw     x3, 96(x31)      # Should be 0xbcfec832
    # ---------------- ECALL ------------------
    ecall

