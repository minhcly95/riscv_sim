use crate::{trap::TrapCause, Exception, System, Trap};
use colored::*;
use std::{fs, io};

pub struct Env {
    pub sys: System,
}

impl Env {
    pub fn new() -> Env {
        let sys = System::new(0x100000);
        Env { sys }
    }

    pub fn load_from_file(&mut self, file_name: &str) -> io::Result<()> {
        let image = fs::read(file_name)?;
        let len = image.len();
        self.log_with_pc(&format!("{} with {len} bytes", "Load image".blue()));
        self.sys.mem.ram.as_u8_mut()[0..len].copy_from_slice(&image);
        Ok(())
    }

    pub fn run_until_trapped(&mut self) -> Trap {
        loop {
            if let Err(trap) = self.sys.step() {
                self.log_with_pc(&format!(
                    "{} due to a trap: {}",
                    "Break".yellow(),
                    format!("{:?}", trap).yellow()
                ));
                return trap;
            }
        }
    }

    pub fn run_until_ecall(&mut self) {
        loop {
            if let Err(Trap {
                cause: TrapCause::Exception(ex),
                ..
            }) = self.sys.step()
            {
                if ex == Exception::EcallFromM
                    || ex == Exception::EcallFromS
                    || ex == Exception::EcallFromU
                {
                    self.log_with_pc(&format!(
                        "{} due to an Ecall: {}",
                        "Break".yellow(),
                        format!("{:?}", ex).yellow()
                    ));
                    return;
                }
            }
        }
    }

    pub fn run_for(&mut self, repeat: usize) {
        for _ in 0..repeat {
            let _ = self.sys.step();
        }
    }

    pub fn run_for_or_until_ecall(&mut self, repeat: usize) -> Result<(), Exception> {
        for _ in 0..repeat {
            if let Err(Trap {
                cause: TrapCause::Exception(ex),
                ..
            }) = self.sys.step()
            {
                if ex == Exception::EcallFromM
                    || ex == Exception::EcallFromS
                    || ex == Exception::EcallFromU
                {
                    self.log_with_pc(&format!(
                        "{} due to an exception: {}",
                        "Break".yellow(),
                        format!("{:?}", ex).yellow()
                    ));
                    return Err(ex);
                }
            }
        }
        Ok(())
    }

    fn log_with_pc(&self, str: &str) {
        println!("{:8x} {str}", self.sys.pc());
    }
}
