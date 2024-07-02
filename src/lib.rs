pub mod decode;
pub mod env;
pub mod exec;
pub mod instr;
pub mod interrupt;
pub mod sys;
pub mod translate;
pub mod trap;

pub use env::Env;
pub use instr::{reg::Reg, Instr};
pub use sys::System;
pub use trap::{Exception, Interrupt, Trap};

pub type Result = core::result::Result<(), Trap>;

type Result32 = core::result::Result<u32, Trap>;

type ResultE = core::result::Result<(), Exception>;
type Result8E = core::result::Result<u8, Exception>;
type Result16E = core::result::Result<u16, Exception>;
type Result32E = core::result::Result<u32, Exception>;
type Result64E = core::result::Result<u64, Exception>;
