use bytesize::ByteSize;
use riscv_sim::*;
use std::{path::PathBuf, str::FromStr};

fn run_test(binary_file: &str) {
    let cfg = Config {
        binary: Some(PathBuf::from_str(binary_file).unwrap()),
        size: ByteSize::mib(1),
        base: 0,
        verbose: true,
    };

    let mut sys = System::from_config(cfg); // 1MB

    // Modify base addr of RAM
    sys.mem.ram_base = 0x8000_0000;
    *sys.pc_mut() = 0x8000_0000;

    if let Ok(()) = run_for_or_until_ecall(&mut sys, 10000) {
        println!("{:#?}", &sys);
        panic!("Timeout");
    }

    let gp = sys.reg(&Reg::new(3)) >> 1;
    if gp != 0 {
        println!("{:#?}", sys);
        panic!("Test number {} failed", gp);
    }
}

mod rv32mi;
mod rv32si;
mod rv32ua;
mod rv32ui;
mod rv32um;
