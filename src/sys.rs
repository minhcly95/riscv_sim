pub mod mem;
pub mod state;

use mem::*;
use state::*;

use crate::Reg;

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
}
