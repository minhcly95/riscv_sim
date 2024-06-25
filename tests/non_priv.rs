use riscv_sim::Env;
use std::fs;

#[test]
fn int_test() {
    let mut env = Env::new();
    env.load_from_file("asm/target/int_test.bin").unwrap();
    env.run_until_break();

    // Ref file is a text file containing a list of hex numbers
    let ref_dat: Vec<_> = fs::read_to_string("asm/dat/int_test_ref.dat")
        .unwrap()
        .lines()
        .map(|line| u32::from_str_radix(line, 16).unwrap())
        .collect();

    // Data in mem starts from 0x1000 in bytes, which is 0x400 in words
    let mem_dat = &env.sys.mem.as_u32()[0x400..(0x400 + 66)];

    assert_eq!(ref_dat, mem_dat);
}

#[test]
fn fibonacci() {
    let mut env = Env::new();
    env.load_from_file("asm/target/fibonacci.bin").unwrap();
    env.run_until_break();

    // Ref file is a text file containing a list of decimal numbers
    let ref_dat: Vec<u32> = fs::read_to_string("asm/dat/fibonacci_ref.dat")
        .unwrap()
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();

    // Data in mem starts from 0x1000 in bytes, which is 0x400 in words
    let mem_dat = &env.sys.mem.as_u32()[0x400..(0x400 + 40)];

    assert_eq!(ref_dat, mem_dat);
}

#[test]
fn mul_test() {
    let mut env = Env::new();
    env.load_from_file("asm/target/mul_test.bin").unwrap();
    env.run_until_break();

    // Ref file is a text file containing a list of hex numbers
    let ref_dat: Vec<_> = fs::read_to_string("asm/dat/mul_test_ref.dat")
        .unwrap()
        .lines()
        .map(|line| u32::from_str_radix(line, 16).unwrap())
        .collect();

    // Data in mem starts from 0x1000 in bytes, which is 0x400 in words
    let mem_dat = &env.sys.mem.as_u32()[0x400..(0x400 + 33)];

    assert_eq!(ref_dat, mem_dat);
}
