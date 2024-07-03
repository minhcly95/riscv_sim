use crate::{
    sys::{
        control::{Control, MPriv, SatpMode},
        System,
    },
    Exception, Result64E,
};

const MASK_OFFSET: u32 = 0xfff;
const MASK_VPN: u32 = 0x3ff;

pub fn translate(sys: &mut System, addr: u32, access_type: AccessType) -> Result64E {
    let Control {
        satp_mode,
        satp_ppn,
        ..
    } = sys.ctrl;

    // Effective privilege (depends on MPRV)
    let eff_priv = effective_privilege(sys, access_type);

    // No translation in M-mode
    if eff_priv == MPriv::M || satp_mode == SatpMode::Bare {
        return Ok(addr as u64);
    }

    // Components
    let vpn = [(addr >> 12) & MASK_VPN, (addr >> 22) & MASK_VPN];
    let mut ppn = satp_ppn;

    // Two-level translation
    for level in [1, 0] {
        let pte_addr = ((ppn as u64) << 12) | (vpn[level] << 2) as u64;
        let pte =
            PageTableEntry::from(sys.mem.read_u32(pte_addr)?).ok_or(page_fault(access_type))?;

        if !pte.valid {
            return Err(page_fault(access_type));
        } else if pte.perm != Permission::NonLeaf {
            return process_page(sys, pte, pte_addr, addr, access_type, level != 0);
        }

        ppn = pte.ppn;
    }

    // Still no leaf, page table is faulty
    Err(page_fault(access_type))
}

pub fn process_page(
    sys: &mut System,
    mut pte: PageTableEntry,
    pte_addr: u64,
    addr: u32,
    access_type: AccessType,
    superpage: bool,
) -> Result64E {
    let Control { sum, mxr, .. } = sys.ctrl;

    // Check permission
    match access_type {
        AccessType::Instr => {
            if !pte.perm.can_exec() {
                return Err(page_fault(access_type));
            }
        }
        AccessType::Load => {
            // If MXR = 1, loads from X pages are allowed
            if !(pte.perm.can_read() || mxr && pte.perm.can_exec()) {
                return Err(page_fault(access_type));
            }
        }
        AccessType::Store => {
            if !pte.perm.can_write() {
                return Err(page_fault(access_type));
            }
        }
    }

    // Effective privilege (depends on MPRV)
    let eff_priv = effective_privilege(sys, access_type);

    // Check if the user is accessing a S-mode page
    if !pte.user && eff_priv == MPriv::U {
        return Err(page_fault(access_type));
    }

    // Check if the supervisor is accessing a U-mode page
    if !sum && pte.user && eff_priv == MPriv::S {
        return Err(page_fault(access_type));
    }

    // Check for misaligned superpage
    if superpage && (pte.ppn & MASK_VPN) != 0 {
        return Err(page_fault(access_type));
    }

    // Modify A/D bits
    if !pte.access || (access_type == AccessType::Store && !pte.dirty) {
        pte.access = true;
        if access_type == AccessType::Store {
            pte.dirty = true;
        }
        sys.mem.write_u32(pte_addr, pte.to_int())?;
    }

    // Translation is successful
    if superpage {
        let offset = addr & (MASK_OFFSET | (MASK_VPN << 12));
        Ok(((pte.ppn as u64) >> 10 << 22) | offset as u64)
    } else {
        let offset = addr & MASK_OFFSET;
        Ok(((pte.ppn as u64) << 12) | offset as u64)
    }
}

fn page_fault(access_type: AccessType) -> Exception {
    match access_type {
        AccessType::Instr => Exception::InstrPageFault,
        AccessType::Load => Exception::LoadPageFault,
        AccessType::Store => Exception::StorePageFault,
    }
}

