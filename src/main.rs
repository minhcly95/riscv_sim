use clap::Parser;
use config::ConfigError;
use riscv_sim::*;
use std::process;

fn main() {
    let cfg = Config::parse();

    let cfg = match cfg.validate() {
        Ok(c) => c,
        Err(e) => match e {
            ConfigError::InvalidBinary(f) => {
                eprintln!("Invalid binary file: {}", f.display());
                process::exit(1);
            }
        },
    };

    let mut sys = System::from_config(cfg);
    run_until_ecall(&mut sys);
}
