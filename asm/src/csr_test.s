.global _start

.text
_start:
    # Load the base addr of the trace array
    li     x31, 0x00001000
    # Load some random 32-bit numbers
    li     x1, 0xbcfec832
    li     x2, 0x51290ce3
    li     x4, 0x0e27d515
    # ----------------- MUL -------------------
    # CSRRW
    csrw   mscratch, x4
    csrrw  x3, mscratch, x1     # Should be 0x0e27d515
    sw     x3, 0(x31)
    # CSRRS
    csrrs  x3, mscratch, x2     # Should be 0xbcfec832
    sw     x3, 4(x31)
    # CSRRC
    csrrc  x3, mscratch, x1     # Should be 0xfdffccf3
    sw     x3, 8(x31)
    # CSRRWI
    csrrwi x3, mscratch, 0x13   # Should be 0x410104c1
    sw     x3, 12(x31)
    csrrw  x3, mscratch, x2     # Should be 0x00000013
    sw     x3, 16(x31)
    # CSRRSI
    csrrsi x3, mscratch, 0x07   # Should be 0x51290ce3
    sw     x3, 20(x31)
    # CSRRCI
    csrrci x3, mscratch, 0x1d   # Should be 0x51290ce7
    sw     x3, 24(x31)
    csrr   x3, mscratch         # Should be 0x51290ce2
    sw     x3, 28(x31)
    # ---------------- ECALL ------------------
    ecall

