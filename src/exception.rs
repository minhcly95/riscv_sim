#[derive(Debug, PartialEq, Eq)]
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
