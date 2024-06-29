use super::*;

#[test]
fn div() {
    run_test("target/isa/rv32um-p-div.bin");
}
#[test]
fn divu() {
    run_test("target/isa/rv32um-p-divu.bin");
}
#[test]
fn mul() {
    run_test("target/isa/rv32um-p-mul.bin");
}
#[test]
fn mulh() {
    run_test("target/isa/rv32um-p-mulh.bin");
}
#[test]
fn mulhsu() {
    run_test("target/isa/rv32um-p-mulhsu.bin");
}
#[test]
fn mulhu() {
    run_test("target/isa/rv32um-p-mulhu.bin");
}
#[test]
fn rem() {
    run_test("target/isa/rv32um-p-rem.bin");
}
#[test]
fn remu() {
    run_test("target/isa/rv32um-p-remu.bin");
}
