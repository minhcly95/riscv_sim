#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Trap {
    pub cause: TrapCause,
    pub val: u32,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TrapCause {
    Exception(Exception),
    Interrupt(Interrupt),
}

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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Interrupt {
    SSoft,
    MSoft,
    STimer,
    MTimer,
    SExt,
    MExt,
}

impl Trap {
    pub fn from_code(cause: u32, val: u32) -> Option<Trap> {
        let cause = TrapCause::from(cause)?;
        Some(Trap { cause, val })
    }

    pub fn from_exception(ex: Exception, val: u32) -> Trap {
        let cause = TrapCause::Exception(ex);
        Trap { cause, val }
    }

    pub fn from_interrupt(int: Interrupt, val: u32) -> Trap {
        let cause = TrapCause::Interrupt(int);
        Trap { cause, val }
    }
}

impl TrapCause {
    pub fn from(code: u32) -> Option<TrapCause> {
        if (code as i32) < 0 {
            Some(TrapCause::Interrupt(Interrupt::from(code & 0x7fff_ffff)?))
        } else {
            Some(TrapCause::Exception(Exception::from(code)?))
        }
    }

    pub fn to_int(&self) -> u32 {
        match self {
            TrapCause::Exception(e) => e.to_int(),
            TrapCause::Interrupt(i) => i.to_int() | 0x8000_0000,
        }
    }
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

impl Interrupt {
    pub fn from(code: u32) -> Option<Interrupt> {
        match code {
            1 => Some(Interrupt::SSoft),
            3 => Some(Interrupt::MSoft),
            5 => Some(Interrupt::STimer),
            7 => Some(Interrupt::MTimer),
            9 => Some(Interrupt::SExt),
            11 => Some(Interrupt::MExt),
            _ => None,
        }
    }

    pub fn to_int(&self) -> u32 {
        match self {
            Interrupt::SSoft => 1,
            Interrupt::MSoft => 3,
            Interrupt::STimer => 5,
            Interrupt::MTimer => 7,
            Interrupt::SExt => 9,
            Interrupt::MExt => 11,
        }
    }
}
