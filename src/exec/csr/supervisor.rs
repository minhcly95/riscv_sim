use super::{Result, Result32};
use crate::{
    instr::csr::{CsrRegS::*, *},
    sys::{control::*, make_illegal},
    trap::TrapCause,
    System,
};

pub fn csr_read_s(sys: &mut System, csr: &CsrRegS) -> Result32 {
    match csr {
        // Supervisor trap setup
        SStatus => Ok(read_sstatus(sys)),
        SIe => Ok(read_sie(sys)),
        STvec => Ok(read_stvec(sys)),
        SCounterEn => Ok(read_scounteren(sys)),
        // Supervisor configuration
        SEnvCfg => Ok(read_senvcfg(sys)),
        // Supervisor trap handling
        SScratch => Ok(read_sscratch(sys)),
        SEpc => Ok(read_sepc(sys)),
        SCause => Ok(read_scause(sys)),
        STval => Ok(read_stval(sys)),
        SIp => Ok(read_sip(sys)),
        // Supervisor protection and translation
        SAtp => read_satp(sys),
    }
}

pub fn csr_write_s(sys: &mut System, csr: &CsrRegS, val: u32) -> Result {
    match csr {
        // Supervisor trap setup
        SStatus => Ok(write_sstatus(sys, val)),
        SIe => Ok(write_sie(sys, val)),
        STvec => Ok(write_stvec(sys, val)),
        SCounterEn => Ok(write_scounteren(sys, val)),
        // Supervisor configuration
        SEnvCfg => Ok(write_senvcfg(sys, val)),
        // Supervisor trap handling
        SScratch => Ok(write_sscratch(sys, val)),
        SEpc => Ok(write_sepc(sys, val)),
        SCause => write_scause(sys, val),
        STval => Ok(write_stval(sys, val)),
        SIp => Ok(write_sip(sys, val)),
        // Supervisor protection and translation
        SAtp => write_satp(sys, val),
    }
}

// ----------------- SSTATUS --------------------
fn read_sstatus(sys: &System) -> u32 {
    let Control {
        sie,
        spie,
        spp,
        sum,
        mxr,
        ..
    } = &sys.ctrl;
    let spp = spp.to_int();
    (*sie as u32) << 1
        | (*spie as u32) << 5
        | (spp as u32) << 8
        | (*sum as u32) << 18
        | (*mxr as u32) << 19
}

fn write_sstatus(sys: &mut System, val: u32) {
    sys.ctrl.sie = (val & (1 << 1)) != 0;
    sys.ctrl.spie = (val & (1 << 5)) != 0;
    sys.ctrl.sum = (val & (1 << 18)) != 0;
    sys.ctrl.mxr = (val & (1 << 19)) != 0;

    if let Some(spp) = SPriv::from((val >> 8) & 0b1) {
        sys.ctrl.spp = spp;
    }
}

// ------------------ STVEC ---------------------
fn read_stvec(sys: &System) -> u32 {
    let Control {
        stvec_base,
        stvec_mode,
        ..
    } = &sys.ctrl;
    stvec_base | stvec_mode.to_int()
}

fn write_stvec(sys: &mut System, val: u32) {
    sys.ctrl.stvec_base = val & 0xffff_fffc;
    if let Some(stvec_mode) = TvecMode::from(val & 0b11) {
        sys.ctrl.stvec_mode = stvec_mode;
    }
}

// ------------------- SIP ----------------------
fn read_sip(sys: &System) -> u32 {
    sys.ctrl.ip.0 & 0x222
}

fn write_sip(sys: &mut System, val: u32) {
    // seip, stip are read-only
    sys.ctrl.ip.0 &= !0x2;
    sys.ctrl.ip.0 = val & 0x2
}

// ------------------- SIE ----------------------
fn read_sie(sys: &System) -> u32 {
    sys.ctrl.ie.0 & 0x222
}

fn write_sie(sys: &mut System, val: u32) {
    sys.ctrl.ie.0 &= !0x222;
    sys.ctrl.ie.0 |= val & 0x222;
}

// ---------------- SCOUNTEREN ------------------
fn read_scounteren(sys: &System) -> u32 {
    let Control {
        scycle_en,
        stime_en,
        sinstret_en,
        ..
    } = &sys.ctrl;
    (*scycle_en as u32) | (*stime_en as u32) << 1 | (*sinstret_en as u32) << 2
}

fn write_scounteren(sys: &mut System, val: u32) {
    sys.ctrl.scycle_en = (val & 1) != 0;
    sys.ctrl.stime_en = (val & (1 << 1)) != 0;
    sys.ctrl.sinstret_en = (val & (1 << 2)) != 0;
}

// ----------------- SSCRATCH -------------------
fn read_sscratch(sys: &System) -> u32 {
    sys.ctrl.sscratch
}

fn write_sscratch(sys: &mut System, val: u32) {
    sys.ctrl.sscratch = val;
}

// ------------------- SEPC ---------------------
fn read_sepc(sys: &System) -> u32 {
    sys.ctrl.sepc & 0xffff_fffc
}

fn write_sepc(sys: &mut System, val: u32) {
    sys.ctrl.sepc = val & 0xffff_fffc;
}

// ------------------ SCAUSE --------------------
fn read_scause(sys: &System) -> u32 {
    sys.ctrl.strap.cause.to_int()
}

fn write_scause(sys: &mut System, val: u32) -> Result {
    if let Some(scause) = TrapCause::from(val) {
        sys.ctrl.strap.cause = scause;
        Ok(())
    } else {
        Err(make_illegal(sys))
    }
}

// ------------------ STVAL ---------------------
fn read_stval(sys: &System) -> u32 {
    sys.ctrl.strap.val
}

fn write_stval(sys: &mut System, val: u32) {
    sys.ctrl.strap.val = val;
}

// ----------------- SENVCFG --------------------
fn read_senvcfg(sys: &System) -> u32 {
    sys.ctrl.sfiom as u32
}

fn write_senvcfg(sys: &mut System, val: u32) {
    sys.ctrl.sfiom = (val & 1) != 0;
}

// ------------------ SATP ----------------------
fn read_satp(sys: &System) -> Result32 {
    if sys.ctrl.tvm {
        Err(make_illegal(sys))?
    }
    Ok(0)
}

fn write_satp(sys: &mut System, _val: u32) -> Result {
    if sys.ctrl.tvm {
        Err(make_illegal(sys))?
    }
    Ok(())
}
