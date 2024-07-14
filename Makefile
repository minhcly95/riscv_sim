TARGET ?= riscv32-unknown-linux-gnu

ASM = fibonacci int_test mul_test lrsc_test amo_test csr_test
ASM_DIR = target/asm
ASM_BIN = $(addprefix $(ASM_DIR)/,$(addsuffix .bin,$(ASM)))
ASM_OBJ = $(addprefix $(ASM_DIR)/,$(addsuffix .o,$(ASM)))

ISA += $(shell cat isa/rv32ui.txt)
ISA += $(shell cat isa/rv32um.txt)
ISA += $(shell cat isa/rv32ua.txt)
ISA += $(shell cat isa/rv32mi.txt)
ISA += $(shell cat isa/rv32si.txt)
ISA_DIR = target/isa
ISA_BIN = $(addprefix $(ISA_DIR)/,$(addsuffix .bin,$(ISA)))

.PHONY: all asm isa clean clean-asm clean-isa

all: asm isa dt

# Build-in tests
asm: $(ASM_BIN)

$(ASM_BIN): | $(ASM_DIR)

$(ASM_DIR):
	mkdir -p $(ASM_DIR)

$(ASM_DIR)/%.bin: $(ASM_DIR)/%.o
	$(TARGET)-objcopy -j .text -O binary $< $@

$(ASM_DIR)/%.o: asm/src/%.s
	$(TARGET)-as -march=rv32ima_zicsr -mabi=ilp32 -o $@ $<

# riscv-tests
isa: $(ISA_BIN)

$(ISA_BIN): | $(ISA_DIR)

$(ISA_DIR):
	mkdir -p $(ISA_DIR)

$(ISA_DIR)/%.bin: $(RISCV_TESTS)/isa/%
	$(TARGET)-objcopy -O binary $< $@

# Device tree
dt: target/riscv_sim.dtb

target/riscv_sim.dtb: dt/riscv_sim.dts
	dtc -O dtb -o $@ $<

# Clean
clean: clean-asm clean-isa clean-dt

clean-asm:
	rm -f $(ASM_BIN)
	rm -f $(ASM_OBJ)

clean-isa:
	rm -f $(ISA_BIN)

clean-dt:
	rm -f target/riscv_sim.dtb
