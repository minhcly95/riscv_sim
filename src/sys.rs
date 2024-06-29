use crate::{decode, exec::Result, execute, Exception, Reg};
use colored::*;

pub mod control;
pub mod mem;
pub mod state;

use control::*;
use mem::*;
use state::*;

#[derive(Debug)]
pub struct System {
    pub state: State,
    pub mem: Memory,
    pub ctrl: Control,
}

impl System {
    pub fn new(mem_size: usize) -> System {
        System {
            state: State::new(),
            mem: Memory::new(mem_size),
            ctrl: Control::new(),
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

    pub fn push_trap_m(&mut self, trap: Trap) {
        // Save previous status
        self.ctrl.mpie = self.ctrl.mie;
        self.ctrl.mpp = self.ctrl.privilege;
        // Push new status
        self.ctrl.mie = false;
        self.ctrl.privilege = MPriv::M;
        // Trap information
        self.ctrl.mepc = self.pc();
        self.ctrl.mcause = trap;
        self.ctrl.mtval = 0;
        // Jump to trap vector
        *self.pc_mut() = match self.ctrl.mtvec_mode {
            MTvecMode::Direct => self.ctrl.mtvec_base,
            MTvecMode::Vectored => self.ctrl.mtvec_base, // No interrupt exists
        }
    }

    pub fn pop_trap_m(&mut self) {
        // Restore previous status
        self.ctrl.mie = self.ctrl.mpie;
        self.ctrl.privilege = self.ctrl.mpp;
        // Push dummy status
        self.ctrl.mpie = true;
        self.ctrl.mpp = MPriv::U;
        // Jump back to original PC
        *self.pc_mut() = self.ctrl.mepc;
        // Also clear LR reservation
        self.mem.clear_reservation();
    }

    fn fetch_decode_exec(&mut self) -> Result {
        // Fetch
        let pc = self.pc();
        let code: u32 = self.mem.fetch(pc)?;

        // Decode
        let instr = decode(code).ok_or(Exception::IllegalInstr)?;
        println!("{:8x} {:?}", pc, instr);

        // Execute
        execute(self, &instr)
    }

    fn retire(&mut self, res: &Result) {
        match res {
            Ok(_) => {
                // Count the number of retired instruction if Ok
                if !self.ctrl.minstret_inhibit {
                    self.ctrl.minstret = self.ctrl.minstret.wrapping_add(1)
                }
            }
            Err(e) => {
                // Handle the trap if there's an exception
                self.push_trap_m(Trap::Exception(*e));
            }
        }
        // Count the number of cycles passed
        if !self.ctrl.mcycle_inhibit {
            self.ctrl.mcycle = self.ctrl.mcycle.wrapping_add(1);
        }
    }

    pub fn step(&mut self) -> Result {
        // Fetch decode exec
        let res = self.fetch_decode_exec();
        if let Err(e) = res {
            println!("{:8x} {}", self.pc(), format!("{:?}", e).yellow());
        }

        // Retire
        self.retire(&res);

        res
    }
}
