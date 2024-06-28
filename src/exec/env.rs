use super::{advance_pc, Result};
use crate::{instr::funct::EnvFunct, sys::control::MPriv, Exception, System};

pub fn execute_env(sys: &mut System, f: &EnvFunct) -> Result {
    match f {
        EnvFunct::Call => match sys.ctrl.privilege {
            MPriv::U => Err(Exception::EcallFromU),
            MPriv::M => Err(Exception::EcallFromM),
        },
        EnvFunct::Break => Err(Exception::Breakpoint),
        EnvFunct::Mret => Ok(sys.pop_trap_m()), // Do not advance_pc here
        EnvFunct::Wfi => {
            // Implemented as NOP, unless TW = 1 in U-mode
            if sys.ctrl.privilege == MPriv::U && sys.ctrl.tw {
                Err(Exception::IllegalInstr)
            } else {
                advance_pc(sys);
                Ok(())
            }
        }
    }
}