fn effective_privilege(sys: &System, access_type: AccessType) -> MPriv {
    let Control {
        privilege,
        mpp,
        mprv,
        ..
    } = sys.ctrl;

    if mprv && access_type != AccessType::Instr {
        mpp
    } else {
        privilege
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum AccessType {
    Instr,
    Load,
    Store,
}

pub struct PageTableEntry {
    valid: bool,
    perm: Permission,
    user: bool,
    global: bool,
    access: bool,
    dirty: bool,
    rsw: u32,
    ppn: u32,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Permission {
    NonLeaf,
    R,
    RW,
    X,
    RX,
    RWX,
}

impl PageTableEntry {
    fn from(code: u32) -> Option<PageTableEntry> {
        let valid = (code & 1) != 0;
        let perm = Permission::from((code >> 1) & 0b111)?;
        let user = (code & (1 << 4)) != 0;
        let global = (code & (1 << 5)) != 0;
        let access = (code & (1 << 6)) != 0;
        let dirty = (code & (1 << 7)) != 0;
        let rsw = (code >> 8) & 0b11;
        let ppn = code >> 10;
        Some(PageTableEntry {
            valid,
            perm,
            user,
            global,
            access,
            dirty,
            rsw,
            ppn,
        })
    }

    fn to_int(&self) -> u32 {
        let PageTableEntry {
            valid,
            perm,
            user,
            global,
            access,
            dirty,
            rsw,
            ppn,
        } = self;
        (*valid as u32)
            | perm.to_int() << 1
            | (*user as u32) << 4
            | (*global as u32) << 5
            | (*access as u32) << 6
            | (*dirty as u32) << 7
            | rsw << 8
            | ppn << 10
    }
}

impl Permission {
    fn from(code: u32) -> Option<Permission> {
        match code {
            0b000 => Some(Self::NonLeaf),
            0b001 => Some(Self::R),
            0b011 => Some(Self::RW),
            0b100 => Some(Self::X),
            0b101 => Some(Self::RX),
            0b111 => Some(Self::RWX),
            _ => None,
        }
    }

    fn to_int(&self) -> u32 {
        match self {
            Permission::NonLeaf => 0b000,
            Permission::R => 0b001,
            Permission::RW => 0b011,
            Permission::X => 0b100,
            Permission::RX => 0b101,
            Permission::RWX => 0b111,
        }
    }

    fn can_read(&self) -> bool {
        match self {
            Permission::NonLeaf => false,
            Permission::R => true,
            Permission::RW => true,
            Permission::X => false,
            Permission::RX => true,
            Permission::RWX => true,
        }
    }

    fn can_write(&self) -> bool {
        match self {
            Permission::NonLeaf => false,
            Permission::R => false,
            Permission::RW => true,
            Permission::X => false,
            Permission::RX => false,
            Permission::RWX => true,
        }
    }

    fn can_exec(&self) -> bool {
        match self {
            Permission::NonLeaf => false,
            Permission::R => false,
            Permission::RW => false,
            Permission::X => true,
            Permission::RX => true,
            Permission::RWX => true,
        }
    }
}

#[cfg(test)]
mod tests {
    use bytesize::ByteSize;
    use super::*;
    use crate::{Config, Exception::*};

    const RAM_BASE: u64 = 0x00400_000;

    // We create a RAM with these 5 pages:
    // S-mode data page (RAM_BASE)
    // U-mode data page (RAM_BASE + 0x1_000)
    // Root page table (RAM_BASE + 0x3_000)
    // Sub page table containing S-mode page (RAM_BASE + 0x7_000)
    // Sub page table containing U-mode page  (RAM_BASE + 0xa_000)
    const S_PAGE_PA: u64 = RAM_BASE;
    const U_PAGE_PA: u64 = RAM_BASE + 0x1_000;
    const PT_ROOT_PA: u64 = RAM_BASE + 0x3_000;
    const PT_SUB_S_PA: u64 = RAM_BASE + 0x7_000;
    const PT_SUB_U_PA: u64 = RAM_BASE + 0xa_000;

    // Virtual addresses of the data pages
    const S_PAGE_VA: u32 = 0xbcfec_000;
    const U_PAGE_VA: u32 = 0x51290_000;

    // Components
    const S_VPN0: u32 = (S_PAGE_VA >> 12) & MASK_VPN;
    const S_VPN1: u32 = (S_PAGE_VA >> 22) & MASK_VPN;
    const U_VPN0: u32 = (U_PAGE_VA >> 12) & MASK_VPN;
    const U_VPN1: u32 = (U_PAGE_VA >> 22) & MASK_VPN;

    fn make_sys() -> System {
        let mut sys = System::from_config(Config {
            binary: None,
            size: ByteSize::b(0x10000), // 16kB
            base: RAM_BASE as u32,
            verbose: true,
        });

        // Enable paging
        sys.ctrl.privilege = MPriv::S;
        sys.ctrl.satp_mode = SatpMode::Sv32;
        sys.ctrl.satp_ppn = 0x00403;

        // Add page table entries
        // Root page table
        sys.mem
            .write_u32(
                PT_ROOT_PA | (S_VPN1 << 2) as u64,
                PageTableEntry {
                    valid: true,
                    perm: Permission::NonLeaf,
                    user: false,
                    global: false,
                    access: false,
                    dirty: false,
                    rsw: 0,
                    ppn: (PT_SUB_S_PA >> 12) as u32,
                }
                .to_int(),
            )
            .unwrap();
        sys.mem
            .write_u32(
                PT_ROOT_PA | (U_VPN1 << 2) as u64,
                PageTableEntry {
                    valid: true,
                    perm: Permission::NonLeaf,
                    user: false,
                    global: false,
                    access: false,
                    dirty: false,
                    rsw: 0,
                    ppn: (PT_SUB_U_PA >> 12) as u32,
                }
                .to_int(),
            )
            .unwrap();

        // Sub page table for S page
        sys.mem
            .write_u32(
                PT_SUB_S_PA | (S_VPN0 << 2) as u64,
                PageTableEntry {
                    valid: true,
                    perm: Permission::RWX,
                    user: false,
                    global: false,
                    access: false,
                    dirty: false,
                    rsw: 0,
                    ppn: (S_PAGE_PA >> 12) as u32,
                }
                .to_int(),
            )
            .unwrap();

        // Sub page table for U page
        sys.mem
            .write_u32(
                PT_SUB_U_PA | (U_VPN0 << 2) as u64,
                PageTableEntry {
                    valid: true,
                    perm: Permission::RWX,
                    user: true,
                    global: false,
                    access: false,
                    dirty: false,
                    rsw: 0,
                    ppn: (U_PAGE_PA >> 12) as u32,
                }
                .to_int(),
            )
            .unwrap();

        sys
    }

    #[test]
    #[rustfmt::skip]
    fn test_translate() {
        let mut sys = make_sys();

        sys.ctrl.privilege = MPriv::S;
        assert_eq!(translate(&mut sys, S_PAGE_VA | 0x832, AccessType::Load).unwrap(), S_PAGE_PA | 0x832);
        assert_eq!(translate(&mut sys, S_PAGE_VA | 0xce3, AccessType::Store).unwrap(), S_PAGE_PA | 0xce3);
        assert_eq!(translate(&mut sys, S_PAGE_VA | 0x515, AccessType::Instr).unwrap(), S_PAGE_PA | 0x515);

        sys.ctrl.privilege = MPriv::U;
        assert_eq!(translate(&mut sys, U_PAGE_VA | 0x832, AccessType::Load).unwrap(), U_PAGE_PA | 0x832);
        assert_eq!(translate(&mut sys, U_PAGE_VA | 0xce3, AccessType::Store).unwrap(), U_PAGE_PA | 0xce3);
        assert_eq!(translate(&mut sys, U_PAGE_VA | 0x515, AccessType::Instr).unwrap(), U_PAGE_PA | 0x515);
    }

    #[test]
    #[rustfmt::skip]
    fn test_translate_cross_priv() {
        let mut sys = make_sys();

        sys.ctrl.privilege = MPriv::S;
        assert_eq!(translate(&mut sys, U_PAGE_VA | 0x832, AccessType::Load), Err(LoadPageFault));
        assert_eq!(translate(&mut sys, U_PAGE_VA | 0xce3, AccessType::Store), Err(StorePageFault));
        assert_eq!(translate(&mut sys, U_PAGE_VA | 0x515, AccessType::Instr), Err(InstrPageFault));

        sys.ctrl.privilege = MPriv::U;
        assert_eq!(translate(&mut sys, S_PAGE_VA | 0x832, AccessType::Load), Err(LoadPageFault));
        assert_eq!(translate(&mut sys, S_PAGE_VA | 0xce3, AccessType::Store), Err(StorePageFault));
        assert_eq!(translate(&mut sys, S_PAGE_VA | 0x515, AccessType::Instr), Err(InstrPageFault));
    }

    #[test]
    #[rustfmt::skip]
    fn test_translate_sum() {
        let mut sys = make_sys();
        sys.ctrl.sum = true;

        sys.ctrl.privilege = MPriv::S;
        assert_eq!(translate(&mut sys, U_PAGE_VA | 0x832, AccessType::Load).unwrap(), U_PAGE_PA | 0x832);
        assert_eq!(translate(&mut sys, U_PAGE_VA | 0xce3, AccessType::Store).unwrap(), U_PAGE_PA | 0xce3);
        assert_eq!(translate(&mut sys, U_PAGE_VA | 0x515, AccessType::Instr).unwrap(), U_PAGE_PA | 0x515);

        sys.ctrl.privilege = MPriv::U;
        assert_eq!(translate(&mut sys, S_PAGE_VA | 0x832, AccessType::Load), Err(LoadPageFault));
        assert_eq!(translate(&mut sys, S_PAGE_VA | 0xce3, AccessType::Store), Err(StorePageFault));
        assert_eq!(translate(&mut sys, S_PAGE_VA | 0x515, AccessType::Instr), Err(InstrPageFault));
    }

    #[test]
    #[rustfmt::skip]
    fn test_translate_m_mode() {
        let mut sys = make_sys();
        sys.ctrl.privilege = MPriv::M;

        // M-mode does not translate the addresses
        assert_eq!(translate(&mut sys, S_PAGE_VA | 0x832, AccessType::Load).unwrap(), S_PAGE_VA as u64 | 0x832);
        assert_eq!(translate(&mut sys, S_PAGE_VA | 0xce3, AccessType::Store).unwrap(), S_PAGE_VA as u64 | 0xce3);
        assert_eq!(translate(&mut sys, S_PAGE_VA | 0x515, AccessType::Instr).unwrap(), S_PAGE_VA as u64 | 0x515);

        assert_eq!(translate(&mut sys, U_PAGE_VA | 0x832, AccessType::Load).unwrap(), U_PAGE_VA as u64 | 0x832);
        assert_eq!(translate(&mut sys, U_PAGE_VA | 0xce3, AccessType::Store).unwrap(), U_PAGE_VA as u64 | 0xce3);
        assert_eq!(translate(&mut sys, U_PAGE_VA | 0x515, AccessType::Instr).unwrap(), U_PAGE_VA as u64 | 0x515);
    }

    #[test]
    #[rustfmt::skip]
    fn test_translate_mprv() {
        let mut sys = make_sys();
        sys.ctrl.privilege = MPriv::M;
        sys.ctrl.mprv = true;

        // Translate the addresses as S-mode
        sys.ctrl.mpp = MPriv::S;
        assert_eq!(translate(&mut sys, S_PAGE_VA | 0x832, AccessType::Load).unwrap(), S_PAGE_PA | 0x832);
        assert_eq!(translate(&mut sys, S_PAGE_VA | 0xce3, AccessType::Store).unwrap(), S_PAGE_PA | 0xce3);
        // MPRV does not apply to instruction fetch
        assert_eq!(translate(&mut sys, S_PAGE_VA | 0x515, AccessType::Instr).unwrap(), S_PAGE_VA as u64 | 0x515);

        assert_eq!(translate(&mut sys, U_PAGE_VA | 0x832, AccessType::Load), Err(LoadPageFault));
        assert_eq!(translate(&mut sys, U_PAGE_VA | 0xce3, AccessType::Store), Err(StorePageFault));
        // MPRV does not apply to instruction fetch
        assert_eq!(translate(&mut sys, U_PAGE_VA | 0x515, AccessType::Instr).unwrap(), U_PAGE_VA as u64 | 0x515);

        // Translate the addresses as U-mode
        sys.ctrl.mpp = MPriv::U;
        assert_eq!(translate(&mut sys, S_PAGE_VA | 0x832, AccessType::Load), Err(LoadPageFault));
        assert_eq!(translate(&mut sys, S_PAGE_VA | 0xce3, AccessType::Store), Err(StorePageFault));
        // MPRV does not apply to instruction fetch
        assert_eq!(translate(&mut sys, S_PAGE_VA | 0x515, AccessType::Instr).unwrap(), S_PAGE_VA as u64 | 0x515);

        assert_eq!(translate(&mut sys, U_PAGE_VA | 0x832, AccessType::Load).unwrap(), U_PAGE_PA | 0x832);
        assert_eq!(translate(&mut sys, U_PAGE_VA | 0xce3, AccessType::Store).unwrap(), U_PAGE_PA | 0xce3);
        // MPRV does not apply to instruction fetch
        assert_eq!(translate(&mut sys, U_PAGE_VA | 0x515, AccessType::Instr).unwrap(), U_PAGE_VA as u64 | 0x515);
    }

    #[test]
    #[rustfmt::skip]
    fn test_translate_mprv_sum() {
        let mut sys = make_sys();
        sys.ctrl.privilege = MPriv::M;
        sys.ctrl.mprv = true;
        sys.ctrl.sum = true;

        sys.ctrl.mpp = MPriv::S;
        assert_eq!(translate(&mut sys, U_PAGE_VA | 0x832, AccessType::Load).unwrap(), U_PAGE_PA | 0x832);
        assert_eq!(translate(&mut sys, U_PAGE_VA | 0xce3, AccessType::Store).unwrap(), U_PAGE_PA | 0xce3);
        // MPRV does not apply to instruction fetch
        assert_eq!(translate(&mut sys, U_PAGE_VA | 0x515, AccessType::Instr).unwrap(), U_PAGE_VA as u64 | 0x515);

        sys.ctrl.mpp = MPriv::U;
        assert_eq!(translate(&mut sys, S_PAGE_VA | 0x832, AccessType::Load), Err(LoadPageFault));
        assert_eq!(translate(&mut sys, S_PAGE_VA | 0xce3, AccessType::Store), Err(StorePageFault));
        // MPRV does not apply to instruction fetch
        assert_eq!(translate(&mut sys, U_PAGE_VA | 0x515, AccessType::Instr).unwrap(), U_PAGE_VA as u64 | 0x515);
    }
}
