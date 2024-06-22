use super::{advance_pc, Result};
use crate::{instr::reg::Reg, Exception::*, LoadFunct, System};

pub fn execute_load(sys: &mut System, rd: &Reg, rs1: &Reg, imm: i32, f: &LoadFunct) -> Result {
    let rs1 = sys.reg(rs1);
    let addr = (rs1 + imm) as usize;
    let data = match f {
        LoadFunct::B => sign_extend_8(*sys.mem.u8().get(addr).ok_or(LoadAccessFault)?),
        LoadFunct::Bu => *sys.mem.u8().get(addr).ok_or(LoadAccessFault)? as i32,
        LoadFunct::H => {
            if addr & 0b1 != 0 {
                return Err(LoadAddrMisaligned);
            }
            sign_extend_16(*sys.mem.u16().get(addr >> 1).ok_or(LoadAccessFault)?)
        }
        LoadFunct::Hu => {
            if addr & 0b1 != 0 {
                return Err(LoadAddrMisaligned);
            }
            *sys.mem.u16().get(addr >> 1).ok_or(LoadAccessFault)? as i32
        }
        LoadFunct::W => {
            if addr & 0b11 != 0 {
                return Err(LoadAddrMisaligned);
            }
            *sys.mem.u32().get(addr >> 2).ok_or(LoadAccessFault)? as i32
        }
    };
    *sys.reg_mut(rd) = data;
    advance_pc(sys);
    Ok(())
}

fn sign_extend_8(imm: u8) -> i32 {
    ((imm as i32) << 24) >> 24
}

