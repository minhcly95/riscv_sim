use super::advance_pc;
use crate::{
    instr::{funct::*, reg::Reg},
    System,
};

pub fn execute_op(sys: &mut System, rd: &Reg, rs1: &Reg, rs2: &Reg, f: &OpFunct) {
    let rs1 = sys.reg(rs1);
    let rs2 = sys.reg(rs2);
    let rd = sys.reg_mut(rd);
    match f {
        OpFunct::Add => *rd = rs1.wrapping_add(rs2),
        OpFunct::Sub => *rd = rs1.wrapping_sub(rs2),
        OpFunct::Slt => *rd = if rs1 < rs2 { 1 } else { 0 },
        OpFunct::Sltu => *rd = if (rs1 as u32) < (rs2 as u32) { 1 } else { 0 },
        OpFunct::Xor => *rd = rs1 ^ rs2,
        OpFunct::Or => *rd = rs1 | rs2,
        OpFunct::And => *rd = rs1 & rs2,
        OpFunct::Sll => *rd = rs1.wrapping_shl(rs2 as u32),
        OpFunct::Srl => *rd = (rs1 as u32).wrapping_shr(rs2 as u32) as i32,
        OpFunct::Sra => *rd = rs1.wrapping_shr(rs2 as u32),
    };
    advance_pc(sys);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_op(sys: &mut System, rd: u8, rs1: u8, rs2: u8, f: OpFunct, expect: u32) {
        execute_op(sys, &Reg::new(rd), &Reg::new(rs1), &Reg::new(rs2), &f);
        assert_eq!(sys.state.reg(&Reg::new(rd)), expect as i32);
    }

    #[test]
    fn test_execute_op() {
        let mut sys = System::new(0);
        *sys.state.reg_mut(&Reg::new(1)) = 0xbcfec832_u32 as i32;
        *sys.state.reg_mut(&Reg::new(2)) = 0x51290ce3_u32 as i32;

        assert_op(&mut sys, 3, 1, 2, OpFunct::Add, 0x0e27d515);
        assert_op(&mut sys, 4, 1, 2, OpFunct::Sub, 0x6bd5bb4f);
        assert_op(&mut sys, 5, 2, 1, OpFunct::Sub, 0x942a44b1);
        assert_op(&mut sys, 6, 1, 2, OpFunct::Xor, 0xedd7c4d1);
        assert_op(&mut sys, 7, 1, 2, OpFunct::Or, 0xfdffccf3);
        assert_op(&mut sys, 8, 1, 2, OpFunct::And, 0x10280822);
        assert_op(&mut sys, 9, 1, 2, OpFunct::Slt, 1);
        assert_op(&mut sys, 10, 2, 1, OpFunct::Slt, 0);
        assert_op(&mut sys, 11, 1, 1, OpFunct::Slt, 0);
        assert_op(&mut sys, 12, 2, 2, OpFunct::Slt, 0);
        assert_op(&mut sys, 13, 1, 2, OpFunct::Sltu, 0);
        assert_op(&mut sys, 14, 2, 1, OpFunct::Sltu, 1);
        assert_op(&mut sys, 15, 1, 2, OpFunct::Sll, 0xe7f64190);
        assert_op(&mut sys, 16, 2, 1, OpFunct::Sll, 0x338c0000);
        assert_op(&mut sys, 17, 1, 2, OpFunct::Srl, 0x179fd906);
        assert_op(&mut sys, 18, 2, 1, OpFunct::Srl, 0x0000144a);
        assert_op(&mut sys, 19, 1, 2, OpFunct::Sra, 0xf79fd906);
        assert_op(&mut sys, 20, 2, 1, OpFunct::Sra, 0x0000144a);

        assert_eq!(sys.state.pc(), 18 * 4);
    }
}
