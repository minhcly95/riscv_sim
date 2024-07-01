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
        Time => read_time(sys),
        InstRet => read_instret(sys),
        HpmCounter(_) => Err(make_illegal(sys)),
        Cycleh => read_cycleh(sys),
        Timeh => read_timeh(sys),
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
        Timeh => Err(make_illegal(sys)),
        InstReth => Err(make_illegal(sys)),
        HpmCounterh(_) => Err(make_illegal(sys)),
    }
}

// ------------------ CYCLE ---------------------
fn read_cycle(sys: &System) -> Result32 {
    // Must take into account mcounteren and scounteren
    if sys.ctrl.privilege == MPriv::M
        || sys.ctrl.mcycle_en && (sys.ctrl.privilege == MPriv::S || sys.ctrl.scycle_en)
    {
        Ok(read_mcycle(sys))
    } else {
        Err(make_illegal(sys))
    }
}

fn read_cycleh(sys: &System) -> Result32 {
    // Must take into account mcounteren and scounteren
    if sys.ctrl.privilege == MPriv::M
        || sys.ctrl.mcycle_en && (sys.ctrl.privilege == MPriv::S || sys.ctrl.scycle_en)
    {
        Ok(read_mcycleh(sys))
    } else {
        Err(make_illegal(sys))
    }
}

// ------------------- TIME ---------------------
fn read_time(sys: &System) -> Result32 {
    // Must take into account mcounteren and scounteren
    if sys.ctrl.privilege == MPriv::M
        || sys.ctrl.mtime_en && (sys.ctrl.privilege == MPriv::S || sys.ctrl.stime_en)
    {
        Ok(read_mcycle(sys))
    } else {
        Err(make_illegal(sys))
    }
}

fn read_timeh(sys: &System) -> Result32 {
    // Must take into account mcounteren and scounteren
    if sys.ctrl.privilege == MPriv::M
        || sys.ctrl.mtime_en && (sys.ctrl.privilege == MPriv::S || sys.ctrl.stime_en)
    {
        Ok(read_mcycleh(sys))
    } else {
        Err(make_illegal(sys))
    }
}

// ----------------- INSTRET --------------------
fn read_instret(sys: &System) -> Result32 {
    // Must take into account mcounteren and scounteren
    if sys.ctrl.privilege == MPriv::M
        || sys.ctrl.minstret_en && (sys.ctrl.privilege == MPriv::S || sys.ctrl.sinstret_en)
    {
        Ok(read_minstret(sys))
    } else {
        Err(make_illegal(sys))
    }
}

fn read_instreth(sys: &System) -> Result32 {
    // Must take into account mcounteren and scounteren
    if sys.ctrl.privilege == MPriv::M
        || sys.ctrl.minstret_en && (sys.ctrl.privilege == MPriv::S || sys.ctrl.sinstret_en)
    {
        Ok(read_minstreth(sys))
    } else {
        Err(make_illegal(sys))
    }
}
