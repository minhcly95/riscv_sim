use super::{advance_pc, Result};
use crate::{instr::reg::Reg, StoreFunct, System};

pub fn execute_store(sys: &mut System, rs1: &Reg, rs2: &Reg, imm: i32, f: &StoreFunct) -> Result {
    let rs1 = sys.reg(rs1);
    let rs2 = sys.reg(rs2);
    let addr = (rs1 + imm) as u32;
    match f {
        StoreFunct::B => sys.mem.write_u8(addr, rs2 as u8)?,
        StoreFunct::H => sys.mem.write_u16(addr, rs2 as u16)?,
        StoreFunct::W => sys.mem.write_u32(addr, rs2 as u32)?,
    };
    advance_pc(sys);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Exception::{self, *};

    fn assert_store(sys: &mut System, rs1: u8, rs2: u8, imm: i32, f: StoreFunct, expect: &[u8]) {
        let rs1 = Reg::new(rs1);
        execute_store(sys, &rs1, &Reg::new(rs2), imm, &f).unwrap();

        let addr = (sys.state.reg(&rs1) + imm) as usize;
        for i in 0..expect.len() {
            assert_eq!(sys.mem.as_u8()[addr + i], expect[i]);
        }
    }

    fn assert_store_failed(
        sys: &mut System,
        rs1: u8,
        rs2: u8,
        imm: i32,
        f: StoreFunct,
        ex: Exception,
    ) {
        let res = execute_store(sys, &Reg::new(rs1), &Reg::new(rs2), imm, &f);
        assert_eq!(res, Err(ex));
    }

    #[test]
    fn test_execute_load_byte() {
        let mut sys = System::new(16);
        *sys.reg_mut(&Reg::new(1)) = 0xbcfec832_u32 as i32;
        *sys.reg_mut(&Reg::new(2)) = 0x32bcfec8_u32 as i32;
        *sys.reg_mut(&Reg::new(3)) = 0xc832bcfe_u32 as i32;
        *sys.reg_mut(&Reg::new(4)) = 0xfec832bc_u32 as i32;

        assert_store(&mut sys, 0, 1, 0, StoreFunct::B, &[0x32]);
        assert_store(&mut sys, 0, 2, 1, StoreFunct::B, &[0xc8]);
        assert_store(&mut sys, 0, 3, 2, StoreFunct::B, &[0xfe]);
        assert_store(&mut sys, 0, 4, 3, StoreFunct::B, &[0xbc]);

        assert_eq!(sys.mem.as_u32()[0], 0xbcfec832);
        assert_eq!(sys.state.pc(), 16);
    }

    #[test]
    fn test_execute_store_halfword() {
        let mut sys = System::new(16);
        *sys.reg_mut(&Reg::new(1)) = 0x51290ce3_u32 as i32;
        *sys.reg_mut(&Reg::new(2)) = 0x0ce35129_u32 as i32;

        assert_store(&mut sys, 0, 1, 0, StoreFunct::H, &[0xe3, 0x0c]);
        assert_store(&mut sys, 0, 2, 2, StoreFunct::H, &[0x29, 0x51]);

        assert_eq!(sys.mem.as_u32()[0], 0x51290ce3);
        assert_eq!(sys.state.pc(), 8);
    }

    #[test]
    fn test_execute_store_word() {
        let mut sys = System::new(16);
        *sys.reg_mut(&Reg::new(1)) = 0xbcfec832_u32 as i32;
        *sys.reg_mut(&Reg::new(2)) = 0x51290ce3_u32 as i32;

        assert_store(&mut sys, 0, 1, 0, StoreFunct::W, &[0x32, 0xc8, 0xfe, 0xbc]);
        assert_store(&mut sys, 0, 2, 4, StoreFunct::W, &[0xe3, 0x0c, 0x29, 0x51]);

        assert_eq!(sys.mem.as_u32()[0], 0xbcfec832);
        assert_eq!(sys.mem.as_u32()[1], 0x51290ce3);
        assert_eq!(sys.state.pc(), 8);
    }

    #[test]
    fn test_execute_store_fault() {
        let mut sys = System::new(16);

        assert_store_failed(&mut sys, 0, 1, 16, StoreFunct::B, StoreAccessFault);
        assert_store_failed(&mut sys, 0, 1, -4, StoreFunct::B, StoreAccessFault);

        assert_store_failed(&mut sys, 0, 1, 16, StoreFunct::H, StoreAccessFault);
        assert_store_failed(&mut sys, 0, 1, -4, StoreFunct::H, StoreAccessFault);

        assert_store_failed(&mut sys, 0, 1, 16, StoreFunct::W, StoreAccessFault);
        assert_store_failed(&mut sys, 0, 1, -4, StoreFunct::W, StoreAccessFault);
    }

    #[test]
    fn test_execute_store_misaligned_halfword() {
        let mut sys = System::new(16);

        assert_store_failed(&mut sys, 0, 1, 1, StoreFunct::H, StoreAddrMisaligned);
        assert_store_failed(&mut sys, 0, 1, 3, StoreFunct::H, StoreAddrMisaligned);
        assert_store_failed(&mut sys, 0, 1, 5, StoreFunct::H, StoreAddrMisaligned);
        assert_store_failed(&mut sys, 0, 1, 7, StoreFunct::H, StoreAddrMisaligned);
    }

    #[test]
    fn test_execute_store_misaligned_word() {
        let mut sys = System::new(16);

        assert_store_failed(&mut sys, 0, 1, 1, StoreFunct::W, StoreAddrMisaligned);
        assert_store_failed(&mut sys, 0, 1, 2, StoreFunct::W, StoreAddrMisaligned);
        assert_store_failed(&mut sys, 0, 1, 3, StoreFunct::W, StoreAddrMisaligned);

        assert_store_failed(&mut sys, 0, 1, 5, StoreFunct::W, StoreAddrMisaligned);
        assert_store_failed(&mut sys, 0, 1, 6, StoreFunct::W, StoreAddrMisaligned);
        assert_store_failed(&mut sys, 0, 1, 7, StoreFunct::W, StoreAddrMisaligned);
    }
}
