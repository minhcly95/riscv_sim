use crate::Exception;

#[derive(Debug)]
pub struct Control {
    pub privilege: MPriv, // Current privilege mode
    // mstatus: Status
    pub mie: bool,  // M-mode interrupt enable
    pub mpie: bool, // M-mode previous interrupt enable
    pub mpp: MPriv, // M-mode previous privilege mode
    pub mprv: bool, // Modify privilege
    pub tw: bool,   // Trap wait-for-interrupt
    // mtvec: Trap vector
    pub mtvec_base: u32,       // Trap vector base address
    pub mtvec_mode: MTvecMode, // Trap vector mode
    // mscratch: Scratch register
    pub mscratch: u32,
    // mepc: Exception PC
    pub mepc: u32,
    // mcause: Trap cause
    pub mcause: Trap,
    // mtval: Trap value
    pub mtval: u32,
    // menvcfg: Environment configuration
    pub fiom: bool, // Fence IO implies memory
    // mcycle: Counter for clock cycles
    pub mcycle: u64,
    pub mcycle_en: bool,
    pub mcycle_inhibit: bool,
    // minstret: Counter for retired instructions
    pub minstret: u64,
    pub minstret_en: bool,
    pub minstret_inhibit: bool,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum MPriv {
    U,
    M,
}

#[derive(Debug, PartialEq, Eq)]
pub enum MTvecMode {
    Direct,
    Vectored,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Trap {
    Exception(Exception),
}

impl Control {
    pub fn new() -> Control {
        Control {
            privilege: MPriv::M,
            mie: false,
            mpie: false,
            mpp: MPriv::U,
            mprv: false,
            tw: false,
            mtvec_base: 0x100,
            mtvec_mode: MTvecMode::Direct,
            mscratch: 0,
            mepc: 0,
            mcause: Trap::Exception(Exception::InstrAddrMisaligned),
            mtval: 0,
            fiom: false,
            mcycle: 0,
            mcycle_en: false,
            mcycle_inhibit: false,
            minstret: 0,
            minstret_en: false,
            minstret_inhibit: false,
        }
    }
}

impl MPriv {
    pub fn from(code: u32) -> Option<MPriv> {
        match code {
            0b00 => Some(MPriv::U),
            0b11 => Some(MPriv::M),
            _ => None,
        }
    }

    pub fn to_int(&self) -> u32 {
        match self {
            MPriv::U => 0b00,
            MPriv::M => 0b11,
        }
    }
}

impl MTvecMode {
    pub fn from(code: u32) -> Option<MTvecMode> {
        match code {
            0b0 => Some(MTvecMode::Direct),
            0b1 => Some(MTvecMode::Vectored),
            _ => None,
        }
    }

    pub fn to_int(&self) -> u32 {
        match self {
            MTvecMode::Direct => 0b0,
            MTvecMode::Vectored => 0b1,
        }
    }
}

impl Trap {
    pub fn from(code: u32) -> Option<Trap> {
        Exception::from(code).map(Trap::Exception)
    }

    pub fn to_int(&self) -> u32 {
        match self {
            Trap::Exception(e) => e.to_int(),
        }
    }
}
