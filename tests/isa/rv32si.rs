use super::*;

#[test]
fn csr() {
    run_test("target/isa/rv32si-p-csr.bin");
}
#[test]
fn dirty() {
    run_test("target/isa/rv32si-p-dirty.bin");
}
#[test]
fn ma_fetch() {
    run_test("target/isa/rv32si-p-ma_fetch.bin");
}
#[test]
fn scall() {
    run_test("target/isa/rv32si-p-scall.bin");
}
#[test]
fn sbreak() {
    run_test("target/isa/rv32si-p-sbreak.bin");
}
#[test]
fn wfi() {
    run_test("target/isa/rv32si-p-wfi.bin");
}
