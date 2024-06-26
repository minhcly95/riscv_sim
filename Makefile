ASM = fibonacci int_test mul_test lrsc_test amo_test

.PHONY: asm clean-asm

run:
	cargo run

build:
	cargo build

asm: $(addprefix asm/target/,$(addsuffix .bin,$(ASM)))

asm/target/%.bin: asm/src/%.s
	riscv64-linux-gnu-as -march=rv32ima -o asm/target/$*.o $<
	riscv64-linux-gnu-objcopy -j .text -O binary asm/target/$*.o $@

clean-asm:
	rm asm/target/*.bin
	rm asm/target/*.o
