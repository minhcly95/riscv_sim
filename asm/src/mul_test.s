.global _start

.text
_start:
    # Load the base addr of the trace array
    li     x31, 0x00001000
    # Load 2 random 32-bit numbers
    li     x1, 0xbcfec832
    li     x2, 0x51290ce3
    # ----------------- MUL -------------------
    # MUL x1 * x2
    mul    x3, x1, x2       # Should be 0x694fdc56
    sw     x3, 0(x31)
    mulh   x3, x1, x2       # Should be 0xeac1dec6
    sw     x3, 4(x31)
    mulhu  x3, x1, x2       # Should be 0x3beaeba9
    sw     x3, 8(x31)
    mulhsu x3, x1, x2       # Should be 0xeac1dec6
    sw     x3, 12(x31)
    mulhsu x3, x2, x1       # Should be 0x3beaeba9
    sw     x3, 16(x31)
    # MUL x1 * x1
    mul    x3, x1, x1       # Should be 0x4fc629c4
    sw     x3, 20(x31)
    mulh   x3, x1, x1       # Should be 0x1189a337
    sw     x3, 24(x31)
    mulhu  x3, x1, x1       # Should be 0x8b87339b
    sw     x3, 28(x31)
    mulhsu x3, x1, x1       # Should be 0xce886b69
    sw     x3, 32(x31)
    # MUL x2 * x2
    mul    x3, x2, x2       # Should be 0xc75c1149
    sw     x3, 36(x31)
    mulh   x3, x2, x2       # Should be 0x19bb00bc
    sw     x3, 40(x31)
    mulhu  x3, x2, x2       # Should be 0x19bb00bc
    sw     x3, 44(x31)
    mulhsu x3, x2, x2       # Should be 0x19bb00bc
    sw     x3, 48(x31)
    # --------------- DIV REM -----------------
    li     x2, 0xff290ce3
    # DIV
    div    x3, x1, x2       # Should be 0x0000004f
    sw     x3, 52(x31)
    div    x3, x2, x1       # Should be 0x00000000
    sw     x3, 56(x31)
    # DIVU
    divu   x3, x1, x2       # Should be 0x00000000
    sw     x3, 60(x31)
    divu   x3, x2, x1       # Should be 0x00000001
    sw     x3, 64(x31)
    # REM
    rem    x3, x1, x2       # Should be 0xff53ce25
    sw     x3, 68(x31)
    rem    x3, x2, x1       # Should be 0xff290ce3
    sw     x3, 72(x31)
    # REMU
    remu   x3, x1, x2       # Should be 0xbcfec832
    sw     x3, 76(x31)
    remu   x3, x2, x1       # Should be 0x422a44b1
    sw     x3, 80(x31)
    # -------------- DIV ZERO -----------------
    li     x2, 0x51290ce3
    # DIV
    div    x3, x1, x0       # Should be 0xffffffff
    sw     x3, 84(x31)
    div    x3, x2, x0       # Should be 0xffffffff
    sw     x3, 88(x31)
    # DIVU
    divu   x3, x1, x0       # Should be 0xffffffff
    sw     x3, 92(x31)
    divu   x3, x2, x0       # Should be 0xffffffff
    sw     x3, 96(x31)
    # REM
    rem    x3, x1, x0       # Should be 0xbcfec832
    sw     x3, 100(x31)
    rem    x3, x2, x0       # Should be 0x51290ce3
    sw     x3, 104(x31)
    # REMU
    remu   x3, x1, x0       # Should be 0xbcfec832
    sw     x3, 108(x31)
    remu   x3, x2, x0       # Should be 0x51290ce3
    sw     x3, 112(x31)
    # ------------ DIV OVERFLOW ---------------
    li     x1, 0x80000000
    li     x2, 0xffffffff
    # DIV
    div    x3, x1, x2       # Should be 0x80000000
    sw     x3, 116(x31)
    # DIVU
    divu   x3, x1, x2       # Should be 0x00000000
    sw     x3, 120(x31)
    # REM
    rem    x3, x1, x2       # Should be 0x00000000
    sw     x3, 124(x31)
    # REMU
    remu   x3, x1, x2       # Should be 0x80000000
    sw     x3, 128(x31)
    # ---------------- ECALL ------------------
    ecall

