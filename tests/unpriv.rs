use riscv_sim::Env;
use std::fs;

fn test_with_ref(binary_file: &str, ref_file: &str, num_words: usize, is_hex: bool) {
    let mut env = Env::new();
    env.load_from_file(binary_file).unwrap();
    env.run_until_exception();

    // Ref file is a text file containing a list of hex numbers
    let ref_dat: Vec<_> = fs::read_to_string(ref_file)
        .unwrap()
        .lines()
        .map(|line| u32::from_str_radix(line, if is_hex { 16 } else { 10 }).unwrap())
        .collect();

    // Data in mem starts from 0x1000 in bytes, which is 0x400 in words
    let mem_dat = &env.sys.mem.as_u32()[0x400..(0x400 + num_words)];

    assert_eq!(ref_dat, mem_dat);
}

#[test]
fn int_test() {
    test_with_ref(
        "asm/target/int_test.bin",
        "asm/dat/int_test_ref.dat",
        66,
        true,
    );
}

#[test]
fn fibonacci() {
    test_with_ref(
        "asm/target/fibonacci.bin",
        "asm/dat/fibonacci_ref.dat",
        40,
        false,
    );
}

#[test]
fn mul_test() {
    test_with_ref(
        "asm/target/mul_test.bin",
        "asm/dat/mul_test_ref.dat",
        33,
        true,
    );
}

#[test]
fn lrsc_test() {
    test_with_ref(
        "asm/target/lrsc_test.bin",
        "asm/dat/lrsc_test_ref.dat",
        25,
        true,
    );
}

#[test]
fn amo_test() {
    test_with_ref(
        "asm/target/amo_test.bin",
        "asm/dat/amo_test_ref.dat",
        36,
        true,
    );
}

#[test]
fn csr_test() {
    test_with_ref(
        "asm/target/csr_test.bin",
        "asm/dat/csr_test_ref.dat",
        8,
        true,
    );
}
