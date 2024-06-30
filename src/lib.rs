mod decode;
mod env;
mod exception;
mod exec;
mod instr;
mod sys;
mod trap;

pub use decode::decode;
pub use env::Env;
pub use exception::Exception;
pub use exec::execute;
pub use instr::{
    format::{BType, IType, JType, RType, SType, UType},
    funct::{BranchFunct, LoadFunct, OpFunct, OpImmFunct, StoreFunct},
    reg::Reg,
    Instr,
};
pub use sys::System;
pub use trap::Trap;

pub type Result = core::result::Result<(), Trap>;

type Result8 = core::result::Result<u8, Trap>;
type Result16 = core::result::Result<u16, Trap>;
type Result32 = core::result::Result<u32, Trap>;
