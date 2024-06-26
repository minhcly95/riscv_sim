use crate::instr::{format::*, funct::*, *};

const OPCODE_MASK: u32 = (1 << 7) - 1;

const OPCODE_OP: u8 = 0b0110011;
const OPCODE_OPIMM: u8 = 0b0010011;
const OPCODE_LUI: u8 = 0b0110111;
const OPCODE_AUIPC: u8 = 0b0010111;
const OPCODE_LOAD: u8 = 0b0000011;
const OPCODE_STORE: u8 = 0b0100011;
const OPCODE_JAL: u8 = 0b1101111;
const OPCODE_JALR: u8 = 0b1100111;
const OPCODE_BRANCH: u8 = 0b1100011;
const OPCODE_MISC: u8 = 0b0001111;
const OPCODE_SYSTEM: u8 = 0b1110011;
const OPCODE_AMO: u8 = 0b0101111;

pub fn decode(code: u32) -> Option<Instr> {
    let opcode = (code & OPCODE_MASK) as u8;
    match opcode {
        OPCODE_OP => Some(Instr::Op(RType::from(code), OpFunct::from(code)?)),
        OPCODE_OPIMM => {
            let funct = OpImmFunct::from(code)?;
            match funct {
                OpImmFunct::Sll | OpImmFunct::Srl | OpImmFunct::Sra => {
                    Some(Instr::OpImm(IType::from_shamt(code), funct))
                }
                _ => Some(Instr::OpImm(IType::from(code), funct)),
            }
        }
        OPCODE_LUI => Some(Instr::Lui(UType::from(code))),
        OPCODE_AUIPC => Some(Instr::Auipc(UType::from(code))),
        OPCODE_LOAD => Some(Instr::Load(IType::from(code), LoadFunct::from(code)?)),
        OPCODE_STORE => Some(Instr::Store(SType::from(code), StoreFunct::from(code)?)),
        OPCODE_JAL => Some(Instr::Jal(JType::from(code))),
        OPCODE_JALR => Some(Instr::Jalr(IType::from(code))),
        OPCODE_BRANCH => Some(Instr::Branch(BType::from(code), BranchFunct::from(code)?)),
        OPCODE_AMO => Some(Instr::Atomic(RType::from(code), AtomicFunct::from(code)?)),
        OPCODE_MISC => Some(Instr::Fence),
        OPCODE_SYSTEM => Some(Instr::System),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::instr::reg::Reg;

    #[test]
    #[rustfmt::skip]
    fn test_decode_op() {
        assert_eq!(decode(0x003084b3).unwrap(), Instr::Op(RType { rd: Reg::new( 9), rs1: Reg::new( 1), rs2: Reg::new( 3)}, OpFunct::I(OpIFunct::Add)));
        assert_eq!(decode(0x41968ab3).unwrap(), Instr::Op(RType { rd: Reg::new(21), rs1: Reg::new(13), rs2: Reg::new(25)}, OpFunct::I(OpIFunct::Sub)));
        assert_eq!(decode(0x00612033).unwrap(), Instr::Op(RType { rd: Reg::new( 0), rs1: Reg::new( 2), rs2: Reg::new( 6)}, OpFunct::I(OpIFunct::Slt)));
        assert_eq!(decode(0x00613833).unwrap(), Instr::Op(RType { rd: Reg::new(16), rs1: Reg::new( 2), rs2: Reg::new( 6)}, OpFunct::I(OpIFunct::Sltu)));
        assert_eq!(decode(0x00ab4733).unwrap(), Instr::Op(RType { rd: Reg::new(14), rs1: Reg::new(22), rs2: Reg::new(10)}, OpFunct::I(OpIFunct::Xor)));
        assert_eq!(decode(0x01a9e1b3).unwrap(), Instr::Op(RType { rd: Reg::new( 3), rs1: Reg::new(19), rs2: Reg::new(26)}, OpFunct::I(OpIFunct::Or)));
        assert_eq!(decode(0x0021f0b3).unwrap(), Instr::Op(RType { rd: Reg::new( 1), rs1: Reg::new( 3), rs2: Reg::new( 2)}, OpFunct::I(OpIFunct::And)));
        assert_eq!(decode(0x00729733).unwrap(), Instr::Op(RType { rd: Reg::new(14), rs1: Reg::new( 5), rs2: Reg::new( 7)}, OpFunct::I(OpIFunct::Sll)));
        assert_eq!(decode(0x0192d0b3).unwrap(), Instr::Op(RType { rd: Reg::new( 1), rs1: Reg::new( 5), rs2: Reg::new(25)}, OpFunct::I(OpIFunct::Srl)));
        assert_eq!(decode(0x41cb5d33).unwrap(), Instr::Op(RType { rd: Reg::new(26), rs1: Reg::new(22), rs2: Reg::new(28)}, OpFunct::I(OpIFunct::Sra)));
    }

    #[test]
    #[rustfmt::skip]
    fn test_decode_opimm() {
        assert_eq!(decode(0x06630c13).unwrap(), Instr::OpImm(IType { rd: Reg::new(24), rs1: Reg::new( 6), imm: 102   }, OpImmFunct::Add));
        assert_eq!(decode(0xbe2d2113).unwrap(), Instr::OpImm(IType { rd: Reg::new( 2), rs1: Reg::new(26), imm: -1054 }, OpImmFunct::Slt));
        assert_eq!(decode(0x5194b013).unwrap(), Instr::OpImm(IType { rd: Reg::new( 0), rs1: Reg::new( 9), imm: 1305  }, OpImmFunct::Sltu));
        assert_eq!(decode(0xdb1a4e13).unwrap(), Instr::OpImm(IType { rd: Reg::new(28), rs1: Reg::new(20), imm: -591  }, OpImmFunct::Xor));
        assert_eq!(decode(0x44e6e513).unwrap(), Instr::OpImm(IType { rd: Reg::new(10), rs1: Reg::new(13), imm: 1102  }, OpImmFunct::Or));
        assert_eq!(decode(0xc007f593).unwrap(), Instr::OpImm(IType { rd: Reg::new(11), rs1: Reg::new(15), imm: -1024 }, OpImmFunct::And));
        assert_eq!(decode(0x01e79913).unwrap(), Instr::OpImm(IType { rd: Reg::new(18), rs1: Reg::new(15), imm: 30    }, OpImmFunct::Sll));
        assert_eq!(decode(0x01b25193).unwrap(), Instr::OpImm(IType { rd: Reg::new( 3), rs1: Reg::new( 4), imm: 27    }, OpImmFunct::Srl));
        assert_eq!(decode(0x4042d513).unwrap(), Instr::OpImm(IType { rd: Reg::new(10), rs1: Reg::new( 5), imm: 4     }, OpImmFunct::Sra));
    }

    #[test]
    #[rustfmt::skip]
    fn test_decode_load() {
        assert_eq!(decode(0x078f0983).unwrap(), Instr::Load(IType { rd: Reg::new(19), rs1: Reg::new(30), imm: 120   }, LoadFunct::B));
        assert_eq!(decode(0x9a1d9783).unwrap(), Instr::Load(IType { rd: Reg::new(15), rs1: Reg::new(27), imm: -1631 }, LoadFunct::H));
        assert_eq!(decode(0x05c92403).unwrap(), Instr::Load(IType { rd: Reg::new( 8), rs1: Reg::new(18), imm: 92    }, LoadFunct::W));
        assert_eq!(decode(0xe1f6c103).unwrap(), Instr::Load(IType { rd: Reg::new( 2), rs1: Reg::new(13), imm: -481  }, LoadFunct::Bu));
        assert_eq!(decode(0x12c15b83).unwrap(), Instr::Load(IType { rd: Reg::new(23), rs1: Reg::new( 2), imm: 300   }, LoadFunct::Hu));
    }

    #[test]
    #[rustfmt::skip]
    fn test_decode_store() {
        assert_eq!(decode(0xc3a08fa3).unwrap(), Instr::Store(SType { rs1: Reg::new( 1), rs2: Reg::new(26), imm: -961 }, StoreFunct::B));
        assert_eq!(decode(0x7f121723).unwrap(), Instr::Store(SType { rs1: Reg::new( 4), rs2: Reg::new(17), imm: 2030 }, StoreFunct::H));
        assert_eq!(decode(0xd58d2323).unwrap(), Instr::Store(SType { rs1: Reg::new(26), rs2: Reg::new(24), imm: -698 }, StoreFunct::W));
    }

    #[test]
    #[rustfmt::skip]
    fn test_decode_branch() {
        assert_eq!(decode(0x7bd583e3).unwrap(), Instr::Branch(BType { rs1: Reg::new(11), rs2: Reg::new(29), imm: 4006  }, BranchFunct::Eq));
        assert_eq!(decode(0xb44d9f63).unwrap(), Instr::Branch(BType { rs1: Reg::new(27), rs2: Reg::new( 4), imm: -3234 }, BranchFunct::Ne));
        assert_eq!(decode(0x39144763).unwrap(), Instr::Branch(BType { rs1: Reg::new( 8), rs2: Reg::new(17), imm: 910   }, BranchFunct::Lt));
        assert_eq!(decode(0xa8b154e3).unwrap(), Instr::Branch(BType { rs1: Reg::new( 2), rs2: Reg::new(11), imm: -1400 }, BranchFunct::Ge));
        assert_eq!(decode(0x16c3ea63).unwrap(), Instr::Branch(BType { rs1: Reg::new( 7), rs2: Reg::new(12), imm: 372   }, BranchFunct::Ltu));
        assert_eq!(decode(0xc114fae3).unwrap(), Instr::Branch(BType { rs1: Reg::new( 9), rs2: Reg::new(17), imm: -1004 }, BranchFunct::Geu));
    }

    #[test]
    fn test_decode_others() {
        assert_eq!(
            decode(0x54f71fb7).unwrap(),
            Instr::Lui(UType {
                rd: Reg::new(31),
                imm: 0x54f71000u32 as i32
            })
        );
        assert_eq!(
            decode(0xd5dadc17).unwrap(),
            Instr::Auipc(UType {
                rd: Reg::new(24),
                imm: 0xd5dad000u32 as i32
            })
        );
        assert_eq!(
            decode(0xed16326f).unwrap(),
            Instr::Jal(JType {
                rd: Reg::new(4),
                imm: 0xfff63ed0u32 as i32
            })
        );
        assert_eq!(
            decode(0x89158167).unwrap(),
            Instr::Jalr(IType {
                rd: Reg::new(2),
                rs1: Reg::new(11),
                imm: -1903
            })
        );
        assert_eq!(decode(0x0ff0000f).unwrap(), Instr::Fence);
        assert_eq!(decode(0x00000073).unwrap(), Instr::System);
        assert_eq!(decode(0x00100073).unwrap(), Instr::System);
    }

    #[test]
    fn test_decode_illegal() {
        assert_eq!(decode(0x00000000), None);
        assert_eq!(decode(0xffffffff), None);
    }

    #[test]
    #[rustfmt::skip]
    fn test_decode_mul() {
        assert_eq!(decode(0x03138633).unwrap(), Instr::Op(RType { rd: Reg::new(12), rs1: Reg::new( 7), rs2: Reg::new(17)}, OpFunct::M(OpMFunct::Mul)));
        assert_eq!(decode(0x03759db3).unwrap(), Instr::Op(RType { rd: Reg::new(27), rs1: Reg::new(11), rs2: Reg::new(23)}, OpFunct::M(OpMFunct::Mulh)));
        assert_eq!(decode(0x0316a533).unwrap(), Instr::Op(RType { rd: Reg::new(10), rs1: Reg::new(13), rs2: Reg::new(17)}, OpFunct::M(OpMFunct::Mulhsu)));
        assert_eq!(decode(0x028db0b3).unwrap(), Instr::Op(RType { rd: Reg::new( 1), rs1: Reg::new(27), rs2: Reg::new( 8)}, OpFunct::M(OpMFunct::Mulhu)));
        assert_eq!(decode(0x023f4db3).unwrap(), Instr::Op(RType { rd: Reg::new(27), rs1: Reg::new(30), rs2: Reg::new( 3)}, OpFunct::M(OpMFunct::Div)));
        assert_eq!(decode(0x027b54b3).unwrap(), Instr::Op(RType { rd: Reg::new( 9), rs1: Reg::new(22), rs2: Reg::new( 7)}, OpFunct::M(OpMFunct::Divu)));
        assert_eq!(decode(0x02446333).unwrap(), Instr::Op(RType { rd: Reg::new( 6), rs1: Reg::new( 8), rs2: Reg::new( 4)}, OpFunct::M(OpMFunct::Rem)));
        assert_eq!(decode(0x02157933).unwrap(), Instr::Op(RType { rd: Reg::new(18), rs1: Reg::new(10), rs2: Reg::new( 1)}, OpFunct::M(OpMFunct::Remu)));
    }

    #[test]
    #[rustfmt::skip]
    fn test_decode_amo() {
        assert_eq!(decode(0x1008adaf).unwrap(), Instr::Atomic(RType { rd: Reg::new(27), rs1: Reg::new(17), rs2: Reg::new( 0)}, AtomicFunct::LrSc(LrScFunct::Lr)));
        assert_eq!(decode(0x1867ab2f).unwrap(), Instr::Atomic(RType { rd: Reg::new(22), rs1: Reg::new(15), rs2: Reg::new( 6)}, AtomicFunct::LrSc(LrScFunct::Sc)));
        assert_eq!(decode(0x096720af).unwrap(), Instr::Atomic(RType { rd: Reg::new( 1), rs1: Reg::new(14), rs2: Reg::new(22)}, AtomicFunct::Amo(AmoFunct::Swap)));
        assert_eq!(decode(0x0081aeaf).unwrap(), Instr::Atomic(RType { rd: Reg::new(29), rs1: Reg::new( 3), rs2: Reg::new( 8)}, AtomicFunct::Amo(AmoFunct::Add)));
        assert_eq!(decode(0x218525af).unwrap(), Instr::Atomic(RType { rd: Reg::new(11), rs1: Reg::new(10), rs2: Reg::new(24)}, AtomicFunct::Amo(AmoFunct::Xor)));
        assert_eq!(decode(0x408eaaaf).unwrap(), Instr::Atomic(RType { rd: Reg::new(21), rs1: Reg::new(29), rs2: Reg::new( 8)}, AtomicFunct::Amo(AmoFunct::Or)));
        assert_eq!(decode(0x603caa2f).unwrap(), Instr::Atomic(RType { rd: Reg::new(20), rs1: Reg::new(25), rs2: Reg::new( 3)}, AtomicFunct::Amo(AmoFunct::And)));
        assert_eq!(decode(0x812c27af).unwrap(), Instr::Atomic(RType { rd: Reg::new(15), rs1: Reg::new(24), rs2: Reg::new(18)}, AtomicFunct::Amo(AmoFunct::Min)));
        assert_eq!(decode(0xa1d2a3af).unwrap(), Instr::Atomic(RType { rd: Reg::new( 7), rs1: Reg::new( 5), rs2: Reg::new(29)}, AtomicFunct::Amo(AmoFunct::Max)));
        assert_eq!(decode(0xc1892f2f).unwrap(), Instr::Atomic(RType { rd: Reg::new(30), rs1: Reg::new(18), rs2: Reg::new(24)}, AtomicFunct::Amo(AmoFunct::Minu)));
        assert_eq!(decode(0xe0512daf).unwrap(), Instr::Atomic(RType { rd: Reg::new(27), rs1: Reg::new( 2), rs2: Reg::new( 5)}, AtomicFunct::Amo(AmoFunct::Maxu)));
    }
}
