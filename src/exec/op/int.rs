use crate::{
    instr::{funct::*, reg::Reg},
    System,
};

pub fn execute_op_i(sys: &mut System, rd: &Reg, rs1: &Reg, rs2: &Reg, f: &OpIFunct) {
    let rs1 = sys.reg(rs1);
    let rs2 = sys.reg(rs2);
    let rd = sys.reg_mut(rd);
    match f {
        OpIFunct::Add => *rd = rs1.wrapping_add(rs2),
        OpIFunct::Sub => *rd = rs1.wrapping_sub(rs2),
        OpIFunct::Slt => *rd = if (rs1 as i32) < (rs2 as i32) { 1 } else { 0 },
        OpIFunct::Sltu => *rd = if (rs1 as u32) < (rs2 as u32) { 1 } else { 0 },
        OpIFunct::Xor => *rd = rs1 ^ rs2,
        OpIFunct::Or => *rd = rs1 | rs2,
        OpIFunct::And => *rd = rs1 & rs2,
        OpIFunct::Sll => *rd = rs1.wrapping_shl(rs2 as u32),
        OpIFunct::Srl => *rd = (rs1 as u32).wrapping_shr(rs2 as u32) as i32,
        OpIFunct::Sra => *rd = (rs1 as i32).wrapping_shr(rs2 as u32),
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::execute_op;

    fn assert_op_i(sys: &mut System, rd: u8, rs1: u8, rs2: u8, f: OpIFunct, expect: u32) {
        execute_op(sys, &Reg::new(rd), &Reg::new(rs1), &Reg::new(rs2), &OpFunct::I(f));
        assert_eq!(sys.state.reg(&Reg::new(rd)), expect as i32);
    }

    #[test]
    fn test_execute_op_i() {
        let mut sys = System::new();
        *sys.state.reg_mut(&Reg::new(1)) = 0xbcfec832_u32 as i32;
        *sys.state.reg_mut(&Reg::new(2)) = 0x51290ce3_u32 as i32;

        assert_op_i(&mut sys, 3, 1, 2, OpIFunct::Add, 0x0e27d515);
        assert_op_i(&mut sys, 4, 1, 2, OpIFunct::Sub, 0x6bd5bb4f);
        assert_op_i(&mut sys, 5, 2, 1, OpIFunct::Sub, 0x942a44b1);
        assert_op_i(&mut sys, 6, 1, 2, OpIFunct::Xor, 0xedd7c4d1);
        assert_op_i(&mut sys, 7, 1, 2, OpIFunct::Or, 0xfdffccf3);
        assert_op_i(&mut sys, 8, 1, 2, OpIFunct::And, 0x10280822);
        assert_op_i(&mut sys, 9, 1, 2, OpIFunct::Slt, 1);
        assert_op_i(&mut sys, 10, 2, 1, OpIFunct::Slt, 0);
        assert_op_i(&mut sys, 11, 1, 1, OpIFunct::Slt, 0);
        assert_op_i(&mut sys, 12, 2, 2, OpIFunct::Slt, 0);
        assert_op_i(&mut sys, 13, 1, 2, OpIFunct::Sltu, 0);
        assert_op_i(&mut sys, 14, 2, 1, OpIFunct::Sltu, 1);
        assert_op_i(&mut sys, 15, 1, 2, OpIFunct::Sll, 0xe7f64190);
        assert_op_i(&mut sys, 16, 2, 1, OpIFunct::Sll, 0x338c0000);
        assert_op_i(&mut sys, 17, 1, 2, OpIFunct::Srl, 0x179fd906);
        assert_op_i(&mut sys, 18, 2, 1, OpIFunct::Srl, 0x0000144a);
        assert_op_i(&mut sys, 19, 1, 2, OpIFunct::Sra, 0xf79fd906);
        assert_op_i(&mut sys, 20, 2, 1, OpIFunct::Sra, 0x0000144a);

        assert_eq!(sys.state.pc(), 18 * 4);
    }
}
