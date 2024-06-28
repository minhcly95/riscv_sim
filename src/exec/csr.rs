pub mod machine;
pub mod user;

use super::{advance_pc, Result};
use crate::{
    instr::{csr::*, funct::*, reg::Reg},
    sys::control::*,
    Exception::{self, IllegalInstr},
    System,
};
use machine::*;
use user::*;

type Result32 = core::result::Result<u32, Exception>;

pub fn execute_csr(sys: &mut System, rd: &Reg, src: &CsrSrc, csr: &CsrReg, f: &CsrFunct) -> Result {
    match f {
        CsrFunct::Rw => {
            if rd.index() != 0 {
                *sys.reg_mut(rd) = csr_read(sys, csr)? as i32;
            }
            csr_write(sys, csr, get_src(sys, src))?;
        }
        CsrFunct::Rs => {
            let val = csr_read(sys, csr)?;
            *sys.reg_mut(rd) = val as i32;
            if !src.is_zero() {
                csr_write(sys, csr, val | get_src(sys, src))?;
            }
        }
        CsrFunct::Rc => {
            let val = csr_read(sys, csr)?;
            *sys.reg_mut(rd) = val as i32;
            if !src.is_zero() {
                csr_write(sys, csr, val & !get_src(sys, src))?;
            }
        }
    }
    advance_pc(sys);
    Ok(())
}

fn get_src(sys: &System, src: &CsrSrc) -> u32 {
    match src {
        CsrSrc::Reg(r) => sys.reg(r) as u32,
        CsrSrc::Imm(i) => *i as u32,
    }
}

fn csr_read(sys: &mut System, csr: &CsrReg) -> Result32 {
    match csr {
        CsrReg::U(u) => csr_read_u(sys, u),
        CsrReg::M(m) => {
            // Must be in M-mode to access
            if sys.ctrl.privilege == MPriv::M {
                csr_read_m(sys, m)
            } else {
                Err(IllegalInstr)
            }
        }
    }
}

fn csr_write(sys: &mut System, csr: &CsrReg, val: u32) -> Result {
    match csr {
        CsrReg::U(u) => csr_write_u(sys, u, val),
        CsrReg::M(m) => {
            // Must be in M-mode to access
            if sys.ctrl.privilege == MPriv::M {
                csr_write_m(sys, m, val)
            } else {
                Err(IllegalInstr)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_csr_reg(sys: &mut System, rd: u8, rs1: u8, csr: CsrReg, f: CsrFunct, expect: u32) {
        execute_csr(sys, &Reg::new(rd), &CsrSrc::Reg(Reg::new(rs1)), &csr, &f).unwrap();
        assert_eq!(sys.state.reg(&Reg::new(rd)), expect as i32);
    }

    fn assert_csr_imm(sys: &mut System, rd: u8, imm: u8, csr: CsrReg, f: CsrFunct, expect: u32) {
        execute_csr(sys, &Reg::new(rd), &CsrSrc::Imm(imm), &csr, &f).unwrap();
        assert_eq!(sys.state.reg(&Reg::new(rd)), expect as i32);
    }

    #[test]
    #[rustfmt::skip]
    fn test_execute_csr() {
        // We use mscratch since it behaves like a normal register
        let mut sys = System::new(0);
        *sys.state.reg_mut(&Reg::new(1)) = 0xbcfec832_u32 as i32;
        *sys.state.reg_mut(&Reg::new(2)) = 0x51290ce3_u32 as i32;
        sys.ctrl.mscratch = 0x0e27d515;

        assert_csr_reg(&mut sys, 3, 1, CsrReg::M(CsrRegM::MScratch), CsrFunct::Rw, 0x0e27d515);
        assert_csr_reg(&mut sys, 3, 2, CsrReg::M(CsrRegM::MScratch), CsrFunct::Rs, 0xbcfec832);
        assert_csr_reg(&mut sys, 3, 1, CsrReg::M(CsrRegM::MScratch), CsrFunct::Rc, 0xfdffccf3);

        assert_csr_imm(&mut sys, 3, 0b10011, CsrReg::M(CsrRegM::MScratch), CsrFunct::Rw, 0x410104c1);
        assert_csr_reg(&mut sys, 3, 2, CsrReg::M(CsrRegM::MScratch), CsrFunct::Rw, 0b10011);

        assert_csr_imm(&mut sys, 3, 0b00111, CsrReg::M(CsrRegM::MScratch), CsrFunct::Rs, 0x51290ce3);
        assert_csr_imm(&mut sys, 3, 0b11101, CsrReg::M(CsrRegM::MScratch), CsrFunct::Rc, 0x51290ce7);
        assert_csr_imm(&mut sys, 3, 0, CsrReg::M(CsrRegM::MScratch), CsrFunct::Rs, 0x51290ce2);

        assert_eq!(sys.state.pc(), 8 * 4);
    }
}
