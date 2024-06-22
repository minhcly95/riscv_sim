use crate::{
    instr::{format::*, Instr},
    Exception, System,
};

pub type Result = core::result::Result<(), Exception>;

mod auipc;
mod branch;
mod jal;
mod jalr;
mod load;
mod lui;
mod op;
mod opimm;
mod store;

pub fn execute(sys: &mut System, instr: &Instr) -> Result {
    match instr {
        Instr::Op(RType { rd, rs1, rs2 }, f) => op::execute_op(sys, rd, rs1, rs2, f),
        Instr::OpImm(IType { rd, rs1, imm }, f) => opimm::execute_opimm(sys, rd, rs1, *imm, f),
        Instr::Lui(UType { rd, imm }) => lui::execute_lui(sys, rd, *imm),
        Instr::Auipc(UType { rd, imm }) => auipc::execute_auipc(sys, rd, *imm),
        Instr::Load(IType { rd, rs1, imm }, f) => load::execute_load(sys, rd, rs1, *imm, f)?,
        Instr::Store(SType { rs1, rs2, imm }, f) => store::execute_store(sys, rs1, rs2, *imm, f)?,
        Instr::Jal(JType { rd, imm }) => jal::execute_jal(sys, rd, *imm)?,
        Instr::Jalr(IType { rd, rs1, imm }) => jalr::execute_jalr(sys, rd, rs1, *imm)?,
        Instr::Branch(BType { rs1, rs2, imm }, f) => branch::execute_branch(sys, rs1, rs2, *imm, f)?,
        Instr::Fence => (),
        Instr::System => (),
    }
    Ok(())
}

fn advance_pc(sys: &mut System) {
    *sys.pc_mut() = sys.pc().wrapping_add(4);
}
