use super::{machine::*, MPriv, Result, Result32};
use crate::{
    instr::csr::{CsrRegU::*, *},
    sys::make_illegal,
    System,
};

pub fn csr_read_u(sys: &mut System, csr: &CsrRegU) -> Result32 {
    match csr {
        // Unprivileged counter/timer
        Cycle => read_cycle(sys),
        Time => read_cycle(sys),
        InstRet => read_instret(sys),
        HpmCounter(_) => Err(make_illegal(sys)),
        Cycleh => read_cycleh(sys),
        InstReth => read_instreth(sys),
        HpmCounterh(_) => Err(make_illegal(sys)),
    }
}

pub fn csr_write_u(sys: &mut System, csr: &CsrRegU, _val: u32) -> Result {
    match csr {
        // Unprivileged counter/timer (read only)
        Cycle => Err(make_illegal(sys)),
        Time => Err(make_illegal(sys)),
        InstRet => Err(make_illegal(sys)),
        HpmCounter(_) => Err(make_illegal(sys)),
        Cycleh => Err(make_illegal(sys)),
        InstReth => Err(make_illegal(sys)),
        HpmCounterh(_) => Err(make_illegal(sys)),
    }
}

// ------------------ CYCLE ---------------------
fn read_cycle(sys: &System) -> Result32 {
    // Can only read if in M-mode or enabled in mcounteren
    if sys.ctrl.privilege == MPriv::M || sys.ctrl.mcycle_en {
        Ok(read_mcycle(sys))
    } else {
        Err(make_illegal(sys))
    }
}

fn read_cycleh(sys: &System) -> Result32 {
    // Can only read if in M-mode or enabled in mcounteren
    if sys.ctrl.privilege == MPriv::M || sys.ctrl.mcycle_en {
        Ok(read_mcycleh(sys))
    } else {
        Err(make_illegal(sys))
    }
}

// ----------------- INSTRET --------------------
fn read_instret(sys: &System) -> Result32 {
    // Can only read if in M-mode or enabled in mcounteren
    if sys.ctrl.privilege == MPriv::M || sys.ctrl.minstret_en {
        Ok(read_minstret(sys))
    } else {
        Err(make_illegal(sys))
    }
}

fn read_instreth(sys: &System) -> Result32 {
    // Can only read if in M-mode or enabled in mcounteren
    if sys.ctrl.privilege == MPriv::M || sys.ctrl.minstret_en {
        Ok(read_minstreth(sys))
    } else {
        Err(make_illegal(sys))
    }
}
