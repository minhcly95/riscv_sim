use crate::{Exception, System};
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
        self.log_with_pc(&format!(
            "{} with {len} bytes",
            "Load image".blue()
        ));
        self.sys.mem.as_u8_mut()[0..len].copy_from_slice(&image);
        Ok(())
    }

    pub fn run_until_break(&mut self) -> Exception {
        let ex = self.sys.run_until_break();
        self.log_with_pc(&format!(
            "{} due to an exception: {}",
            "Break".yellow(),
            format!("{:?}", ex).yellow()
        ));
        ex
    }

    fn log_with_pc(&self, str: &str) {
        println!("{:8x} {str}", self.sys.pc());
    }
}
