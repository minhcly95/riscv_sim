pub mod config;
pub mod decode;
pub mod exec;
pub mod instr;
pub mod proc;
pub mod run;
pub mod sys;
pub mod translate;
pub mod trap;

pub use config::Config;
pub use instr::{reg::Reg, Instr};
pub use run::{
    load_from_file, run_for, run_for_or_until_ecall, run_until_ecall, run_until_trapped,
};
pub use sys::System;
pub use trap::{Exception, Interrupt, Trap};

pub type Result = core::result::Result<(), Trap>;

type Result32 = core::result::Result<u32, Trap>;

type ResultE = core::result::Result<(), Exception>;
type Result8E = core::result::Result<u8, Exception>;
type Result16E = core::result::Result<u16, Exception>;
type Result32E = core::result::Result<u32, Exception>;
type Result64E = core::result::Result<u64, Exception>;
