use super::{advance_pc, Result};
use crate::{instr::funct::EnvFunct, sys::{control::MPriv, make_illegal}, Exception, System, Trap};

pub fn execute_env(sys: &mut System, f: &EnvFunct) -> Result {
    match f {
        EnvFunct::Call => match sys.ctrl.privilege {
            MPriv::U => Err(Trap::from_exception(Exception::EcallFromU, 0)),
            MPriv::M => Err(Trap::from_exception(Exception::EcallFromM, 0)),
        },
        EnvFunct::Break => Err(Trap::from_exception(Exception::Breakpoint, 0)),
        EnvFunct::Mret => Ok(sys.pop_trap_m()), // Do not advance_pc here
        EnvFunct::Wfi => {
            // Implemented as NOP, unless TW = 1 in U-mode
            if sys.ctrl.privilege == MPriv::U && sys.ctrl.tw {
                Err(make_illegal(sys))
            } else {
                advance_pc(sys);
                Ok(())
            }
        }
    }
}
