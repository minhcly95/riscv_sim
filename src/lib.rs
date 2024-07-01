pub mod decode;
pub mod env;
pub mod exec;
pub mod instr;
pub mod interrupt;
pub mod sys;
pub mod trap;

pub use env::Env;
pub use instr::{reg::Reg, Instr};
pub use sys::System;
pub use trap::{Exception, Interrupt, Trap};

pub type Result = core::result::Result<(), Trap>;

type Result8 = core::result::Result<u8, Trap>;
type Result16 = core::result::Result<u16, Trap>;
type Result32 = core::result::Result<u32, Trap>;
