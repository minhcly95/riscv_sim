use super::Result;
use crate::{instr::reg::Reg, Exception, System, Trap};

pub fn execute_jal(sys: &mut System, rd: &Reg, imm: i32) -> Result {
    let pc = sys.pc();
    let rd = sys.reg_mut(rd);
    let pc_jump = pc.wrapping_add_signed(imm);
    if pc_jump & 0b11 != 0 {
        return Err(Trap::from_exception(
            Exception::InstrAddrMisaligned,
            pc_jump,
        ));
    }
    *rd = pc.wrapping_add(4) as i32;
    *sys.pc_mut() = pc_jump;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_execute_jal() {
        let mut sys = System::new();
        sys.state.pc = 0xc496a1b4;
        execute_jal(&mut sys, &Reg::new(1), 0x109df8 as i32).unwrap();
        assert_eq!(sys.state.reg(&Reg::new(1)), 0xc496a1b8_u32 as i32);
        assert_eq!(sys.state.pc(), 0xc4a73fac);
    }

    #[test]
    fn test_execute_jal_misaligned() {
        let mut sys = System::new();
        sys.state.pc = 0xc496a1b4;

        assert_eq!(
            execute_jal(&mut sys, &Reg::new(1), 0x2),
            Err(Trap::from_exception(
                Exception::InstrAddrMisaligned,
                0xc496a1b6
            ))
        );
        assert_eq!(
            execute_jal(&mut sys, &Reg::new(1), 0x6),
            Err(Trap::from_exception(
                Exception::InstrAddrMisaligned,
                0xc496a1ba
            ))
        );
        assert_eq!(
            execute_jal(&mut sys, &Reg::new(1), 0xa),
            Err(Trap::from_exception(
                Exception::InstrAddrMisaligned,
                0xc496a1be
            ))
        );
        assert_eq!(
            execute_jal(&mut sys, &Reg::new(1), 0xe),
            Err(Trap::from_exception(
                Exception::InstrAddrMisaligned,
                0xc496a1c2
            ))
        );
    }
}
