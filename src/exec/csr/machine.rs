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
const MISA_EXT_S: u32 = 1 << 18;
const MISA: u32 = MISA_MXL_32 | MISA_EXT_I | MISA_EXT_M | MISA_EXT_A | MISA_EXT_U | MISA_EXT_S;

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
        MEdeleg => Ok(read_medeleg(sys)),
        MIdeleg => Ok(read_mideleg(sys)),
        MIe => Ok(read_mie(sys)),
        MTvec => Ok(read_mtvec(sys)),
        MCounterEn => Ok(read_mcounteren(sys)),
        MStatush => Ok(0),
        MEdelegh => Ok(0),
        // Machine trap handling
        MScratch => Ok(read_mscratch(sys)),
        MEpc => Ok(read_mepc(sys)),
        MCause => Ok(read_mcause(sys)),
        MTval => Ok(read_mtval(sys)),
        MIp => Ok(read_mip(sys)),
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
        MEdeleg => Ok(write_medeleg(sys, val)),
        MIdeleg => Ok(write_mideleg(sys, val)),
        MIe => Ok(write_mie(sys, val)),
        MTvec => Ok(write_mtvec(sys, val)),
        MCounterEn => Ok(write_mcounteren(sys, val)),
        MStatush => Ok(()),
        MEdelegh => Ok(()),
        // Machine trap handling
        MScratch => Ok(write_mscratch(sys, val)),
        MEpc => Ok(write_mepc(sys, val)),
        MCause => write_mcause(sys, val),
        MTval => Ok(write_mtval(sys, val)),
        MIp => Ok(write_mip(sys, val)),
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
        sie,
        mie,
        spie,
        mpie,
        spp,
        mpp,
        mprv,
        sum,
        mxr,
        tvm,
        tw,
        tsr,
        ..
    } = &sys.ctrl;
    let mpp = mpp.to_int();
    let spp = spp.to_int();
    (*sie as u32) << 1
        | (*mie as u32) << 3
        | (*spie as u32) << 5
        | (*mpie as u32) << 7
        | (spp as u32) << 8
        | (mpp as u32) << 11
        | (*mprv as u32) << 17
        | (*sum as u32) << 18
        | (*mxr as u32) << 19
        | (*tvm as u32) << 20
        | (*tw as u32) << 21
        | (*tsr as u32) << 22
}

fn write_mstatus(sys: &mut System, val: u32) {
    sys.ctrl.sie = (val & (1 << 1)) != 0;
    sys.ctrl.mie = (val & (1 << 3)) != 0;
    sys.ctrl.spie = (val & (1 << 5)) != 0;
    sys.ctrl.mpie = (val & (1 << 7)) != 0;
    sys.ctrl.mprv = (val & (1 << 17)) != 0;
    sys.ctrl.sum = (val & (1 << 18)) != 0;
    sys.ctrl.mxr = (val & (1 << 19)) != 0;
    sys.ctrl.tvm = (val & (1 << 20)) != 0;
    sys.ctrl.tw = (val & (1 << 21)) != 0;
    sys.ctrl.tsr = (val & (1 << 22)) != 0;

    if let Some(mpp) = MPriv::from((val >> 11) & 0b11) {
        sys.ctrl.mpp = mpp;
    }
    if let Some(spp) = SPriv::from((val >> 8) & 0b1) {
        sys.ctrl.spp = spp;
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
    if let Some(mtvec_mode) = TvecMode::from(val & 0b11) {
        sys.ctrl.mtvec_mode = mtvec_mode;
    }
}

// ----------------- MEDELEG --------------------
const EDELEG_MASK: u32 = 0xcbeff; // All except for M-mode ecall

fn read_medeleg(sys: &System) -> u32 {
    sys.ctrl.medeleg.0 & EDELEG_MASK
}

fn write_medeleg(sys: &mut System, val: u32) {
    // meip, mtip, msip are read-only
    sys.ctrl.medeleg.0 &= !EDELEG_MASK;
    sys.ctrl.medeleg.0 |= val & EDELEG_MASK;
}

// ----------------- MIDELEG --------------------
const IDELEG_MASK: u32 = 0x222; // All S-mode interrupta

fn read_mideleg(sys: &System) -> u32 {
    sys.ctrl.mideleg.0 & IDELEG_MASK
}

fn write_mideleg(sys: &mut System, val: u32) {
    // meip, mtip, msip are read-only
    sys.ctrl.mideleg.0 &= !IDELEG_MASK;
    sys.ctrl.mideleg.0 |= val & IDELEG_MASK;
}

// ------------------- MIP ----------------------
fn read_mip(sys: &System) -> u32 {
    sys.ctrl.ip.0 & 0xaaa
}

fn write_mip(sys: &mut System, val: u32) {
    // meip, mtip, msip are read-only
    sys.ctrl.ip.0 &= !0x222;
    sys.ctrl.ip.0 |= val & 0x222;
}

// ------------------- MIE ----------------------
fn read_mie(sys: &System) -> u32 {
    sys.ctrl.ie.0 & 0xaaa
}

fn write_mie(sys: &mut System, val: u32) {
    sys.ctrl.ie.0 &= !0xaaa;
    sys.ctrl.ie.0 = val & 0xaaa;
}

// ---------------- MCOUNTEREN ------------------
fn read_mcounteren(sys: &System) -> u32 {
    let Control {
        mcycle_en,
        mtime_en,
        minstret_en,
        ..
    } = &sys.ctrl;
    (*mcycle_en as u32) | (*mtime_en as u32) << 1 | (*minstret_en as u32) << 2
}

fn write_mcounteren(sys: &mut System, val: u32) {
    sys.ctrl.mcycle_en = (val & 1) != 0;
    sys.ctrl.mtime_en = (val & (1 << 1)) != 0;
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
    sys.ctrl.mfiom as u32
}

fn write_menvcfg(sys: &mut System, val: u32) {
    sys.ctrl.mfiom = (val & 1) != 0;
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
