pub mod csr;
pub mod format;
pub mod funct;
pub mod reg;

use format::*;
use funct::*;

#[derive(Debug, PartialEq, Eq)]
pub enum Instr {
    Op(RType, OpFunct),
    OpImm(IType, OpImmFunct),
    Lui(UType),
    Auipc(UType),
    Load(IType, LoadFunct),
    Store(SType, StoreFunct),
    Jal(JType),
    Jalr(IType),
    Branch(BType, BranchFunct),
    Atomic(RType, AtomicFunct),
    Fence,
    Env(EnvFunct),
    Csr(CsrType, CsrFunct),
}
