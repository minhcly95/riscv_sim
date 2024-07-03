use super::advance_pc;
use crate::{
    instr::reg::Reg,
    System,
};

pub fn execute_lui(sys: &mut System, rd: &Reg, imm: i32) {
    let rd = sys.reg_mut(rd);
    *rd = imm;
    advance_pc(sys);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_execute_lui() {
        let mut sys = System::new();
        execute_lui(&mut sys, &Reg::new(1), 0xc43bd000_u32 as i32);
        assert_eq!(sys.state.reg(&Reg::new(1)), 0xc43bd000_u32 as i32);
        assert_eq!(sys.state.pc(), 0x4);
    }
}