fn sign_extend_16(imm: u16) -> i32 {
    ((imm as i32) << 16) >> 16
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Exception;

    fn assert_load(sys: &mut System, rd: u8, rs1: u8, imm: i32, f: LoadFunct, expect: u32) {
        execute_load(sys, &Reg::new(rd), &Reg::new(rs1), imm, &f).unwrap();
        assert_eq!(sys.state.reg(&Reg::new(rd)), expect as i32);
    }

    fn assert_load_failed(
        sys: &mut System,
        rd: u8,
        rs1: u8,
        imm: i32,
        f: LoadFunct,
        ex: Exception,
    ) {
        let res = execute_load(sys, &Reg::new(rd), &Reg::new(rs1), imm, &f);
        assert_eq!(res, Err(ex));
    }

    #[test]
    fn test_execute_load_byte() {
        let mut sys = System::new(16);
        sys.mem.u32_mut()[0] = 0xbcfec832;

        assert_load(&mut sys, 1, 0, 0, LoadFunct::B, 0x000000_32);
        assert_load(&mut sys, 1, 0, 1, LoadFunct::B, 0xffffff_c8);
        assert_load(&mut sys, 1, 0, 2, LoadFunct::B, 0xffffff_fe);
        assert_load(&mut sys, 1, 0, 3, LoadFunct::B, 0xffffff_bc);
        assert_eq!(sys.state.pc(), 16);
    }

    #[test]
    fn test_execute_load_byte_unsigned() {
        let mut sys = System::new(16);
        sys.mem.u32_mut()[0] = 0xbcfec832;

        assert_load(&mut sys, 1, 0, 0, LoadFunct::Bu, 0x000000_32);
        assert_load(&mut sys, 1, 0, 1, LoadFunct::Bu, 0x000000_c8);
        assert_load(&mut sys, 1, 0, 2, LoadFunct::Bu, 0x000000_fe);
        assert_load(&mut sys, 1, 0, 3, LoadFunct::Bu, 0x000000_bc);
        assert_eq!(sys.state.pc(), 16);
    }

    #[test]
    fn test_execute_load_halfword() {
        let mut sys = System::new(16);
        sys.mem.u32_mut()[0] = 0xbcfec832;

        assert_load(&mut sys, 1, 0, 0, LoadFunct::H, 0xffff_c832);
        assert_load(&mut sys, 1, 0, 2, LoadFunct::H, 0xffff_bcfe);
        assert_eq!(sys.state.pc(), 8);
    }

    #[test]
    fn test_execute_load_halfword_unsigned() {
        let mut sys = System::new(16);
        sys.mem.u32_mut()[0] = 0xbcfec832;

        assert_load(&mut sys, 1, 0, 0, LoadFunct::Hu, 0x0000_c832);
        assert_load(&mut sys, 1, 0, 2, LoadFunct::Hu, 0x0000_bcfe);
        assert_eq!(sys.state.pc(), 8);
    }

    #[test]
    fn test_execute_load_word() {
        let mut sys = System::new(16);
        sys.mem.u32_mut()[0] = 0xbcfec832;

        assert_load(&mut sys, 1, 0, 0, LoadFunct::W, 0xbcfec832);
        assert_eq!(sys.state.pc(), 4);
    }

    #[test]
    fn test_execute_load_fault() {
        let mut sys = System::new(16);

        assert_load_failed(&mut sys, 1, 0, 16, LoadFunct::B, LoadAccessFault);
        assert_load_failed(&mut sys, 1, 0, -4, LoadFunct::B, LoadAccessFault);

        assert_load_failed(&mut sys, 1, 0, 16, LoadFunct::Bu, LoadAccessFault);
        assert_load_failed(&mut sys, 1, 0, -4, LoadFunct::Bu, LoadAccessFault);

        assert_load_failed(&mut sys, 1, 0, 16, LoadFunct::H, LoadAccessFault);
        assert_load_failed(&mut sys, 1, 0, -4, LoadFunct::H, LoadAccessFault);

        assert_load_failed(&mut sys, 1, 0, 16, LoadFunct::Hu, LoadAccessFault);
        assert_load_failed(&mut sys, 1, 0, -4, LoadFunct::Hu, LoadAccessFault);

        assert_load_failed(&mut sys, 1, 0, 16, LoadFunct::W, LoadAccessFault);
        assert_load_failed(&mut sys, 1, 0, -4, LoadFunct::W, LoadAccessFault);
    }

    #[test]
    fn test_execute_load_misaligned_halfword() {
        let mut sys = System::new(16);

        assert_load_failed(&mut sys, 1, 0, 1, LoadFunct::H, LoadAddrMisaligned);
        assert_load_failed(&mut sys, 1, 0, 3, LoadFunct::H, LoadAddrMisaligned);
        assert_load_failed(&mut sys, 1, 0, 5, LoadFunct::H, LoadAddrMisaligned);
        assert_load_failed(&mut sys, 1, 0, 7, LoadFunct::H, LoadAddrMisaligned);

        assert_load_failed(&mut sys, 1, 0, 1, LoadFunct::Hu, LoadAddrMisaligned);
        assert_load_failed(&mut sys, 1, 0, 3, LoadFunct::Hu, LoadAddrMisaligned);
        assert_load_failed(&mut sys, 1, 0, 5, LoadFunct::Hu, LoadAddrMisaligned);
        assert_load_failed(&mut sys, 1, 0, 7, LoadFunct::Hu, LoadAddrMisaligned);
    }

    #[test]
    fn test_execute_load_misaligned_word() {
        let mut sys = System::new(16);

        assert_load_failed(&mut sys, 1, 0, 1, LoadFunct::W, LoadAddrMisaligned);
        assert_load_failed(&mut sys, 1, 0, 2, LoadFunct::W, LoadAddrMisaligned);
        assert_load_failed(&mut sys, 1, 0, 3, LoadFunct::W, LoadAddrMisaligned);

        assert_load_failed(&mut sys, 1, 0, 5, LoadFunct::W, LoadAddrMisaligned);
        assert_load_failed(&mut sys, 1, 0, 6, LoadFunct::W, LoadAddrMisaligned);
        assert_load_failed(&mut sys, 1, 0, 7, LoadFunct::W, LoadAddrMisaligned);
    }
}
