use super::{machine::*, MPriv, Result, Result32};
use crate::{
    instr::csr::{CsrRegU::*, *},
    Exception::IllegalInstr,
    System,
};

pub fn csr_read_u(sys: &mut System, csr: &CsrRegU) -> Result32 {
    match csr {
        // Unprivileged counter/timer
        Cycle => read_cycle(sys),
        InstRet => read_instret(sys),
        HpmCounter(_) => Err(IllegalInstr),
        Cycleh => read_cycleh(sys),
        InstReth => read_instreth(sys),
        HpmCounterh(_) => Err(IllegalInstr),
    }
}

pub fn csr_write_u(_sys: &mut System, csr: &CsrRegU, _val: u32) -> Result {
    match csr {
        // Unprivileged counter/timer (read only)
        Cycle => Err(IllegalInstr),
        InstRet => Err(IllegalInstr),
        HpmCounter(_) => Err(IllegalInstr),
        Cycleh => Err(IllegalInstr),
        InstReth => Err(IllegalInstr),
        HpmCounterh(_) => Err(IllegalInstr),
    }
}

// ------------------ CYCLE ---------------------
fn read_cycle(sys: &System) -> Result32 {
    // Can only read if in M-mode or enabled in mcounteren
    if sys.ctrl.privilege == MPriv::M || sys.ctrl.mcycle_en {
        Ok(read_mcycle(sys))
    } else {
        Err(IllegalInstr)
    }
}

fn read_cycleh(sys: &System) -> Result32 {
    // Can only read if in M-mode or enabled in mcounteren
    if sys.ctrl.privilege == MPriv::M || sys.ctrl.mcycle_en {
        Ok(read_mcycleh(sys))
    } else {
        Err(IllegalInstr)
    }
}

// ----------------- INSTRET --------------------
fn read_instret(sys: &System) -> Result32 {
    // Can only read if in M-mode or enabled in mcounteren
    if sys.ctrl.privilege == MPriv::M || sys.ctrl.minstret_en {
        Ok(read_minstret(sys))
    } else {
        Err(IllegalInstr)
    }
}

fn read_instreth(sys: &System) -> Result32 {
    // Can only read if in M-mode or enabled in mcounteren
    if sys.ctrl.privilege == MPriv::M || sys.ctrl.minstret_en {
        Ok(read_minstreth(sys))
    } else {
        Err(IllegalInstr)
    }
}
