ASM = fibonacci basic_test

.PHONY: asm

run:
	cargo run

build:
	cargo build

asm: $(addprefix asm/target/,$(addsuffix .bin,$(ASM)))

asm/target/%.bin: asm/src/%.s
	riscv64-linux-gnu-as -march=rv32i -o asm/target/$*.o $<
	riscv64-linux-gnu-objcopy -j .text -O binary asm/target/$*.o $@
