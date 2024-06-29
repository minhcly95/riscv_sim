use super::*;

#[test]
fn amoadd_w() {
    run_test("target/isa/rv32ua-p-amoadd_w.bin");
}
#[test]
fn amoand_w() {
    run_test("target/isa/rv32ua-p-amoand_w.bin");
}
#[test]
fn amomax_w() {
    run_test("target/isa/rv32ua-p-amomax_w.bin");
}
#[test]
fn amomaxu_w() {
    run_test("target/isa/rv32ua-p-amomaxu_w.bin");
}
#[test]
fn amomin_w() {
    run_test("target/isa/rv32ua-p-amomin_w.bin");
}
#[test]
fn amominu_w() {
    run_test("target/isa/rv32ua-p-amominu_w.bin");
}
#[test]
fn amoor_w() {
    run_test("target/isa/rv32ua-p-amoor_w.bin");
}
#[test]
fn amoxor_w() {
    run_test("target/isa/rv32ua-p-amoxor_w.bin");
}
#[test]
fn amoswap_w() {
    run_test("target/isa/rv32ua-p-amoswap_w.bin");
}
#[test]
fn lrsc() {
    run_test("target/isa/rv32ua-p-lrsc.bin");
}
