PREFIX ?= riscv64-unknown-elf-
RISCV_TESTS ?= $(RISCV_HOME)/share/riscv-tests

ASM = fibonacci int_test mul_test lrsc_test amo_test csr_test
ASM_DIR = target/asm
ASM_BIN = $(addprefix $(ASM_DIR)/,$(addsuffix .bin,$(ASM)))
ASM_OBJ = $(addprefix $(ASM_DIR)/,$(addsuffix .o,$(ASM)))

ISA += $(shell cat isa/rv32ui.txt)
ISA += $(shell cat isa/rv32um.txt)
ISA += $(shell cat isa/rv32ua.txt)
ISA += $(shell cat isa/rv32mi.txt)
ISA_DIR = target/isa
ISA_BIN = $(addprefix $(ISA_DIR)/,$(addsuffix .bin,$(ISA)))

.PHONY: all asm isa clean clean-asm clean-isa

all: asm isa

asm: $(ASM_BIN)

$(ASM_BIN): | $(ASM_DIR)

$(ASM_DIR):
	mkdir -p $(ASM_DIR)

$(ASM_DIR)/%.bin: $(ASM_DIR)/%.o
	$(PREFIX)objcopy -j .text -O binary $< $@

$(ASM_DIR)/%.o: asm/src/%.s
	$(PREFIX)as -march=rv32ima_zicsr -o $@ $<

isa: $(ISA_BIN)

$(ISA_BIN): | $(ISA_DIR)

$(ISA_DIR):
	mkdir -p $(ISA_DIR)

$(ISA_DIR)/%.bin: $(RISCV_TESTS)/isa/%
	$(PREFIX)objcopy -O binary $< $@

clean: clean-asm clean-isa

clean-asm:
	rm -f $(ASM_BIN)
	rm -f $(ASM_OBJ)

clean-isa:
	rm -f $(ISA_BIN)
