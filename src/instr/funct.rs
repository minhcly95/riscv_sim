const FUNCT3_MASK: u32 = (1 << 3) - 1;
const FUNCT7_MASK: u32 = (1 << 7) - 1;

enum Funct7 {
    Zero,
    One,
    Set5,
}

fn funct3(code: u32) -> u8 {
    ((code >> 12) & FUNCT3_MASK) as u8
}

fn funct7(code: u32) -> Option<Funct7> {
    let f7 = (code >> 25) & FUNCT7_MASK;
    match f7 {
        0b0000000 => Some(Funct7::Zero),
        0b0000001 => Some(Funct7::One),
        0b0100000 => Some(Funct7::Set5),
        _ => None,
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum OpFunct {
    I(OpIFunct),
    M(OpMFunct),
}

#[derive(Debug, PartialEq, Eq)]
pub enum OpIFunct {
    Add,
    Sub,
    Slt,
    Sltu,
    And,
    Or,
    Xor,
    Sll,
    Srl,
    Sra,
}

#[derive(Debug, PartialEq, Eq)]
pub enum OpMFunct {
    Mul,
    Mulh,
    Mulhsu,
    Mulhu,
    Div,
    Divu,
    Rem,
    Remu,
}

#[derive(Debug, PartialEq, Eq)]
pub enum OpImmFunct {
    Add,
    Slt,
    Sltu,
    And,
    Or,
    Xor,
    Sll,
    Srl,
    Sra,
}

#[derive(Debug, PartialEq, Eq)]
pub enum LoadFunct {
    B,
    H,
    W,
    Bu,
    Hu,
}

#[derive(Debug, PartialEq, Eq)]
pub enum StoreFunct {
    B,
    H,
    W,
}

#[derive(Debug, PartialEq, Eq)]
pub enum BranchFunct {
    Eq,
    Ne,
    Lt,
    Ge,
    Ltu,
    Geu,
}

impl OpFunct {
    pub fn from(code: u32) -> Option<OpFunct> {
        let f3 = funct3(code);
        let f7 = funct7(code)?;
        match (f3, f7) {
            // OpI
            (0b000, Funct7::Zero) => Some(OpFunct::I(OpIFunct::Add)),
            (0b000, Funct7::Set5) => Some(OpFunct::I(OpIFunct::Sub)),
            (0b001, Funct7::Zero) => Some(OpFunct::I(OpIFunct::Sll)),
            (0b010, Funct7::Zero) => Some(OpFunct::I(OpIFunct::Slt)),
            (0b011, Funct7::Zero) => Some(OpFunct::I(OpIFunct::Sltu)),
            (0b100, Funct7::Zero) => Some(OpFunct::I(OpIFunct::Xor)),
            (0b101, Funct7::Zero) => Some(OpFunct::I(OpIFunct::Srl)),
            (0b101, Funct7::Set5) => Some(OpFunct::I(OpIFunct::Sra)),
            (0b110, Funct7::Zero) => Some(OpFunct::I(OpIFunct::Or)),
            (0b111, Funct7::Zero) => Some(OpFunct::I(OpIFunct::And)),
            // OpM
            (0b000, Funct7::One) => Some(OpFunct::M(OpMFunct::Mul)),
            (0b001, Funct7::One) => Some(OpFunct::M(OpMFunct::Mulh)),
            (0b010, Funct7::One) => Some(OpFunct::M(OpMFunct::Mulhsu)),
            (0b011, Funct7::One) => Some(OpFunct::M(OpMFunct::Mulhu)),
            (0b100, Funct7::One) => Some(OpFunct::M(OpMFunct::Div)),
            (0b101, Funct7::One) => Some(OpFunct::M(OpMFunct::Divu)),
            (0b110, Funct7::One) => Some(OpFunct::M(OpMFunct::Rem)),
            (0b111, Funct7::One) => Some(OpFunct::M(OpMFunct::Remu)),
            _ => None,
        }
    }
}

impl OpImmFunct {
    pub fn from(code: u32) -> Option<OpImmFunct> {
        let f3 = funct3(code);
        let f7 = funct7(code);
        match (f3, f7) {
            (0b000, _) => Some(OpImmFunct::Add),
            (0b001, Some(Funct7::Zero)) => Some(OpImmFunct::Sll),
            (0b010, _) => Some(OpImmFunct::Slt),
            (0b011, _) => Some(OpImmFunct::Sltu),
            (0b100, _) => Some(OpImmFunct::Xor),
            (0b101, Some(Funct7::Zero)) => Some(OpImmFunct::Srl),
            (0b101, Some(Funct7::Set5)) => Some(OpImmFunct::Sra),
            (0b110, _) => Some(OpImmFunct::Or),
            (0b111, _) => Some(OpImmFunct::And),
            _ => None,
        }
    }
}

impl LoadFunct {
    pub fn from(code: u32) -> Option<LoadFunct> {
        let f3 = funct3(code);
        match f3 {
            0b000 => Some(LoadFunct::B),
            0b001 => Some(LoadFunct::H),
            0b010 => Some(LoadFunct::W),
            0b100 => Some(LoadFunct::Bu),
            0b101 => Some(LoadFunct::Hu),
            _ => None,
        }
    }
}

impl StoreFunct {
    pub fn from(code: u32) -> Option<StoreFunct> {
        let f3 = funct3(code);
        match f3 {
            0b000 => Some(StoreFunct::B),
            0b001 => Some(StoreFunct::H),
            0b010 => Some(StoreFunct::W),
            _ => None,
        }
    }
}

impl BranchFunct {
    pub fn from(code: u32) -> Option<BranchFunct> {
        let f3 = funct3(code);
        match f3 {
            0b000 => Some(BranchFunct::Eq),
            0b001 => Some(BranchFunct::Ne),
            0b100 => Some(BranchFunct::Lt),
            0b101 => Some(BranchFunct::Ge),
            0b110 => Some(BranchFunct::Ltu),
            0b111 => Some(BranchFunct::Geu),
            _ => None,
        }
    }
}
