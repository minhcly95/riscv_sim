use super::Result;
use crate::{instr::reg::Reg, Exception, System};

pub fn execute_jalr(sys: &mut System, rd: &Reg, rs1: &Reg, imm: i32) -> Result {
    let pc = sys.pc();
    let rs1 = sys.reg(rs1);
    let rd = sys.reg_mut(rd);
    let pc_jump = (rs1.wrapping_add(imm) as u32) & 0xfffffffe;
    if pc_jump & 0b11 != 0 {
        return Err(Exception::InstrAddrMisaligned);
    }
    *rd = pc.wrapping_add(4) as i32;
    *sys.pc_mut() = pc_jump;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_execute_jalr() {
        let mut sys = System::new(0);
        sys.state.pc = 0xc496a1b4;
        *sys.reg_mut(&Reg::new(2)) = 0xbcfec832_u32 as i32;

        execute_jalr(&mut sys, &Reg::new(1), &Reg::new(2), 0xfffff_dfa_u32 as i32).unwrap();
        assert_eq!(sys.state.reg(&Reg::new(1)), 0xc496a1b8_u32 as i32);
        assert_eq!(sys.state.pc(), 0xbcfec62c);
    }

    #[test]
    fn test_execute_jalr_misaligned() {
        let mut sys = System::new(0);
        sys.state.pc = 0xc496a1b4;
        *sys.reg_mut(&Reg::new(2)) = 0xbcfec832_u32 as i32;

        assert_eq!(
            execute_jalr(&mut sys, &Reg::new(1), &Reg::new(2), 0x0),
            Err(Exception::InstrAddrMisaligned)
        );
        assert_eq!(
            execute_jalr(&mut sys, &Reg::new(1), &Reg::new(2), 0x1),
            Err(Exception::InstrAddrMisaligned)
        );
        assert_eq!(
            execute_jalr(&mut sys, &Reg::new(1), &Reg::new(2), 0x4),
            Err(Exception::InstrAddrMisaligned)
        );
        assert_eq!(
            execute_jalr(&mut sys, &Reg::new(1), &Reg::new(2), 0x5),
            Err(Exception::InstrAddrMisaligned)
        );
    }
}
