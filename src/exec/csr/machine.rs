use super::{Result, Result32};
use crate::{
    instr::csr::{CsrRegM::*, *},
    sys::{control::*, make_illegal},
    trap::TrapCause,
    System,
};

const MISA_MXL_32: u32 = 1 << 30;
const MISA_EXT_I: u32 = 1 << 8;
const MISA_EXT_M: u32 = 1 << 12;
const MISA_EXT_A: u32 = 1 << 0;
const MISA_EXT_U: u32 = 1 << 20;
const MISA: u32 = MISA_MXL_32 | MISA_EXT_I | MISA_EXT_M | MISA_EXT_A | MISA_EXT_U;

pub fn csr_read_m(sys: &mut System, csr: &CsrRegM) -> Result32 {
    match csr {
        // Machine information
        MVendorId => Ok(0),
        MArchId => Ok(0),
        MImpId => Ok(0),
        MHartId => Ok(0),
        MConfigPtr => Ok(0),
        // Machine trap setup
        MStatus => Ok(read_mstatus(sys)),
        MIsa => Ok(MISA),
        MEdeleg => Err(make_illegal(sys)), // S-mode not supported
        MIdeleg => Err(make_illegal(sys)), // S-mode not supported
        MIe => Ok(0),                      // No interrupt exists
        MTvec => Ok(read_mtvec(sys)),
        MCounterEn => Ok(read_mcounteren(sys)),
        MStatush => Ok(0),
        MEdelegh => Err(make_illegal(sys)), // S-mode not supported
        // Machine trap handling
        MScratch => Ok(read_mscratch(sys)),
        MEpc => Ok(read_mepc(sys)),
        MCause => Ok(read_mcause(sys)),
        MTval => Ok(read_mtval(sys)),
        MIp => Ok(0), // No interrupt exists
        // Machine configuration
        MEnvCfg => Ok(read_menvcfg(sys)),
        MEnvCfgh => Ok(0),
        // Machine memory protection (not supported)
        PmpCfg(_) => Ok(0),
        PmpAddr(_) => Ok(0),
        // Machine counter/timer
        MCycle => Ok(read_mcycle(sys)),
        MInstRet => Ok(read_minstret(sys)),
        MHpmCounter(_) => Ok(0),
        MCycleh => Ok(read_mcycleh(sys)),
        MInstReth => Ok(read_minstreth(sys)),
        MHpmCounterh(_) => Ok(0),
        // Machine counter setup
        MCountInhibit => Ok(read_mcountinhibit(sys)),
        MHpmEvent(_) => Ok(0),
        MHpmEventh(_) => Ok(0),
    }
}

pub fn csr_write_m(sys: &mut System, csr: &CsrRegM, val: u32) -> Result {
    match csr {
        // Machine information (read only)
        MVendorId => Err(make_illegal(sys)),
        MArchId => Err(make_illegal(sys)),
        MImpId => Err(make_illegal(sys)),
        MHartId => Err(make_illegal(sys)),
        MConfigPtr => Err(make_illegal(sys)),
        // Machine trap setup
        MStatus => Ok(write_mstatus(sys, val)),
        MIsa => Ok(()),
        MEdeleg => Err(make_illegal(sys)), // S-mode not supported
        MIdeleg => Err(make_illegal(sys)), // S-mode not supported
        MIe => Ok(()),                     // No interrupt exists
        MTvec => Ok(write_mtvec(sys, val)),
        MCounterEn => Ok(write_mcounteren(sys, val)),
        MStatush => Ok(()),
        MEdelegh => Err(make_illegal(sys)), // S-mode not supported
        // Machine trap handling
        MScratch => Ok(write_mscratch(sys, val)),
        MEpc => Ok(write_mepc(sys, val)),
        MCause => write_mcause(sys, val),
        MTval => Ok(write_mtval(sys, val)),
        MIp => Ok(()), // No interrupt exists
        // Machine configuration
        MEnvCfg => Ok(write_menvcfg(sys, val)),
        MEnvCfgh => Ok(()),
        // Machine memory protection (not supported)
        PmpCfg(_) => Ok(()),
        PmpAddr(_) => Ok(()),
        // Machine counter/timer
        MCycle => Ok(write_mcycle(sys, val)),
        MInstRet => Ok(write_minstret(sys, val)),
        MHpmCounter(_) => Ok(()),
        MCycleh => Ok(write_mcycleh(sys, val)),
        MInstReth => Ok(write_minstreth(sys, val)),
        MHpmCounterh(_) => Ok(()),
        // Machine counter setup
        MCountInhibit => Ok(write_mcountinhibit(sys, val)),
        MHpmEvent(_) => Ok(()),
        MHpmEventh(_) => Ok(()),
    }
}

// ----------------- MSTATUS --------------------
fn read_mstatus(sys: &System) -> u32 {
    let Control {
        mie,
        mpie,
        mpp,
        mprv,
        tw,
        ..
    } = &sys.ctrl;
    let mpp = mpp.to_int();
    (*mie as u32) << 3
        | (*mpie as u32) << 7
        | (mpp as u32) << 11
        | (*mprv as u32) << 17
        | (*tw as u32) << 21
}

