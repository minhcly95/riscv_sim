use super::advance_pc;
use crate::{
    instr::{funct::*, reg::Reg},
    System,
};

pub fn execute_opimm(sys: &mut System, rd: &Reg, rs1: &Reg, imm: i32, f: &OpImmFunct) {
    let rs1 = sys.reg(rs1);
    let rd = sys.reg_mut(rd);
    match f {
        OpImmFunct::Add => *rd = rs1.wrapping_add(imm),
        OpImmFunct::Slt => *rd = if rs1 < imm { 1 } else { 0 },
        OpImmFunct::Sltu => *rd = if (rs1 as u32) < (imm as u32) { 1 } else { 0 },
        OpImmFunct::Xor => *rd = rs1 ^ imm,
        OpImmFunct::Or => *rd = rs1 | imm,
        OpImmFunct::And => *rd = rs1 & imm,
        OpImmFunct::Sll => *rd = rs1.wrapping_shl(imm as u32),
        OpImmFunct::Srl => *rd = (rs1 as u32).wrapping_shr(imm as u32) as i32,
        OpImmFunct::Sra => *rd = rs1.wrapping_shr(imm as u32),
    };
    advance_pc(sys);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_opimm(sys: &mut System, rd: u8, rs1: u8, imm: i32, f: OpImmFunct, expect: u32) {
        execute_opimm(sys, &Reg::new(rd), &Reg::new(rs1), imm, &f);
        assert_eq!(sys.reg(&Reg::new(rd)), expect as i32);
    }

    #[test]
    fn test_execute_opimm() {
        let mut sys = System::new(0);
        *sys.state.reg_mut(&Reg::new(1)) = 0xbcfec832_u32 as i32;
        *sys.state.reg_mut(&Reg::new(2)) = 0x51290ce3_u32 as i32;

        let a = 0xfffff89b_u32 as i32;
        let b = 0x3b2;

        assert_opimm(&mut sys, 3, 1, a, OpImmFunct::Add, 0xbcfec0cd);
        assert_opimm(&mut sys, 4, 2, b, OpImmFunct::Add, 0x51291095);
        assert_opimm(&mut sys, 5, 1, a, OpImmFunct::Xor, 0x430130a9);
        assert_opimm(&mut sys, 6, 2, b, OpImmFunct::Xor, 0x51290f51);
        assert_opimm(&mut sys, 7, 1, a, OpImmFunct::Or, 0xfffff8bb);
        assert_opimm(&mut sys, 8, 2, b, OpImmFunct::Or, 0x51290ff3);
        assert_opimm(&mut sys, 9, 1, a, OpImmFunct::And, 0xbcfec812);
        assert_opimm(&mut sys, 10, 2, b, OpImmFunct::And, 0x000000a2);
        assert_opimm(&mut sys, 11, 1, a, OpImmFunct::Slt, 1);
        assert_opimm(&mut sys, 12, 1, b, OpImmFunct::Slt, 1);
        assert_opimm(&mut sys, 13, 2, a, OpImmFunct::Slt, 0);
        assert_opimm(&mut sys, 14, 2, b, OpImmFunct::Slt, 0);
        assert_opimm(&mut sys, 15, 1, a, OpImmFunct::Sltu, 1);
        assert_opimm(&mut sys, 16, 1, b, OpImmFunct::Sltu, 0);
        assert_opimm(&mut sys, 17, 2, a, OpImmFunct::Sltu, 1);
        assert_opimm(&mut sys, 18, 2, b, OpImmFunct::Sltu, 0);
        assert_opimm(&mut sys, 19, 1, 11, OpImmFunct::Sll, 0xf6419000);
        assert_opimm(&mut sys, 20, 2, 2, OpImmFunct::Sll, 0x44a4338c);
        assert_opimm(&mut sys, 21, 1, 11, OpImmFunct::Srl, 0x00179fd9);
        assert_opimm(&mut sys, 22, 2, 2, OpImmFunct::Srl, 0x144a4338);
        assert_opimm(&mut sys, 23, 1, 11, OpImmFunct::Sra, 0xfff79fd9);
        assert_opimm(&mut sys, 24, 2, 2, OpImmFunct::Sra, 0x144a4338);

        assert_eq!(sys.state.pc(), 22 * 4);
    }
}
