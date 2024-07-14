use std::u64;

use crate::{
    decode::decode, exec::execute, instr::reg::Reg, proc::*, run::load_from_file, translate::*,
    trap::TrapCause, Config, Exception, Result, Result32, Trap,
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
    pub cfg: Config,
    pub state: State,
    pub mem: MemMap,
    pub ctrl: Control,
    code: u32,
}

impl System {
    pub fn new() -> System {
        Self::from_config(Config::new())
    }

    pub fn from_config(cfg: Config) -> System {
        let size = cfg.size.as_u64();
        let binary = cfg.binary.clone();

        let mut sys = System {
            cfg,
            state: State::new(),
            mem: MemMap::new(size),
            ctrl: Control::new(),
            code: 0,
        };

        let ram_base = sys.cfg.base as u64;
        let ram_end = ram_base + size;
        sys.mem.ram_range = ram_base..ram_end;
        *sys.pc_mut() = sys.cfg.base;

        if let Some(path) = binary {
            load_from_file(&mut sys, path).unwrap();
        }
        sys
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
        // Fetch decode exec
        let res = fetch_decode_exec(self);
        if let Err(e) = res {
            log_with_pc(self, &format!("{}", format!("{:?}", e).yellow()), true);
        }

        // Retire
        retire(self, res);

        res
    }
}

pub fn make_illegal(sys: &System) -> Trap {
    Trap::from_exception(Exception::IllegalInstr, sys.code)
}

pub fn log_with_pc(sys: &System, str: &str, debug: bool) {
    if !debug || sys.cfg.verbose {
        println!("{:8x} {str}", sys.pc());
    }
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
    log_with_pc(sys, &format!("{:?}", instr), true);

    // Execute
    execute(sys, &instr)
}

pub fn fetch(sys: &mut System) -> Result32 {
    let vpc = sys.pc();
    let make_trap = |ex| Trap::from_exception(ex, vpc);
    let ppc = translate(sys, vpc, AccessType::Instr).map_err(make_trap)?;
    let attr = AccessAttr {
        atype: AccessType::Instr,
        width: AccessWidth::Word,
        lrsc: false,
        amo: false,
    };
    let code: u32 = sys.mem.read_u32(ppc, attr).map_err(make_trap)?;
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