fn write_mstatus(sys: &mut System, val: u32) {
    sys.ctrl.mie = (val & (1 << 3)) != 0;
    sys.ctrl.mpie = (val & (1 << 7)) != 0;
    sys.ctrl.mprv = (val & (1 << 17)) != 0;
    sys.ctrl.tw = (val & (1 << 21)) != 0;

    if let Some(mpp) = MPriv::from((val >> 11) & 0b11) {
        sys.ctrl.mpp = mpp;
    }
}

// ------------------ MTVEC ---------------------
fn read_mtvec(sys: &System) -> u32 {
    let Control {
        mtvec_base,
        mtvec_mode,
        ..
    } = &sys.ctrl;
    mtvec_base | mtvec_mode.to_int()
}

fn write_mtvec(sys: &mut System, val: u32) {
    sys.ctrl.mtvec_base = val & 0xffff_fffc;
    if let Some(mtvec_mode) = MTvecMode::from(val & 0b11) {
        sys.ctrl.mtvec_mode = mtvec_mode;
    }
}

// ---------------- MCOUNTEREN ------------------
fn read_mcounteren(sys: &System) -> u32 {
    let Control {
        mcycle_en,
        minstret_en,
        ..
    } = &sys.ctrl;
    (*mcycle_en as u32) | (*minstret_en as u32) << 2
}

fn write_mcounteren(sys: &mut System, val: u32) {
    sys.ctrl.mcycle_en = (val & 1) != 0;
    sys.ctrl.minstret_en = (val & (1 << 2)) != 0;
}

// ----------------- MSCRATCH -------------------
fn read_mscratch(sys: &System) -> u32 {
    sys.ctrl.mscratch
}

fn write_mscratch(sys: &mut System, val: u32) {
    sys.ctrl.mscratch = val;
}

// ------------------- MEPC ---------------------
fn read_mepc(sys: &System) -> u32 {
    sys.ctrl.mepc & 0xffff_fffc
}

fn write_mepc(sys: &mut System, val: u32) {
    sys.ctrl.mepc = val & 0xffff_fffc;
}

// ------------------ MCAUSE --------------------
fn read_mcause(sys: &System) -> u32 {
    sys.ctrl.mtrap.cause.to_int()
}

fn write_mcause(sys: &mut System, val: u32) -> Result {
    if let Some(mcause) = TrapCause::from(val) {
        sys.ctrl.mtrap.cause = mcause;
        Ok(())
    } else {
        Err(make_illegal(sys))
    }
}

// ------------------ MTVAL ---------------------
fn read_mtval(sys: &System) -> u32 {
    sys.ctrl.mtrap.val
}

fn write_mtval(sys: &mut System, val: u32) {
    sys.ctrl.mtrap.val = val;
}

// ----------------- MENVCFG --------------------
fn read_menvcfg(sys: &System) -> u32 {
    sys.ctrl.fiom as u32
}

fn write_menvcfg(sys: &mut System, val: u32) {
    sys.ctrl.fiom = (val & 1) != 0;
}

// ----------------- MCYCLE ---------------------
pub fn read_mcycle(sys: &System) -> u32 {
    sys.ctrl.mcycle as u32
}

pub fn read_mcycleh(sys: &System) -> u32 {
    (sys.ctrl.mcycle >> 32) as u32
}

fn write_mcycle(sys: &mut System, val: u32) {
    sys.ctrl.mcycle &= 0xffff_ffff_0000_0000;
    sys.ctrl.mcycle |= val as u64;
}

fn write_mcycleh(sys: &mut System, val: u32) {
    sys.ctrl.mcycle &= 0x0000_0000_ffff_ffff;
    sys.ctrl.mcycle |= (val as u64) << 32;
}

// ---------------- MINSTRET --------------------
pub fn read_minstret(sys: &System) -> u32 {
    sys.ctrl.minstret as u32
}

pub fn read_minstreth(sys: &System) -> u32 {
    (sys.ctrl.minstret >> 32) as u32
}

fn write_minstret(sys: &mut System, val: u32) {
    sys.ctrl.minstret &= 0xffff_ffff_0000_0000;
    sys.ctrl.minstret |= val as u64;
}

fn write_minstreth(sys: &mut System, val: u32) {
    sys.ctrl.minstret &= 0x0000_0000_ffff_ffff;
    sys.ctrl.minstret |= (val as u64) << 32;
}

// -------------- MCOUNTINHIBIT -----------------
fn read_mcountinhibit(sys: &System) -> u32 {
    let Control {
        mcycle_inhibit,
        minstret_inhibit,
        ..
    } = &sys.ctrl;
    (*mcycle_inhibit as u32) | (*minstret_inhibit as u32) << 2
}

fn write_mcountinhibit(sys: &mut System, val: u32) {
    sys.ctrl.mcycle_inhibit = (val & 1) != 0;
    sys.ctrl.minstret_inhibit = (val & (1 << 2)) != 0;
}
