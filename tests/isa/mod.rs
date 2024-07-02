use riscv_sim::{Env, Reg};

fn run_test(binary_file: &str) {
    let mut env = Env::new();
    env.load_from_file(binary_file).unwrap();

    // Modify base addr of RAM
    env.sys.mem.ram_base = 0x8000_0000;
    *env.sys.pc_mut() = 0x8000_0000;

    if let Ok(()) = env.run_for_or_until_ecall(10000) {
        println!("{:#?}", env.sys);
        panic!("Timeout");
    }

    let gp = env.sys.reg(&Reg::new(3)) >> 1;
    if gp != 0 {
        println!("{:#?}", env.sys);
        panic!("Test number {} failed", gp);
    }
}

mod rv32ui;
mod rv32um;
mod rv32ua;
mod rv32mi;
mod rv32si;
