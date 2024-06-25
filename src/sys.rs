use crate::{decode, exec::Result, execute, Exception, Reg};

pub mod mem;
pub mod state;

use mem::*;
use state::*;

#[derive(Debug)]
pub struct System {
    pub state: State,
    pub mem: Memory,
}

impl System {
    pub fn new(mem_size: usize) -> System {
        System {
            state: State::new(),
            mem: Memory::new(mem_size),
        }
    }

    pub fn reg(&self, r: &Reg) -> i32 {
        self.state.reg(r)
    }

    pub fn reg_mut(&mut self, r: &Reg) -> &mut i32 {
        self.state.reg_mut(r)
    }

    pub fn pc(&self) -> u32 {
        self.state.pc()
    }

    pub fn pc_mut(&mut self) -> &mut u32 {
        self.state.pc_mut()
    }

    pub fn step(&mut self) -> Result {
        // Fetch
        let pc = self.pc();
        let code: u32 = self.mem.fetch(pc)?;

        // Decode
        let instr = decode(code).ok_or(Exception::IllegalInstr)?;

        // Execute
        execute(self, &instr)
    }

    pub fn run_until_break(&mut self) -> Exception {
        loop {
            if let Err(e) = self.step() {
                break e;
            }
        }
    }
}
