use std::u64;

use crate::{
    decode::decode,
    exec::execute,
    instr::reg::Reg,
    proc::*,
    run::{load_dtb_from_file, load_image_from_file},
    translate::*,
    trap::TrapCause,
    Config, Exception, Result, Result32, Trap,
};
use colored::*;

pub mod control;
pub mod mem_map;
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
        let dtb = cfg.dtb.clone();
        let kernel = cfg.kernel.clone();

        let mut sys = System {
            cfg,
            state: State::new(),
            mem: MemMap::new(size),
            ctrl: Control::new(),
            code: 0,
        };

        // Adjust the ram base
        let ram_base = sys.cfg.base as u64;
        sys.mem.ram_base = ram_base;
        *sys.pc_mut() = sys.cfg.base;

        // Load binary file to ram
        if let Some(path) = binary {
            load_image_from_file(&mut sys, path, 0).unwrap();
        }

        // Load device tree blob to rom
        if let Some(path) = dtb {
            load_dtb_from_file(&mut sys, path).unwrap();
        }

        // Load kernel file to ram at 0x00400000
        if let Some(path) = kernel {
            load_image_from_file(&mut sys, path, 0x00400000).unwrap();
        }

        // Initialize a0 to hartid (0) and a1 to dtb_base
        *sys.reg_mut(&Reg::new(10)) = 0;
        *sys.reg_mut(&Reg::new(11)) = sys.mem.dtb_base as i32;

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
    // Update and check interrupt
    update_interrupt(sys);
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
    // Tick the timer
    sys.mem.timer.time = sys.mem.timer.time.wrapping_add(1);
}
