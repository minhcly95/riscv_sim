use super::Result;
use crate::{
    instr::{funct::BranchFunct, reg::Reg},
    Exception, System, Trap,
};

pub fn execute_branch(sys: &mut System, rs1: &Reg, rs2: &Reg, imm: i32, f: &BranchFunct) -> Result {
    let pc = sys.pc();
    let rs1 = sys.reg(rs1);
    let rs2 = sys.reg(rs2);
    let branch_cond = match f {
        BranchFunct::Eq => rs1 == rs2,
        BranchFunct::Ne => rs1 != rs2,
        BranchFunct::Lt => rs1 < rs2,
        BranchFunct::Ge => rs1 >= rs2,
        BranchFunct::Ltu => (rs1 as u32) < (rs2 as u32),
        BranchFunct::Geu => (rs1 as u32) >= (rs2 as u32),
    };
    let pc_next = pc.wrapping_add_signed(if branch_cond { imm } else { 4 });
    if pc_next & 0b11 != 0 {
        return Err(Trap::from_exception(
            Exception::InstrAddrMisaligned,
            pc_next,
        ));
    }
    *sys.pc_mut() = pc_next;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_branch(
        sys: &mut System,
        rs1: u8,
        rs2: u8,
        imm: i32,
        f: BranchFunct,
        pc_start: u32,
        pc_expect: u32,
    ) {
        sys.state.pc = pc_start;
        execute_branch(sys, &Reg::new(rs1), &Reg::new(rs2), imm, &f).unwrap();
        assert_eq!(sys.state.pc, pc_expect);
    }

    fn assert_branch_failed(
        sys: &mut System,
        rs1: u8,
        rs2: u8,
        imm: i32,
        f: BranchFunct,
        pc_start: u32,
    ) {
        sys.state.pc = pc_start;
        assert_eq!(
            execute_branch(sys, &Reg::new(rs1), &Reg::new(rs2), imm, &f),
            Err(Trap::from_exception(
                Exception::InstrAddrMisaligned,
                pc_start.wrapping_add_signed(imm)
            ))
        );
    }

    #[test]
    fn test_execute_branch() {
        let mut sys = System::new();
        *sys.reg_mut(&Reg::new(1)) = 0xbcfec832_u32 as i32;
        *sys.reg_mut(&Reg::new(2)) = 0x51290ce3_u32 as i32;
        *sys.reg_mut(&Reg::new(4)) = 0xbcfec832_u32 as i32;

        assert_branch(&mut sys, 1, 2, 0x3b4, BranchFunct::Eq, 0, 0x004);
        assert_branch(&mut sys, 2, 1, 0x3b4, BranchFunct::Eq, 0, 0x004);
        assert_branch(&mut sys, 1, 4, 0x3b4, BranchFunct::Eq, 0, 0x3b4);

        assert_branch(&mut sys, 1, 2, 0x3b4, BranchFunct::Ne, 0, 0x3b4);
        assert_branch(&mut sys, 2, 1, 0x3b4, BranchFunct::Ne, 0, 0x3b4);
        assert_branch(&mut sys, 1, 4, 0x3b4, BranchFunct::Ne, 0, 0x004);

        assert_branch(&mut sys, 1, 2, 0x3b4, BranchFunct::Lt, 0, 0x3b4);
        assert_branch(&mut sys, 2, 1, 0x3b4, BranchFunct::Lt, 0, 0x004);
        assert_branch(&mut sys, 1, 4, 0x3b4, BranchFunct::Lt, 0, 0x004);

        assert_branch(&mut sys, 1, 2, 0x3b4, BranchFunct::Ge, 0, 0x004);
        assert_branch(&mut sys, 2, 1, 0x3b4, BranchFunct::Ge, 0, 0x3b4);
        assert_branch(&mut sys, 1, 4, 0x3b4, BranchFunct::Ge, 0, 0x3b4);

        assert_branch(&mut sys, 1, 2, 0x3b4, BranchFunct::Ltu, 0, 0x004);
        assert_branch(&mut sys, 2, 1, 0x3b4, BranchFunct::Ltu, 0, 0x3b4);
        assert_branch(&mut sys, 1, 4, 0x3b4, BranchFunct::Ltu, 0, 0x004);

        assert_branch(&mut sys, 1, 2, 0x3b4, BranchFunct::Geu, 0, 0x3b4);
        assert_branch(&mut sys, 2, 1, 0x3b4, BranchFunct::Geu, 0, 0x004);
        assert_branch(&mut sys, 1, 4, 0x3b4, BranchFunct::Geu, 0, 0x3b4);
    }

    #[test]
    fn test_execute_branch_misaligned() {
        let mut sys = System::new();
        *sys.reg_mut(&Reg::new(1)) = 0xbcfec832_u32 as i32;
        *sys.reg_mut(&Reg::new(2)) = 0x51290ce3_u32 as i32;
        *sys.reg_mut(&Reg::new(4)) = 0xbcfec832_u32 as i32;

        assert_branch(&mut sys, 1, 2, 0x2, BranchFunct::Eq, 0, 0x4);
        assert_branch(&mut sys, 2, 1, 0x2, BranchFunct::Eq, 0, 0x4);
        assert_branch_failed(&mut sys, 1, 4, 0x2, BranchFunct::Eq, 0);

        assert_branch_failed(&mut sys, 1, 2, 0x2, BranchFunct::Ne, 0);
        assert_branch_failed(&mut sys, 2, 1, 0x2, BranchFunct::Ne, 0);
        assert_branch(&mut sys, 1, 4, 0x2, BranchFunct::Ne, 0, 0x4);

        assert_branch_failed(&mut sys, 1, 2, 0x2, BranchFunct::Lt, 0);
        assert_branch(&mut sys, 2, 1, 0x2, BranchFunct::Lt, 0, 0x4);
        assert_branch(&mut sys, 1, 4, 0x2, BranchFunct::Lt, 0, 0x4);

        assert_branch(&mut sys, 1, 2, 0x2, BranchFunct::Ge, 0, 0x4);
        assert_branch_failed(&mut sys, 2, 1, 0x2, BranchFunct::Ge, 0);
        assert_branch_failed(&mut sys, 1, 4, 0x2, BranchFunct::Ge, 0);

        assert_branch(&mut sys, 1, 2, 0x2, BranchFunct::Ltu, 0, 0x4);
        assert_branch_failed(&mut sys, 2, 1, 0x2, BranchFunct::Ltu, 0);
        assert_branch(&mut sys, 1, 4, 0x2, BranchFunct::Ltu, 0, 0x4);

        assert_branch_failed(&mut sys, 1, 2, 0x2, BranchFunct::Geu, 0);
        assert_branch(&mut sys, 2, 1, 0x2, BranchFunct::Geu, 0, 0x4);
        assert_branch_failed(&mut sys, 1, 4, 0x2, BranchFunct::Geu, 0);
    }
}
