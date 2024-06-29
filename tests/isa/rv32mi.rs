use super::*;

#[test]
fn csr() {
    run_test("target/isa/rv32mi-p-csr.bin");
}
#[test]
fn mcsr() {
    run_test("target/isa/rv32mi-p-mcsr.bin");
}
#[test]
fn illegal() {
    run_test("target/isa/rv32mi-p-illegal.bin");
}
#[test]
fn ma_fetch() {
    run_test("target/isa/rv32mi-p-ma_fetch.bin");
}
#[test]
fn ma_addr() {
    run_test("target/isa/rv32mi-p-ma_addr.bin");
}
#[test]
fn scall() {
    run_test("target/isa/rv32mi-p-scall.bin");
}
#[test]
fn sbreak() {
    run_test("target/isa/rv32mi-p-sbreak.bin");
}
#[test]
fn shamt() {
    run_test("target/isa/rv32mi-p-shamt.bin");
}
#[test]
fn lw() {
    run_test("target/isa/rv32mi-p-lw-misaligned.bin");
}
#[test]
fn lh() {
    run_test("target/isa/rv32mi-p-lh-misaligned.bin");
}
#[test]
fn sh() {
    run_test("target/isa/rv32mi-p-sh-misaligned.bin");
}
#[test]
fn sw() {
    run_test("target/isa/rv32mi-p-sw-misaligned.bin");
}
#[test]
fn zicntr() {
    run_test("target/isa/rv32mi-p-zicntr.bin");
}
