use super::{advance_pc, Result};
use crate::{
    instr::funct::EnvFunct,
    proc::{pop_trap_m, pop_trap_s},
    sys::{control::MPriv, make_illegal},
    Exception, System, Trap,
};

pub fn execute_env(sys: &mut System, f: &EnvFunct) -> Result {
    match f {
        EnvFunct::Call => execute_ecall(sys),
        EnvFunct::Break => Err(Trap::from_exception(Exception::Breakpoint, 0)),
        EnvFunct::Sret => execute_sret(sys),
        EnvFunct::Mret => execute_mret(sys),
        EnvFunct::Wfi => execute_wfi(sys),
        EnvFunct::SfenceVma => execute_sfence(sys),
    }
}

fn execute_ecall(sys: &mut System) -> Result {
    match sys.ctrl.privilege {
        MPriv::U => Err(Trap::from_exception(Exception::EcallFromU, 0)),
        MPriv::S => Err(Trap::from_exception(Exception::EcallFromS, 0)),
        MPriv::M => Err(Trap::from_exception(Exception::EcallFromM, 0)),
    }
}

fn execute_sret(sys: &mut System) -> Result {
    // Only available in S-mode and M-mode
    if sys.ctrl.privilege == MPriv::U {
        Err(make_illegal(sys))?
    }
    // If TSR = 1, trap in S-mode as well
    if sys.ctrl.tsr && sys.ctrl.privilege == MPriv::S {
        Err(make_illegal(sys))?
    }
    // Do not advance_pc here
    pop_trap_s(sys);
    Ok(())
}

fn execute_mret(sys: &mut System) -> Result {
    // Only available in M-mode
    if sys.ctrl.privilege != MPriv::M {
        Err(make_illegal(sys))?
    }
    // Do not advance_pc here
    pop_trap_m(sys);
    Ok(())
}

fn execute_wfi(sys: &mut System) -> Result {
    // Implemented as NOP, unless TW = 1 in S-mode or U-mode
    if sys.ctrl.tw && sys.ctrl.privilege != MPriv::M {
        Err(make_illegal(sys))
    } else {
        advance_pc(sys);
        Ok(())
    }
}

fn execute_sfence(sys: &mut System) -> Result {
    // Only available in S-mode and M-mode
    if sys.ctrl.privilege == MPriv::U {
        Err(make_illegal(sys))?
    }
    // If TVM = 1, trap in S-mode as well
    if sys.ctrl.tvm && sys.ctrl.privilege == MPriv::S {
        Err(make_illegal(sys))?
    }
    // Otherwise, treated as a NOP (since the sim doesn't have a TLB to flush)
    advance_pc(sys);
    Ok(())
}
