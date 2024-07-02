use super::{advance_pc, Result};
use crate::{
    instr::{funct::*, reg::Reg},
    translate::*,
    Exception, System, Trap,
};
use core::panic;

pub fn execute_atomic(sys: &mut System, rd: &Reg, rs1: &Reg, rs2: &Reg, f: &AtomicFunct) -> Result {
    match f {
        AtomicFunct::LrSc(LrScFunct::Lr) => load_reserved(sys, rd, rs1)?,
        AtomicFunct::LrSc(LrScFunct::Sc) => store_conditional(sys, rd, rs1, rs2)?,
        AtomicFunct::Amo(af) => execute_amo(sys, rd, rs1, rs2, af)?,
    }
    advance_pc(sys);
    Ok(())
}

fn load_reserved(sys: &mut System, rd: &Reg, rs1: &Reg) -> Result {
    let vaddr = sys.reg(rs1) as u32;
    let make_trap = |ex| Trap::from_exception(ex, vaddr);
    // Translate virtual address
    let paddr = translate(sys, vaddr, AccessType::Load).map_err(make_trap)?;
    // Load and reserve data with physical address
    let data = sys.mem.read_u32(paddr).map_err(make_trap)? as i32;
    sys.mem.reserve(paddr);
    *sys.reg_mut(rd) = data;
    Ok(())
}

fn store_conditional(sys: &mut System, rd: &Reg, rs1: &Reg, rs2: &Reg) -> Result {
    let vaddr = sys.reg(rs1) as u32;
    let make_trap = |ex| Trap::from_exception(ex, vaddr);
    // Translate virtual address
    let paddr = translate(sys, vaddr, AccessType::Store).map_err(make_trap)?;
    // Check and store data with physical address
    if sys.mem.is_reserved(paddr) {
        // Only write when reservation is still valid
        let data = sys.reg(rs2);
        sys.mem.write_u32(paddr, data as u32).map_err(make_trap)?;
        *sys.reg_mut(rd) = 0;
    } else {
        // Still generate exceptions for faulty accesses
        sys.mem.check_write_u32(paddr).map_err(make_trap)?;
        *sys.reg_mut(rd) = 1;
    }
    // Invalidate any reservation
    sys.mem.clear_reservation();
    Ok(())
}

