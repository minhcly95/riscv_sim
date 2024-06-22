use super::advance_pc;
use crate::{instr::reg::Reg, System};

pub fn execute_auipc(sys: &mut System, rd: &Reg, imm: i32) {
    let pc = sys.pc();
    let rd = sys.reg_mut(rd);
    *rd = pc.wrapping_add_signed(imm) as i32;
    advance_pc(sys);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_execute_auipc() {
        let mut sys = System::new(0);
        sys.state.pc = 0x164;
        execute_auipc(&mut sys, &Reg::new(1), 0xc43bd000_u32 as i32);
        assert_eq!(sys.state.reg(&Reg::new(1)), 0xc43bd164_u32 as i32);
        assert_eq!(sys.state.pc(), 0x168);
    }
}
