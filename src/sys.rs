use crate::{
    decode::decode, exec::execute, instr::reg::Reg, interrupt::check_interrupt, translate::*,
    trap::TrapCause, Exception, Result, Result32, Trap,
};
use colored::*;

pub mod control;
pub mod mem_map;
pub mod ram;
pub mod state;

use control::*;
use mem_map::*;
use state::*;

#[derive(Debug)]
pub struct System {
    pub state: State,
    pub mem: MemMap,
    pub ctrl: Control,
    code: u32,
}

impl System {
    pub fn new(mem_size: usize) -> System {
        System {
            state: State::new(),
            mem: MemMap::new(mem_size),
            ctrl: Control::new(),
            code: 0,
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
        self.ctrl.mtrap = trap;
        // Jump to trap vector
        *self.pc_mut() = trap_vector_addr(
            &self.ctrl.mtrap,
            self.ctrl.mtvec_base,
            &self.ctrl.mtvec_mode,
        );
    }

    pub fn pop_trap_m(&mut self) {
        // Restore previous status
        self.ctrl.mie = self.ctrl.mpie;
        self.ctrl.privilege = self.ctrl.mpp;
        // Push dummy status
        self.ctrl.mpie = true;
        self.ctrl.mpp = MPriv::U;
        // If move to a less privilege mode, clear MPRV
        if self.ctrl.privilege != MPriv::M {
            self.ctrl.mprv = false;
        }
        // Jump back to original PC
        *self.pc_mut() = self.ctrl.mepc;
        // Also clear LR reservation
        self.mem.clear_reservation();
    }

    pub fn push_trap_s(&mut self, trap: Trap) {
        // Save previous status
        self.ctrl.spie = self.ctrl.sie;
        self.ctrl.spp =
            SPriv::from_m(self.ctrl.privilege).expect("Cannot trap to S-mode from M-mode");
        // Push new status
        self.ctrl.sie = false;
        self.ctrl.privilege = MPriv::S;
        // Trap information
        self.ctrl.sepc = self.pc();
        self.ctrl.strap = trap;
        // Jump to trap vector
        *self.pc_mut() = trap_vector_addr(
            &self.ctrl.strap,
            self.ctrl.stvec_base,
            &self.ctrl.stvec_mode,
        );
    }

    pub fn pop_trap_s(&mut self) {
        // Restore previous status
        self.ctrl.sie = self.ctrl.spie;
        self.ctrl.privilege = MPriv::from_s(self.ctrl.spp);
        // Push dummy status
        self.ctrl.spie = true;
        self.ctrl.spp = SPriv::U;
        // Clear MPRV (since SRET always change the privilege mode to either S or U)
        self.ctrl.mprv = false;
        // Jump back to original PC
        *self.pc_mut() = self.ctrl.sepc;
        // Also clear LR reservation
        self.mem.clear_reservation();
    }

    pub fn step(&mut self) -> Result {
        // Fetch decode exec
        let res = fetch_decode_exec(self);
        if let Err(e) = res {
            println!("{:8x} {}", self.pc(), format!("{:?}", e).yellow());
        }

        // Retire
        retire(self, res);

        res
    }
}

pub fn make_illegal(sys: &System) -> Trap {
    Trap::from_exception(Exception::IllegalInstr, sys.code)
}

fn fetch_decode_exec(sys: &mut System) -> Result {
    // Check interrupt
    check_interrupt(sys)?;

    // Fetch
    let code = fetch(sys)?;
    sys.code = code;

    // Decode
    let instr = decode(code).ok_or(Trap {
        cause: TrapCause::Exception(Exception::IllegalInstr),
        val: code,
    })?;
    println!("{:8x} {:?}", sys.pc(), instr);

    // Execute
    execute(sys, &instr)
}

fn fetch(sys: &mut System) -> Result32 {
    let vpc = sys.pc();
    let make_trap = |ex| Trap::from_exception(ex, vpc);
    let ppc = translate(sys, vpc, AccessType::Instr).map_err(make_trap)?;
    let code: u32 = sys.mem.fetch(ppc).map_err(make_trap)?;
    Ok(code)
}

fn retire(sys: &mut System, res: Result) {
    match res {
        Ok(_) => {
            // Count the number of retired instruction if Ok
            if !sys.ctrl.minstret_inhibit {
                sys.ctrl.minstret = sys.ctrl.minstret.wrapping_add(1)
            }
        }
        Err(trap) => {
            // Handle the trap if there's an exception
            handle_trap(sys, trap);
        }
    }
    // Count the number of cycles passed
    if !sys.ctrl.mcycle_inhibit {
        sys.ctrl.mcycle = sys.ctrl.mcycle.wrapping_add(1);
    }
}

fn handle_trap(sys: &mut System, trap: Trap) {
    let Control {
        privilege,
        medeleg,
        mideleg,
        ..
    } = &sys.ctrl;
    // Determine the mode (M or S) to handle the trap
    match trap.cause {
        TrapCause::Exception(ex) => {
            if *privilege == MPriv::M || !medeleg.get(&ex) {
                sys.push_trap_m(trap);
            } else {
                sys.push_trap_s(trap);
            }
        }
        TrapCause::Interrupt(int) => {
            if *privilege == MPriv::M || !mideleg.get(&int) {
                sys.push_trap_m(trap);
            } else {
                sys.push_trap_s(trap);
            }
        }
    }
}

fn trap_vector_addr(trap: &Trap, base: u32, mode: &TvecMode) -> u32 {
    match mode {
        TvecMode::Direct => base,
        TvecMode::Vectored => match trap.cause {
            TrapCause::Exception(_) => base,
            TrapCause::Interrupt(int) => base + (int.to_int() << 2),
        },
    }
}