fn execute_amo(sys: &mut System, rd: &Reg, rs1: &Reg, rs2: &Reg, f: &AmoFunct) -> Result {
    // Always read rs1 and rs2 before writing rd
    let rs2 = sys.reg(rs2);
    let vaddr = sys.reg(rs1) as u32;
    let make_trap = |ex| Trap::from_exception(ex, vaddr);
    // Translate virtual address
    let paddr = translate(sys, vaddr, AccessType::Store).map_err(make_trap)?;

    // Read the data
    let data = sys
        .mem
        .read_u32(paddr)
        .map_err(|ex| {
            // An exception raised from an AMO counts as a store exception
            match ex {
                Exception::LoadAddrMisaligned => Exception::StoreAddrMisaligned,
                Exception::LoadAccessFault => Exception::StoreAccessFault,
                _ => panic!("Unexpected exception when calling Memory::read_u32"),
            }
        })
        .map_err(make_trap)? as i32;

    // Modify the data as store back at addr
    let new_data: i32;
    match f {
        AmoFunct::Add => new_data = data.wrapping_add(rs2),
        AmoFunct::Swap => new_data = rs2,
        AmoFunct::Xor => new_data = data ^ rs2,
        AmoFunct::Or => new_data = data | rs2,
        AmoFunct::And => new_data = data & rs2,
        AmoFunct::Min => new_data = data.min(rs2),
        AmoFunct::Max => new_data = data.max(rs2),
        AmoFunct::Minu => new_data = (data as u32).min(rs2 as u32) as i32,
        AmoFunct::Maxu => new_data = (data as u32).max(rs2 as u32) as i32,
    }
    sys.mem.write_u32(paddr, new_data as u32).map_err(make_trap)?;

    // Store original data to rd
    *sys.reg_mut(rd) = data;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{exec::store::execute_store, Trap};
    use Exception::*;

    const TEST_ADDR: u64 = 0x4;
    const OTHER_ADDR: u64 = 0x8;

    fn load_reserved(sys: &mut System, rd: u8, rs1: u8) -> Result {
        execute_atomic(
            sys,
            &Reg::new(rd),
            &Reg::new(rs1),
            &Reg::zero(),
            &AtomicFunct::LrSc(LrScFunct::Lr),
        )
    }

    fn store_conditional(sys: &mut System, rd: u8, rs1: u8, rs2: u8) -> Result {
        execute_atomic(
            sys,
            &Reg::new(rd),
            &Reg::new(rs1),
            &Reg::new(rs2),
            &AtomicFunct::LrSc(LrScFunct::Sc),
        )
    }

    fn store(sys: &mut System, rs1: u8, rs2: u8, imm: i32, f: StoreFunct) {
        execute_store(sys, &Reg::new(rs1), &Reg::new(rs2), imm, &f).unwrap();
    }

    fn assert_reg(sys: &System, r: u8, val: u32) {
        assert_eq!(sys.reg(&Reg::new(r)), val as i32);
    }

    fn sc_and_assert(sys: &mut System, expect: u32, success: bool) {
        store_conditional(sys, 4, 1, 2).unwrap();
        assert_eq!(sys.mem.read_u32(TEST_ADDR).unwrap(), expect);
        assert_reg(&sys, 4, if success { 0 } else { 1 });
        assert!(!sys.mem.is_reserved(TEST_ADDR));
    }

    fn assert_amo(
        sys: &mut System,
        rd: u8,
        rs1: u8,
        rs2: u8,
        f: AmoFunct,
        expect_rd: u32,
        expect_mem: u32,
    ) {
        execute_atomic(
            sys,
            &Reg::new(rd),
            &Reg::new(rs1),
            &Reg::new(rs2),
            &AtomicFunct::Amo(f),
        )
        .unwrap();
        assert_eq!(sys.reg(&Reg::new(rd)), expect_rd as i32);
        assert_eq!(
            sys.mem.read_u32(sys.reg(&Reg::new(rs1)) as u64).unwrap(),
            expect_mem
        );
    }

    fn assert_amo_failed(sys: &mut System, rd: u8, rs1: u8, rs2: u8, f: AmoFunct, ex: Exception) {
        let addr = sys.reg(&Reg::new(rs1)) as u32;
        assert_eq!(
            execute_atomic(
                sys,
                &Reg::new(rd),
                &Reg::new(rs1),
                &Reg::new(rs2),
                &AtomicFunct::Amo(f),
            ),
            Err(Trap::from_exception(ex, addr))
        );
    }

    #[test]
    fn test_lrsc_success() {
        let mut sys = System::new(16);
        sys.mem.write_u32(TEST_ADDR, 0xbcfec832).unwrap();
        *sys.reg_mut(&Reg::new(1)) = TEST_ADDR as i32;
        *sys.reg_mut(&Reg::new(2)) = 0x51290ce3_u32 as i32;

        // Load reserved
        load_reserved(&mut sys, 3, 1).unwrap();
        assert_reg(&sys, 3, 0xbcfec832);
        assert!(sys.mem.is_reserved(TEST_ADDR));
        // Store conditional (should succeed)
        sc_and_assert(&mut sys, 0x51290ce3, true);
        assert_eq!(sys.pc(), 8);
    }

    #[test]
    fn test_lrsc_other_addr() {
        let mut sys = System::new(16);
        sys.mem.write_u32(TEST_ADDR, 0xbcfec832).unwrap();
        sys.mem.write_u32(OTHER_ADDR, 0x942a44b1).unwrap();
        *sys.reg_mut(&Reg::new(1)) = TEST_ADDR as i32;
        *sys.reg_mut(&Reg::new(2)) = 0x51290ce3_u32 as i32;
        *sys.reg_mut(&Reg::new(6)) = OTHER_ADDR as i32;

        // Load reserved
        load_reserved(&mut sys, 3, 1).unwrap();
        assert_reg(&sys, 3, 0xbcfec832);
        assert!(sys.mem.is_reserved(TEST_ADDR));
        // Store conditional (other addr - should fail)
        store_conditional(&mut sys, 4, 6, 2).unwrap();
        assert_eq!(sys.mem.read_u32(TEST_ADDR).unwrap(), 0xbcfec832);
        assert_eq!(sys.mem.read_u32(OTHER_ADDR).unwrap(), 0x942a44b1);
        assert_reg(&sys, 4, 1);
        assert!(!sys.mem.is_reserved(TEST_ADDR));
        assert!(!sys.mem.is_reserved(OTHER_ADDR));

        assert_eq!(sys.pc(), 8);
    }

    #[test]
    fn test_lrsc_store_w() {
        let mut sys = System::new(16);
        sys.mem.write_u32(TEST_ADDR, 0xbcfec832).unwrap();
        *sys.reg_mut(&Reg::new(1)) = TEST_ADDR as i32;
        *sys.reg_mut(&Reg::new(2)) = 0x51290ce3_u32 as i32;
        *sys.reg_mut(&Reg::new(5)) = 0x942a44b1_u32 as i32;

        // Load reserved
        load_reserved(&mut sys, 3, 1).unwrap();
        // Store
        store(&mut sys, 0, 5, TEST_ADDR as i32, StoreFunct::W);
        // Store conditional (should fail)
        sc_and_assert(&mut sys, 0x942a44b1, false);
        assert_eq!(sys.pc(), 12);
    }

    #[test]
    fn test_lrsc_store_h() {
        let mut sys = System::new(16);
        sys.mem.write_u32(TEST_ADDR, 0xbcfec832).unwrap();
        *sys.reg_mut(&Reg::new(1)) = TEST_ADDR as i32;
        *sys.reg_mut(&Reg::new(2)) = 0x51290ce3_u32 as i32;
        *sys.reg_mut(&Reg::new(5)) = 0x942a44b1_u32 as i32;

        // Load reserved
        load_reserved(&mut sys, 3, 1).unwrap();
        // Store
        store(&mut sys, 0, 5, 2 + TEST_ADDR as i32, StoreFunct::H);
        // Store conditional (should fail)
        sc_and_assert(&mut sys, 0x44b1c832, false);
        assert_eq!(sys.pc(), 12);
    }

    #[test]
    fn test_lrsc_store_b() {
        let mut sys = System::new(16);
        sys.mem.write_u32(TEST_ADDR, 0xbcfec832).unwrap();
        *sys.reg_mut(&Reg::new(1)) = TEST_ADDR as i32;
        *sys.reg_mut(&Reg::new(2)) = 0x51290ce3_u32 as i32;
        *sys.reg_mut(&Reg::new(5)) = 0x942a44b1_u32 as i32;

        // Load reserved
        load_reserved(&mut sys, 3, 1).unwrap();
        // Store
        store(&mut sys, 0, 5, 3 + TEST_ADDR as i32, StoreFunct::B);
        // Store conditional (should fail)
        sc_and_assert(&mut sys, 0xb1fec832, false);
        assert_eq!(sys.pc(), 12);
    }

    #[test]
    fn test_lrsc_store_w_other_addr() {
        let mut sys = System::new(16);
        sys.mem.write_u32(TEST_ADDR, 0xbcfec832).unwrap();
        *sys.reg_mut(&Reg::new(1)) = TEST_ADDR as i32;
        *sys.reg_mut(&Reg::new(2)) = 0x51290ce3_u32 as i32;
        *sys.reg_mut(&Reg::new(5)) = 0x942a44b1_u32 as i32;

        // Load reserved
        load_reserved(&mut sys, 3, 1).unwrap();
        // Store (other addr)
        store(&mut sys, 0, 5, OTHER_ADDR as i32, StoreFunct::W);
        // Store conditional (should succeed)
        sc_and_assert(&mut sys, 0x51290ce3, true);
        assert_eq!(sys.pc(), 12);
    }

    #[test]
    fn test_lrsc_two_sc() {
        let mut sys = System::new(16);
        sys.mem.write_u32(TEST_ADDR, 0xbcfec832).unwrap();
        *sys.reg_mut(&Reg::new(1)) = TEST_ADDR as i32;
        *sys.reg_mut(&Reg::new(2)) = 0x51290ce3_u32 as i32;
        *sys.reg_mut(&Reg::new(5)) = 0x942a44b1_u32 as i32;

        // Load reserved
        load_reserved(&mut sys, 3, 1).unwrap();
        // Store conditional (should succeed)
        store_conditional(&mut sys, 4, 1, 5).unwrap();
        // Store conditional (should fail)
        sc_and_assert(&mut sys, 0x942a44b1, false);
        assert_eq!(sys.pc(), 12);
    }

    #[test]
    fn test_lrsc_two_sc_other_addr() {
        let mut sys = System::new(16);
        sys.mem.write_u32(TEST_ADDR, 0xbcfec832).unwrap();
        *sys.reg_mut(&Reg::new(1)) = TEST_ADDR as i32;
        *sys.reg_mut(&Reg::new(2)) = 0x51290ce3_u32 as i32;
        *sys.reg_mut(&Reg::new(5)) = 0x942a44b1_u32 as i32;
        *sys.reg_mut(&Reg::new(6)) = OTHER_ADDR as i32;

        // Load reserved
        load_reserved(&mut sys, 3, 1).unwrap();
        // Store conditional (other addr)
        store_conditional(&mut sys, 4, 6, 5).unwrap();
        // Store conditional (should fail)
        sc_and_assert(&mut sys, 0xbcfec832, false);
        assert_eq!(sys.pc(), 12);
    }

    #[test]
    fn test_lrsc_two_lr() {
        let mut sys = System::new(16);
        sys.mem.write_u32(TEST_ADDR, 0xbcfec832).unwrap();
        *sys.reg_mut(&Reg::new(1)) = TEST_ADDR as i32;
        *sys.reg_mut(&Reg::new(2)) = 0x51290ce3_u32 as i32;
        *sys.reg_mut(&Reg::new(6)) = OTHER_ADDR as i32;

        // Load reserved
        load_reserved(&mut sys, 3, 1).unwrap();
        // Load reserved (other addr)
        load_reserved(&mut sys, 4, 6).unwrap();
        // Store conditional (should fail)
        sc_and_assert(&mut sys, 0xbcfec832, false);
        assert_eq!(sys.pc(), 12);
    }

    #[test]
    fn test_lr_fault() {
        let mut sys = System::new(16);

        *sys.reg_mut(&Reg::new(1)) = 16;
        assert_eq!(
            load_reserved(&mut sys, 2, 1),
            Err(Trap::from_exception(LoadAccessFault, 16))
        );

        *sys.reg_mut(&Reg::new(1)) = -1;
        assert_eq!(
            load_reserved(&mut sys, 2, 1),
            Err(Trap::from_exception(LoadAccessFault, u32::MAX))
        );
    }

    #[test]
    fn test_lr_misaligned() {
        let mut sys = System::new(16);

        *sys.reg_mut(&Reg::new(1)) = 1;
        assert_eq!(
            load_reserved(&mut sys, 2, 1),
            Err(Trap::from_exception(LoadAddrMisaligned, 1))
        );

        *sys.reg_mut(&Reg::new(1)) = 2;
        assert_eq!(
            load_reserved(&mut sys, 2, 1),
            Err(Trap::from_exception(LoadAddrMisaligned, 2))
        );

        *sys.reg_mut(&Reg::new(1)) = 3;
        assert_eq!(
            load_reserved(&mut sys, 2, 1),
            Err(Trap::from_exception(LoadAddrMisaligned, 3))
        );
    }

    #[test]
    fn test_sc_fault() {
        let mut sys = System::new(16);

        *sys.reg_mut(&Reg::new(1)) = 16;
        assert_eq!(
            store_conditional(&mut sys, 4, 1, 2),
            Err(Trap::from_exception(StoreAccessFault, 16))
        );

        *sys.reg_mut(&Reg::new(1)) = -1;
        assert_eq!(
            store_conditional(&mut sys, 4, 1, 2),
            Err(Trap::from_exception(StoreAccessFault, u32::MAX))
        );
    }

    #[test]
    fn test_sc_misaligned() {
        let mut sys = System::new(16);

        *sys.reg_mut(&Reg::new(1)) = 1;
        assert_eq!(
            store_conditional(&mut sys, 4, 1, 2),
            Err(Trap::from_exception(StoreAddrMisaligned, 1))
        );

        *sys.reg_mut(&Reg::new(1)) = 2;
        assert_eq!(
            store_conditional(&mut sys, 4, 1, 2),
            Err(Trap::from_exception(StoreAddrMisaligned, 2))
        );

        *sys.reg_mut(&Reg::new(1)) = 3;
        assert_eq!(
            store_conditional(&mut sys, 4, 1, 2),
            Err(Trap::from_exception(StoreAddrMisaligned, 3))
        );
    }

    #[test]
    fn test_amo() {
        let mut sys = System::new(16);
        sys.mem.write_u32(0, 0).unwrap();
        *sys.state.reg_mut(&Reg::new(1)) = 0xbcfec832_u32 as i32;
        *sys.state.reg_mut(&Reg::new(2)) = 0x51290ce3_u32 as i32;

        assert_amo(&mut sys, 3, 0, 1, AmoFunct::Swap, 0x00000000, 0xbcfec832);
        assert_amo(&mut sys, 3, 0, 2, AmoFunct::Add, 0xbcfec832, 0x0e27d515);
        assert_amo(&mut sys, 3, 0, 1, AmoFunct::Add, 0x0e27d515, 0xcb269d47);
        assert_amo(&mut sys, 3, 0, 2, AmoFunct::Xor, 0xcb269d47, 0x9a0f91a4);
        assert_amo(&mut sys, 3, 0, 1, AmoFunct::And, 0x9a0f91a4, 0x980e8020);
        assert_amo(&mut sys, 3, 0, 2, AmoFunct::Or, 0x980e8020, 0xd92f8ce3);
        assert_amo(&mut sys, 3, 0, 2, AmoFunct::Min, 0xd92f8ce3, 0xd92f8ce3);
        assert_amo(&mut sys, 3, 0, 2, AmoFunct::Max, 0xd92f8ce3, 0x51290ce3);
        assert_amo(&mut sys, 3, 0, 1, AmoFunct::Minu, 0x51290ce3, 0x51290ce3);
        assert_amo(&mut sys, 3, 0, 1, AmoFunct::Maxu, 0x51290ce3, 0xbcfec832);

        assert_eq!(sys.pc(), 10 * 4);
    }

    #[test]
    #[rustfmt::skip]
    fn test_amo_same_reg() {
        // Test everything using only 1 register to check for data races (write must be after read)
        let mut sys = System::new(16);
        sys.mem.write_u32(0, 0).unwrap();

        *sys.state.reg_mut(&Reg::new(1)) = 0xbcfec832_u32 as i32;
        assert_amo(&mut sys, 1, 0, 1, AmoFunct::Swap, 0x00000000, 0xbcfec832);
        *sys.state.reg_mut(&Reg::new(1)) = 0x51290ce3_u32 as i32;
        assert_amo(&mut sys, 1, 0, 1, AmoFunct::Add, 0xbcfec832, 0x0e27d515);
        *sys.state.reg_mut(&Reg::new(1)) = 0xbcfec832_u32 as i32;
        assert_amo(&mut sys, 1, 0, 1, AmoFunct::Add, 0x0e27d515, 0xcb269d47);
        *sys.state.reg_mut(&Reg::new(1)) = 0x51290ce3_u32 as i32;
        assert_amo(&mut sys, 1, 0, 1, AmoFunct::Xor, 0xcb269d47, 0x9a0f91a4);
        *sys.state.reg_mut(&Reg::new(1)) = 0xbcfec832_u32 as i32;
        assert_amo(&mut sys, 1, 0, 1, AmoFunct::And, 0x9a0f91a4, 0x980e8020);
        *sys.state.reg_mut(&Reg::new(1)) = 0x51290ce3_u32 as i32;
        assert_amo(&mut sys, 1, 0, 1, AmoFunct::Or, 0x980e8020, 0xd92f8ce3);
        *sys.state.reg_mut(&Reg::new(1)) = 0x51290ce3_u32 as i32;
        assert_amo(&mut sys, 1, 0, 1, AmoFunct::Min, 0xd92f8ce3, 0xd92f8ce3);
        *sys.state.reg_mut(&Reg::new(1)) = 0x51290ce3_u32 as i32;
        assert_amo(&mut sys, 1, 0, 1, AmoFunct::Max, 0xd92f8ce3, 0x51290ce3);
        *sys.state.reg_mut(&Reg::new(1)) = 0xbcfec832_u32 as i32;
        assert_amo(&mut sys, 1, 0, 1, AmoFunct::Minu, 0x51290ce3, 0x51290ce3);
        *sys.state.reg_mut(&Reg::new(1)) = 0xbcfec832_u32 as i32;
        assert_amo(&mut sys, 1, 0, 1, AmoFunct::Maxu, 0x51290ce3, 0xbcfec832);

        assert_eq!(sys.pc(), 10 * 4);
    }

    #[test]
    fn test_amo_fault() {
        let mut sys = System::new(16);
        sys.mem.write_u32(0, 0).unwrap();
        *sys.state.reg_mut(&Reg::new(1)) = 16;
        *sys.state.reg_mut(&Reg::new(2)) = -1;

        assert_amo_failed(&mut sys, 3, 1, 2, AmoFunct::Swap, StoreAccessFault);
        assert_amo_failed(&mut sys, 3, 2, 1, AmoFunct::Add, StoreAccessFault);
        assert_amo_failed(&mut sys, 3, 1, 2, AmoFunct::Xor, StoreAccessFault);
        assert_amo_failed(&mut sys, 3, 2, 1, AmoFunct::And, StoreAccessFault);
        assert_amo_failed(&mut sys, 3, 1, 2, AmoFunct::Or, StoreAccessFault);
        assert_amo_failed(&mut sys, 3, 2, 1, AmoFunct::Min, StoreAccessFault);
        assert_amo_failed(&mut sys, 3, 1, 2, AmoFunct::Max, StoreAccessFault);
        assert_amo_failed(&mut sys, 3, 2, 1, AmoFunct::Minu, StoreAccessFault);
        assert_amo_failed(&mut sys, 3, 1, 2, AmoFunct::Maxu, StoreAccessFault);
    }

    #[test]
    fn test_amo_misaligned() {
        let mut sys = System::new(16);
        sys.mem.write_u32(0, 0).unwrap();
        *sys.state.reg_mut(&Reg::new(1)) = 1;
        *sys.state.reg_mut(&Reg::new(2)) = 6;

        assert_amo_failed(&mut sys, 3, 1, 2, AmoFunct::Swap, StoreAddrMisaligned);
        assert_amo_failed(&mut sys, 3, 2, 1, AmoFunct::Add, StoreAddrMisaligned);
        assert_amo_failed(&mut sys, 3, 1, 2, AmoFunct::Xor, StoreAddrMisaligned);
        assert_amo_failed(&mut sys, 3, 2, 1, AmoFunct::And, StoreAddrMisaligned);
        assert_amo_failed(&mut sys, 3, 1, 2, AmoFunct::Or, StoreAddrMisaligned);
        assert_amo_failed(&mut sys, 3, 2, 1, AmoFunct::Min, StoreAddrMisaligned);
        assert_amo_failed(&mut sys, 3, 1, 2, AmoFunct::Max, StoreAddrMisaligned);
        assert_amo_failed(&mut sys, 3, 2, 1, AmoFunct::Minu, StoreAddrMisaligned);
        assert_amo_failed(&mut sys, 3, 1, 2, AmoFunct::Maxu, StoreAddrMisaligned);
    }
}
