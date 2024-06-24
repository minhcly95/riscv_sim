use crate::{
    instr::{funct::*, reg::Reg},
    System,
};

pub fn execute_op_m(sys: &mut System, rd: &Reg, rs1: &Reg, rs2: &Reg, f: &OpMFunct) {
    let rs1 = sys.reg(rs1);
    let rs2 = sys.reg(rs2);
    let rd = sys.reg_mut(rd);
    match f {
        OpMFunct::Mul => *rd = rs1.wrapping_mul(rs2),
        OpMFunct::Mulh => *rd = (((rs1 as i64).wrapping_mul(rs2 as i64)) >> 32) as i32,
        OpMFunct::Mulhsu => *rd = (((rs1 as i64).wrapping_mul(rs2 as u32 as i64)) >> 32) as i32,
        OpMFunct::Mulhu => {
            *rd = (((rs1 as u32 as u64).wrapping_mul(rs2 as u32 as u64)) >> 32) as i32
        }
        OpMFunct::Div => *rd = if rs2 == 0 { -1 } else { rs1.wrapping_div(rs2) },
        OpMFunct::Divu => *rd = if rs2 == 0 { -1 } else { (rs1 as u32).wrapping_div(rs2 as u32) as i32 },
        OpMFunct::Rem => *rd = if rs2 == 0 { rs1 } else { rs1.wrapping_rem(rs2) },
        OpMFunct::Remu => *rd = if rs2 == 0 { rs1 } else { (rs1 as u32).wrapping_rem(rs2 as u32) as i32 },
    };
}

#[cfg(test)]
mod tests {
    use super::super::execute_op;
    use super::*;

    fn assert_op_m(sys: &mut System, rd: u8, rs1: u8, rs2: u8, f: OpMFunct, expect: u32) {
        execute_op(
            sys,
            &Reg::new(rd),
            &Reg::new(rs1),
            &Reg::new(rs2),
            &OpFunct::M(f),
        );
        assert_eq!(sys.state.reg(&Reg::new(rd)), expect as i32);
    }

    #[test]
    fn test_execute_op_mul() {
        let mut sys = System::new(0);
        *sys.state.reg_mut(&Reg::new(1)) = 0xbcfec832_u32 as i32;
        *sys.state.reg_mut(&Reg::new(2)) = 0x51290ce3_u32 as i32;

        assert_op_m(&mut sys, 3, 1, 2, OpMFunct::Mul, 0x694fdc56);
        assert_op_m(&mut sys, 3, 1, 2, OpMFunct::Mulh, 0xeac1dec6);
        assert_op_m(&mut sys, 3, 1, 2, OpMFunct::Mulhu, 0x3beaeba9);
        assert_op_m(&mut sys, 3, 1, 2, OpMFunct::Mulhsu, 0xeac1dec6);
        assert_op_m(&mut sys, 3, 2, 1, OpMFunct::Mulhsu, 0x3beaeba9);

        assert_op_m(&mut sys, 3, 1, 1, OpMFunct::Mul, 0x4fc629c4);
        assert_op_m(&mut sys, 3, 1, 1, OpMFunct::Mulh, 0x1189a337);
        assert_op_m(&mut sys, 3, 1, 1, OpMFunct::Mulhu, 0x8b87339b);
        assert_op_m(&mut sys, 3, 1, 1, OpMFunct::Mulhsu, 0xce886b69);

        assert_op_m(&mut sys, 3, 2, 2, OpMFunct::Mul, 0xc75c1149);
        assert_op_m(&mut sys, 3, 2, 2, OpMFunct::Mulh, 0x19bb00bc);
        assert_op_m(&mut sys, 3, 2, 2, OpMFunct::Mulhu, 0x19bb00bc);
        assert_op_m(&mut sys, 3, 2, 2, OpMFunct::Mulhsu, 0x19bb00bc);

        assert_eq!(sys.state.pc(), 13 * 4);
    }

    #[test]
    fn test_execute_op_divrem() {
        let mut sys = System::new(0);
        *sys.state.reg_mut(&Reg::new(1)) = 0xbcfec832_u32 as i32;
        *sys.state.reg_mut(&Reg::new(2)) = 0xff290ce3_u32 as i32;

        assert_op_m(&mut sys, 3, 1, 2, OpMFunct::Div, 0x4f);
        assert_op_m(&mut sys, 3, 2, 1, OpMFunct::Div, 0x00);
        assert_op_m(&mut sys, 3, 1, 2, OpMFunct::Divu, 0x00);
        assert_op_m(&mut sys, 3, 2, 1, OpMFunct::Divu, 0x01);

        assert_op_m(&mut sys, 3, 1, 2, OpMFunct::Rem, 0xff53ce25);
        assert_op_m(&mut sys, 3, 2, 1, OpMFunct::Rem, 0xff290ce3);
        assert_op_m(&mut sys, 3, 1, 2, OpMFunct::Remu, 0xbcfec832);
        assert_op_m(&mut sys, 3, 2, 1, OpMFunct::Remu, 0x422a44b1);

        assert_eq!(sys.state.pc(), 8 * 4);
    }

    #[test]
    fn test_execute_op_div_zero() {
        let mut sys = System::new(0);
        *sys.state.reg_mut(&Reg::new(1)) = 0xbcfec832_u32 as i32;
        *sys.state.reg_mut(&Reg::new(2)) = 0x51290ce3_u32 as i32;

        assert_op_m(&mut sys, 3, 1, 0, OpMFunct::Div, 0xffffffff);
        assert_op_m(&mut sys, 3, 2, 0, OpMFunct::Div, 0xffffffff);
        assert_op_m(&mut sys, 3, 1, 0, OpMFunct::Divu, 0xffffffff);
        assert_op_m(&mut sys, 3, 2, 0, OpMFunct::Divu, 0xffffffff);

        assert_op_m(&mut sys, 3, 1, 0, OpMFunct::Rem, 0xbcfec832);
        assert_op_m(&mut sys, 3, 2, 0, OpMFunct::Rem, 0x51290ce3);
        assert_op_m(&mut sys, 3, 1, 0, OpMFunct::Remu, 0xbcfec832);
        assert_op_m(&mut sys, 3, 2, 0, OpMFunct::Remu, 0x51290ce3);

        assert_eq!(sys.state.pc(), 8 * 4);
    }

    #[test]
    fn test_execute_op_div_overflow() {
        let mut sys = System::new(0);
        *sys.state.reg_mut(&Reg::new(1)) = 0x80000000_u32 as i32;
        *sys.state.reg_mut(&Reg::new(2)) = 0xffffffff_u32 as i32;

        assert_op_m(&mut sys, 3, 1, 2, OpMFunct::Div, 0x80000000);
        assert_op_m(&mut sys, 3, 1, 2, OpMFunct::Divu, 0x00000000);

        assert_op_m(&mut sys, 3, 1, 2, OpMFunct::Rem, 0x00000000);
        assert_op_m(&mut sys, 3, 1, 2, OpMFunct::Remu, 0x80000000);

        assert_eq!(sys.state.pc(), 4 * 4);
    }
}
