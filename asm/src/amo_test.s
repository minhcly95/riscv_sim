.global _start

.text
_start:
    # Load the base addr of the trace array
    li        x31, 0x00001000
    # Load 2 random 32-bit numbers
    li        x1, 0xbcfec832
    li        x2, 0x51290ce3
    # ----------------- AMO -------------------
    # AMOSWAP
    sw        x1, (x31)
    amoswap.w x3, x2, (x31) # Should be 0x51290ce3
    sw        x3, 4(x31)    # Should be 0xbcfec832
    addi      x31, x31, 8   # Base + 8
    sw        x2, (x31)
    amoswap.w x3, x1, (x31) # Should be 0xbcfec832
    sw        x3, 4(x31)    # Should be 0x51290ce3
    addi      x31, x31, 8   # Base + 16
    # AMOADD
    sw        x1, (x31)
    amoadd.w  x3, x2, (x31) # Should be 0x0e27d515
    sw        x3, 4(x31)    # Should be 0xbcfec832
    addi      x31, x31, 8   # Base + 24
    sw        x2, (x31)
    amoadd.w  x3, x1, (x31) # Should be 0x0e27d515
    sw        x3, 4(x31)    # Should be 0x51290ce3
    addi      x31, x31, 8   # Base + 32
    # AMOXOR
    sw        x1, (x31)
    amoxor.w  x3, x2, (x31) # Should be 0xedd7c4d1
    sw        x3, 4(x31)    # Should be 0xbcfec832
    addi      x31, x31, 8   # Base + 40
    sw        x2, (x31)
    amoxor.w  x3, x1, (x31) # Should be 0xedd7c4d1
    sw        x3, 4(x31)    # Should be 0x51290ce3
    addi      x31, x31, 8   # Base + 48
    # AMOOR
    sw        x1, (x31)
    amoor.w   x3, x2, (x31) # Should be 0xfdffccf3
    sw        x3, 4(x31)    # Should be 0xbcfec832
    addi      x31, x31, 8   # Base + 56
    sw        x2, (x31)
    amoor.w   x3, x1, (x31) # Should be 0xfdffccf3
    sw        x3, 4(x31)    # Should be 0x51290ce3
    addi      x31, x31, 8   # Base + 64
    # AMOAND
    sw        x1, (x31)
    amoand.w  x3, x2, (x31) # Should be 0x10280822
    sw        x3, 4(x31)    # Should be 0xbcfec832
    addi      x31, x31, 8   # Base + 72
    sw        x2, (x31)
    amoand.w  x3, x1, (x31) # Should be 0x10280822
    sw        x3, 4(x31)    # Should be 0x51290ce3
    addi      x31, x31, 8   # Base + 80
    # AMOMIN
    sw        x1, (x31)
    amomin.w  x3, x2, (x31) # Should be 0xbcfec832
    sw        x3, 4(x31)    # Should be 0xbcfec832
    addi      x31, x31, 8   # Base + 88
    sw        x2, (x31)
    amomin.w  x3, x1, (x31) # Should be 0xbcfec832
    sw        x3, 4(x31)    # Should be 0x51290ce3
    addi      x31, x31, 8   # Base + 96
    # AMOMAX
    sw        x1, (x31)
    amomax.w  x3, x2, (x31) # Should be 0x51290ce3
    sw        x3, 4(x31)    # Should be 0xbcfec832
    addi      x31, x31, 8   # Base + 104
    sw        x2, (x31)
    amomax.w  x3, x1, (x31) # Should be 0x51290ce3
    sw        x3, 4(x31)    # Should be 0x51290ce3
    addi      x31, x31, 8   # Base + 112
    # AMOMINU
    sw        x1, (x31)
    amominu.w x3, x2, (x31) # Should be 0x51290ce3
    sw        x3, 4(x31)    # Should be 0xbcfec832
    addi      x31, x31, 8   # Base + 120
    sw        x2, (x31)
    amominu.w x3, x1, (x31) # Should be 0x51290ce3
    sw        x3, 4(x31)    # Should be 0x51290ce3
    addi      x31, x31, 8   # Base + 128
    # AMOMAXU
    sw        x1, (x31)
    amomaxu.w x3, x2, (x31) # Should be 0xbcfec832
    sw        x3, 4(x31)    # Should be 0xbcfec832
    addi      x31, x31, 8   # Base + 136
    sw        x2, (x31)
    amomaxu.w x3, x1, (x31) # Should be 0xbcfec832
    sw        x3, 4(x31)    # Should be 0x51290ce3
    addi      x31, x31, 8   # Base + 144
    # ---------------- ECALL ------------------
    ecall

