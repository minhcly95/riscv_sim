use bytesize::ByteSize;
use riscv_sim::*;
use std::{fs, path::PathBuf, str::FromStr};

fn test_with_ref(binary_file: &str, ref_file: &str, num_words: usize, is_hex: bool) {
    let cfg = Config {
        binary: Some(PathBuf::from_str(binary_file).unwrap()),
        size: ByteSize::mib(1),
        base: 0,
        dtb: None,
        kernel: None,
        verbose: true,
    };

    let mut sys = System::from_config(cfg);

    if let Ok(()) = run_for_or_until_ecall(&mut sys, 5000) {
        panic!("Timeout");
    }

    // Ref file is a text file containing a list of hex numbers
    let ref_dat: Vec<_> = fs::read_to_string(ref_file)
        .unwrap()
        .lines()
        .map(|line| u32::from_str_radix(line, if is_hex { 16 } else { 10 }).unwrap())
        .collect();

    // Data in mem starts from 0x1000 in bytes, which is 0x400 in words
    let mem_dat = &sys.mem.ram.as_u32()[0x400..(0x400 + num_words)];

    assert_eq!(ref_dat, mem_dat);
}

#[test]
fn int_test() {
    test_with_ref(
        "target/asm/int_test.bin",
        "asm/dat/int_test_ref.dat",
        66,
        true,
    );
}

#[test]
fn fibonacci() {
    test_with_ref(
        "target/asm/fibonacci.bin",
        "asm/dat/fibonacci_ref.dat",
        40,
        false,
    );
}

#[test]
fn mul_test() {
    test_with_ref(
        "target/asm/mul_test.bin",
        "asm/dat/mul_test_ref.dat",
        33,
        true,
    );
}

#[test]
fn lrsc_test() {
    test_with_ref(
        "target/asm/lrsc_test.bin",
        "asm/dat/lrsc_test_ref.dat",
        25,
        true,
    );
}

#[test]
fn amo_test() {
    test_with_ref(
        "target/asm/amo_test.bin",
        "asm/dat/amo_test_ref.dat",
        36,
        true,
    );
}

#[test]
fn csr_test() {
    test_with_ref(
        "target/asm/csr_test.bin",
        "asm/dat/csr_test_ref.dat",
        8,
        true,
    );
}
