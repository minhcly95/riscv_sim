mod decode;
mod instr;

pub use decode::decode;
pub use instr::{
    format::{IType, RType, SType, UType},
    funct::{BranchFunct, LoadFunct, OpFunct, OpImmFunct, StoreFunct},
    reg::Reg,
    Instr,
};
