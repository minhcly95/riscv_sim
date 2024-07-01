#[derive(Debug, PartialEq, Eq)]
pub enum CsrReg {
    U(CsrRegU),
    S(CsrRegS),
    M(CsrRegM),
}

#[derive(Debug, PartialEq, Eq)]
pub enum CsrRegU {
    Cycle,
    Time,
    InstRet,
    HpmCounter(u8),
    Cycleh,
    Timeh,
    InstReth,
    HpmCounterh(u8),
}

#[derive(Debug, PartialEq, Eq)]
pub enum CsrRegS {
    // Supervisor trap setup
    SStatus,
    SIe,
    STvec,
    SCounterEn,
    // Supervisor configuration
    SEnvCfg,
    // Supervisor trap handling
    SScratch,
    SEpc,
    SCause,
    STval,
    SIp,
    // Supervisor protection and translation
    SAtp,
}

#[derive(Debug, PartialEq, Eq)]
pub enum CsrRegM {
    // Machine information
    MVendorId,
    MArchId,
    MImpId,
    MHartId,
    MConfigPtr,
    // Machine trap setup
    MStatus,
    MIsa,
    MEdeleg,
    MIdeleg,
    MIe,
    MTvec,
    MCounterEn,
    MStatush,
    MEdelegh,
    // Machine trap handling
    MScratch,
    MEpc,
    MCause,
    MTval,
    MIp,
    // Machine configuration
    MEnvCfg,
    MEnvCfgh,
    // Machine memory protection
    PmpCfg(u8),
    PmpAddr(u8),
    // Machine counter/timer
    MCycle,
    MInstRet,
    MHpmCounter(u8),
    MCycleh,
    MInstReth,
    MHpmCounterh(u8),
    // Machine counter setup
    MCountInhibit,
    MHpmEvent(u8),
    MHpmEventh(u8),
}

impl CsrReg {
    pub fn from(imm: i32) -> Option<CsrReg> {
        let imm = imm & 0xfff;
        match imm {
            // Unprivileged counter/timer
            0xc00 => Some(Self::U(CsrRegU::Cycle)),
            0xc01 => Some(Self::U(CsrRegU::Time)),
            0xc02 => Some(Self::U(CsrRegU::InstRet)),
            i @ 0xc03..=0xc1f => Some(Self::U(CsrRegU::HpmCounter((i - 0xc00) as u8))),
            0xc80 => Some(Self::U(CsrRegU::Cycleh)),
            0xc81 => Some(Self::U(CsrRegU::Timeh)),
            0xc82 => Some(Self::U(CsrRegU::InstReth)),
            i @ 0xc83..=0xc9f => Some(Self::U(CsrRegU::HpmCounterh((i - 0xc80) as u8))),
            // Supervisor trap setup
            0x100 => Some(Self::S(CsrRegS::SStatus)),
            0x104 => Some(Self::S(CsrRegS::SIe)),
            0x105 => Some(Self::S(CsrRegS::STvec)),
            0x106 => Some(Self::S(CsrRegS::SCounterEn)),
            // Supervisor configuration
            0x10a => Some(Self::S(CsrRegS::SEnvCfg)),
            // Supervisor trap handling
            0x140 => Some(Self::S(CsrRegS::SScratch)),
            0x141 => Some(Self::S(CsrRegS::SEpc)),
            0x142 => Some(Self::S(CsrRegS::SCause)),
            0x143 => Some(Self::S(CsrRegS::STval)),
            0x144 => Some(Self::S(CsrRegS::SIp)),
            // Supervisor protection and translation
            0x180 => Some(Self::S(CsrRegS::SAtp)),
            // Machine information
            0xf11 => Some(Self::M(CsrRegM::MVendorId)),
            0xf12 => Some(Self::M(CsrRegM::MArchId)),
            0xf13 => Some(Self::M(CsrRegM::MImpId)),
            0xf14 => Some(Self::M(CsrRegM::MHartId)),
            0xf15 => Some(Self::M(CsrRegM::MConfigPtr)),
            // Machine trap setup
            0x300 => Some(Self::M(CsrRegM::MStatus)),
            0x301 => Some(Self::M(CsrRegM::MIsa)),
            0x302 => Some(Self::M(CsrRegM::MEdeleg)),
            0x303 => Some(Self::M(CsrRegM::MIdeleg)),
            0x304 => Some(Self::M(CsrRegM::MIe)),
            0x305 => Some(Self::M(CsrRegM::MTvec)),
            0x306 => Some(Self::M(CsrRegM::MCounterEn)),
            0x310 => Some(Self::M(CsrRegM::MStatush)),
            0x312 => Some(Self::M(CsrRegM::MEdelegh)),
            // Machine trap handling
            0x340 => Some(Self::M(CsrRegM::MScratch)),
            0x341 => Some(Self::M(CsrRegM::MEpc)),
            0x342 => Some(Self::M(CsrRegM::MCause)),
            0x343 => Some(Self::M(CsrRegM::MTval)),
            0x344 => Some(Self::M(CsrRegM::MIp)),
            // Machine configuration
            0x30a => Some(Self::M(CsrRegM::MEnvCfg)),
            0x31a => Some(Self::M(CsrRegM::MEnvCfgh)),
            // Machine memory protection
            i @ 0x3a0..=0x3af => Some(Self::M(CsrRegM::PmpCfg((i - 0x3a0) as u8))),
            i @ 0x3b0..=0x3ef => Some(Self::M(CsrRegM::PmpAddr((i - 0x3b0) as u8))),
            // Machine counter/timer
            0xb00 => Some(Self::M(CsrRegM::MCycle)),
            0xb02 => Some(Self::M(CsrRegM::MInstRet)),
            i @ 0xb03..=0xb1f => Some(Self::M(CsrRegM::MHpmCounter((i - 0xb00) as u8))),
            0xb80 => Some(Self::M(CsrRegM::MCycleh)),
            0xb82 => Some(Self::M(CsrRegM::MInstReth)),
            i @ 0xb83..=0xb9f => Some(Self::M(CsrRegM::MHpmCounterh((i - 0xb80) as u8))),
            // Machine counter setup
            0x320 => Some(Self::M(CsrRegM::MCountInhibit)),
            i @ 0x323..=0x33f => Some(Self::M(CsrRegM::MHpmEvent((i - 0x320) as u8))),
            i @ 0x723..=0x73f => Some(Self::M(CsrRegM::MHpmEventh((i - 0x3720) as u8))),
            _ => None,
        }
    }
}
