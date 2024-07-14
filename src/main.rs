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
            ConfigError::InvalidDtb(f) => {
                eprintln!("Invalid dtb file: {}", f.display());
                process::exit(2);
            }
            ConfigError::InvalidKernel(f) => {
                eprintln!("Invalid kernel file: {}", f.display());
                process::exit(3);
            }
        },
    };

    let mut sys = System::from_config(cfg);
    run_forever(&mut sys);
}
