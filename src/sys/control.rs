use crate::{Exception, Interrupt, Trap};

#[derive(Debug)]
pub struct Control {
    pub privilege: MPriv, // Current privilege mode
    // mstatus: Status
    pub mie: bool,  // M-mode interrupt enable
    pub mpie: bool, // M-mode previous interrupt enable
    pub mpp: MPriv, // M-mode previous privilege mode
    pub mprv: bool, // Modify privilege
    pub tvm: bool,  // Trap virtual memory
    pub tw: bool,   // Trap wait-for-interrupt
    pub tsr: bool,  // Trap SRET
    // mtvec: Trap vector
    pub mtvec_base: u32,      // Trap vector base address
    pub mtvec_mode: TvecMode, // Trap vector mode
    // medeleg: Exception delegation
    pub medeleg: ExceptionMap,
    // mideleg: Interrupt delegation
    pub mideleg: InterruptMap,
    // mip: Interrupt pending
    pub ip: InterruptMap,
    // mie: Interrupt enable
    pub ie: InterruptMap,
    // mscratch: Scratch register
    pub mscratch: u32,
    // mepc: Exception PC
    pub mepc: u32,
    // mcause & mtval: Trap cause and value
    pub mtrap: Trap,
    // menvcfg: Environment configuration
    pub mfiom: bool, // Fence IO implies memory
    // mcycle: Counter for clock cycles
    pub mcycle: u64,
    pub mcycle_en: bool,
    pub mcycle_inhibit: bool,
    // mtime: Counter for real-time (same as cycle)
    pub mtime_en: bool,
    // minstret: Counter for retired instructions
    pub minstret: u64,
    pub minstret_en: bool,
    pub minstret_inhibit: bool,
    // sstatus: Status
    pub sie: bool,  // S-mode interrupt enable
    pub spie: bool, // S-mode previous interrupt enable
    pub spp: SPriv, // S-mode previous privilege mode
    pub sum: bool,  // S-mode user memory access
    pub mxr: bool,  // Make executable read
    // stvec: Trap vector
    pub stvec_base: u32,      // Trap vector base address
    pub stvec_mode: TvecMode, // Trap vector mode
    // scycle: Counter for clock cycles
    pub scycle_en: bool,
    // stime: Counter for real-time (same as cycle)
    pub stime_en: bool,
    // sinstret: Counter for retired instructions
    pub sinstret_en: bool,
    // sscratch: Scratch register
    pub sscratch: u32,
    // sepc: Exception PC
    pub sepc: u32,
    // scause & stval: Trap cause and value
    pub strap: Trap,
    // senvcfg: Environment configuration
    pub sfiom: bool, // Fence IO implies memory
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum MPriv {
    U,
    S,
    M,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum SPriv {
    U,
    S,
}

#[derive(Debug, PartialEq, Eq)]
pub enum TvecMode {
    Direct,
    Vectored,
}

#[derive(Debug)]
pub struct InterruptMap(pub u32);

#[derive(Debug)]
pub struct ExceptionMap(pub u32);

impl Control {
    pub fn new() -> Control {
        Control {
            privilege: MPriv::M,
            mie: false,
            mpie: false,
            mpp: MPriv::U,
            mprv: false,
            tvm: false,
            tw: false,
            tsr: false,
            mtvec_base: 0x100,
            mtvec_mode: TvecMode::Direct,
            medeleg: ExceptionMap::new(),
            mideleg: InterruptMap::new(),
            ip: InterruptMap::new(),
            ie: InterruptMap::new(),
            mscratch: 0,
            mepc: 0,
            mtrap: Trap::from_exception(Exception::InstrAddrMisaligned, 0),
            mfiom: false,
            mcycle: 0,
            mcycle_en: false,
            mcycle_inhibit: false,
            mtime_en: false,
            minstret: 0,
            minstret_en: false,
            minstret_inhibit: false,
            sie: false,
            spie: false,
            spp: SPriv::U,
            sum: false,
            mxr: false,
            stvec_base: 0x100,
            stvec_mode: TvecMode::Direct,
            scycle_en: false,
            stime_en: false,
            sinstret_en: false,
            sscratch: 0,
            sepc: 0,
            strap: Trap::from_exception(Exception::InstrAddrMisaligned, 0),
            sfiom: false,
        }
    }
}

impl MPriv {
    pub fn from(code: u32) -> Option<MPriv> {
        match code {
            0b00 => Some(MPriv::U),
            0b01 => Some(MPriv::S),
            0b11 => Some(MPriv::M),
            _ => None,
        }
    }

    pub fn from_s(spriv: SPriv) -> MPriv {
        match spriv {
            SPriv::U => MPriv::U,
            SPriv::S => MPriv::S,
        }
    }

    pub fn to_int(&self) -> u32 {
        match self {
            MPriv::U => 0b00,
            MPriv::S => 0b01,
            MPriv::M => 0b11,
        }
    }
}

impl SPriv {
    pub fn from(code: u32) -> Option<SPriv> {
        match code {
            0b0 => Some(SPriv::U),
            0b1 => Some(SPriv::S),
            _ => None,
        }
    }

    pub fn from_m(mpriv: MPriv) -> Option<SPriv> {
        match mpriv {
            MPriv::U => Some(SPriv::U),
            MPriv::S => Some(SPriv::S),
            _ => None,
        }
    }

    pub fn to_int(&self) -> u32 {
        match self {
            SPriv::U => 0b0,
            SPriv::S => 0b1,
        }
    }
}

impl TvecMode {
    pub fn from(code: u32) -> Option<TvecMode> {
        match code {
            0b0 => Some(TvecMode::Direct),
            0b1 => Some(TvecMode::Vectored),
            _ => None,
        }
    }

    pub fn to_int(&self) -> u32 {
        match self {
            TvecMode::Direct => 0b0,
            TvecMode::Vectored => 0b1,
        }
    }
}

impl InterruptMap {
    pub fn new() -> InterruptMap {
        InterruptMap(0)
    }

    pub fn get(&self, int: &Interrupt) -> bool {
        let mask = 1 << int.to_int();
        self.0 & mask != 0
    }

    pub fn set(&mut self, int: &Interrupt, val: bool) {
        let mask = 1 << int.to_int();
        self.0 &= !mask;
        self.0 |= mask & (val as u32);
    }
}

impl ExceptionMap {
    pub fn new() -> ExceptionMap {
        ExceptionMap(0)
    }

    pub fn get(&self, ex: &Exception) -> bool {
        let mask = 1 << ex.to_int();
        self.0 & mask != 0
    }

    pub fn set(&mut self, ex: &Exception, val: bool) {
        let mask = 1 << ex.to_int();
        self.0 &= !mask;
        self.0 |= mask & (val as u32);
    }
}
