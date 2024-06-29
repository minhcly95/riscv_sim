use super::*;

#[test]
fn simple() {
    run_test("target/isa/rv32ui-p-simple.bin");
}
#[test]
fn add() {
    run_test("target/isa/rv32ui-p-add.bin");
}
#[test]
fn addi() {
    run_test("target/isa/rv32ui-p-addi.bin");
}
#[test]
fn and() {
    run_test("target/isa/rv32ui-p-and.bin");
}
#[test]
fn andi() {
    run_test("target/isa/rv32ui-p-andi.bin");
}
#[test]
fn auipc() {
    run_test("target/isa/rv32ui-p-auipc.bin");
}
#[test]
fn beq() {
    run_test("target/isa/rv32ui-p-beq.bin");
}
#[test]
fn bge() {
    run_test("target/isa/rv32ui-p-bge.bin");
}
#[test]
fn bgeu() {
    run_test("target/isa/rv32ui-p-bgeu.bin");
}
#[test]
fn blt() {
    run_test("target/isa/rv32ui-p-blt.bin");
}
#[test]
fn bltu() {
    run_test("target/isa/rv32ui-p-bltu.bin");
}
#[test]
fn bne() {
    run_test("target/isa/rv32ui-p-bne.bin");
}
#[test]
fn fence_i() {
    run_test("target/isa/rv32ui-p-fence_i.bin");
}
#[test]
fn jal() {
    run_test("target/isa/rv32ui-p-jal.bin");
}
#[test]
fn jalr() {
    run_test("target/isa/rv32ui-p-jalr.bin");
}
#[test]
fn lb() {
    run_test("target/isa/rv32ui-p-lb.bin");
}
#[test]
fn lbu() {
    run_test("target/isa/rv32ui-p-lbu.bin");
}
#[test]
fn lh() {
    run_test("target/isa/rv32ui-p-lh.bin");
}
#[test]
fn lhu() {
    run_test("target/isa/rv32ui-p-lhu.bin");
}
#[test]
fn lw() {
    run_test("target/isa/rv32ui-p-lw.bin");
}
#[test]
fn lui() {
    run_test("target/isa/rv32ui-p-lui.bin");
}
#[test]
fn or() {
    run_test("target/isa/rv32ui-p-or.bin");
}
#[test]
fn ori() {
    run_test("target/isa/rv32ui-p-ori.bin");
}
#[test]
fn sb() {
    run_test("target/isa/rv32ui-p-sb.bin");
}
#[test]
fn sh() {
    run_test("target/isa/rv32ui-p-sh.bin");
}
#[test]
fn sw() {
    run_test("target/isa/rv32ui-p-sw.bin");
}
#[test]
fn sll() {
    run_test("target/isa/rv32ui-p-sll.bin");
}
#[test]
fn slli() {
    run_test("target/isa/rv32ui-p-slli.bin");
}
#[test]
fn slt() {
    run_test("target/isa/rv32ui-p-slt.bin");
}
#[test]
fn slti() {
    run_test("target/isa/rv32ui-p-slti.bin");
}
#[test]
fn sltiu() {
    run_test("target/isa/rv32ui-p-sltiu.bin");
}
#[test]
fn sltu() {
    run_test("target/isa/rv32ui-p-sltu.bin");
}
#[test]
fn sra() {
    run_test("target/isa/rv32ui-p-sra.bin");
}
#[test]
fn srai() {
    run_test("target/isa/rv32ui-p-srai.bin");
}
#[test]
fn srl() {
    run_test("target/isa/rv32ui-p-srl.bin");
}
#[test]
fn srli() {
    run_test("target/isa/rv32ui-p-srli.bin");
}
#[test]
fn sub() {
    run_test("target/isa/rv32ui-p-sub.bin");
}
#[test]
fn xor() {
    run_test("target/isa/rv32ui-p-xor.bin");
}
#[test]
fn xori() {
    run_test("target/isa/rv32ui-p-xori.bin");
}
