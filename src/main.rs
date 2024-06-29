use riscv_sim::Env;
use std::env;

fn main() {
    let mut args = env::args();
    args.next();
    let file_name = args.next().unwrap();

    let mut env = Env::new();
    env.load_from_file(&file_name).unwrap();
    env.run_until_ecall();
    println!("{:#?}", env.sys);
}
