#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Exception {
    InstrAddrMisaligned,
    InstrAccessFault,
    IllegalInstr,
    Breakpoint,
    LoadAddrMisaligned,
    LoadAccessFault,
    StoreAddrMisaligned,
    StoreAccessFault,
    EcallFromU,
    EcallFromS,
    EcallFromM,
    InstrPageFault,
    LoadPageFault,
    StorePageFault,
    SoftwareCheck,
    HardwareError,
}

impl Exception {
    pub fn from(code: u32) -> Option<Exception> {
        match code {
            0 => Some(Exception::InstrAddrMisaligned),
            1 => Some(Exception::InstrAccessFault),
            2 => Some(Exception::IllegalInstr),
            3 => Some(Exception::Breakpoint),
            4 => Some(Exception::LoadAddrMisaligned),
            5 => Some(Exception::LoadAccessFault),
            6 => Some(Exception::StoreAddrMisaligned),
            7 => Some(Exception::StoreAccessFault),
            8 => Some(Exception::EcallFromU),
            9 => Some(Exception::EcallFromS),
            11 => Some(Exception::EcallFromM),
            12 => Some(Exception::InstrPageFault),
            13 => Some(Exception::LoadPageFault),
            15 => Some(Exception::StorePageFault),
            18 => Some(Exception::SoftwareCheck),
            19 => Some(Exception::HardwareError),
            _ => None,
        }
    }

    pub fn to_int(&self) -> u32 {
        match self {
            Exception::InstrAddrMisaligned => 0,
            Exception::InstrAccessFault => 1,
            Exception::IllegalInstr => 2,
            Exception::Breakpoint => 3,
            Exception::LoadAddrMisaligned => 4,
            Exception::LoadAccessFault => 5,
            Exception::StoreAddrMisaligned => 6,
            Exception::StoreAccessFault => 7,
            Exception::EcallFromU => 8,
            Exception::EcallFromS => 9,
            Exception::EcallFromM => 11,
            Exception::InstrPageFault => 12,
            Exception::LoadPageFault => 13,
            Exception::StorePageFault => 15,
            Exception::SoftwareCheck => 18,
            Exception::HardwareError => 19,
        }
    }
}
