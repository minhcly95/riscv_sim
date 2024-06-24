use super::advance_pc;
use crate::{
    instr::{funct::*, reg::Reg},
    System,
};

mod int;
mod mul;

pub fn execute_op(sys: &mut System, rd: &Reg, rs1: &Reg, rs2: &Reg, f: &OpFunct) {
    match f {
        OpFunct::I(fi) => int::execute_op_i(sys, rd, rs1, rs2, fi),
        OpFunct::M(fm) => mul::execute_op_m(sys, rd, rs1, rs2, fm),
    }
    advance_pc(sys);
}
