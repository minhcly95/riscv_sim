use super::reg::Reg;

#[derive(Debug, PartialEq, Eq)]
pub struct RType {
    pub rd: Reg,
    pub rs1: Reg,
    pub rs2: Reg,
}

#[derive(Debug, PartialEq, Eq)]
pub struct IType {
    pub rd: Reg,
    pub rs1: Reg,
    pub imm: i32,
}

#[derive(Debug, PartialEq, Eq)]
pub struct SType {
    pub rs1: Reg,
    pub rs2: Reg,
    pub imm: i32,
}

#[derive(Debug, PartialEq, Eq)]
pub struct BType {
    pub rs1: Reg,
    pub rs2: Reg,
    pub imm: i32,
}

#[derive(Debug, PartialEq, Eq)]
pub struct UType {
    pub rd: Reg,
    pub imm: i32,
}

#[derive(Debug, PartialEq, Eq)]
pub struct JType {
    pub rd: Reg,
    pub imm: i32,
}

const I_MASK: u32 = (1 << 12) - 1;
const I_SHAMT_MASK: u32 = (1 << 5) - 1;

const S4_0_MASK: u32 = (1 << 5) - 1;
const S11_5_MASK: u32 = ((1 << 7) - 1) << 5;

const B4_1_MASK: u32 = ((1 << 4) - 1) << 1;
const B10_5_MASK: u32 = ((1 << 6) - 1) << 5;
const B11_MASK: u32 = 1 << 11;
const B12_MASK: u32 = 1 << 12;

const U_MASK: u32 = ((1 << 20) - 1) << 12;

const J10_1_MASK: u32 = ((1 << 10) - 1) << 1;
const J11_MASK: u32 = 1 << 11;
const J19_12_MASK: u32 = ((1 << 8) - 1) << 12;
const J20_MASK: u32 = 1 << 20;

fn sign_extend_12(imm: u32) -> i32 {
    (if imm >= 0x800 { imm | 0xfffff_000 } else { imm }) as i32
}

fn sign_extend_13(imm: u32) -> i32 {
    (if imm >= 0x1000 {
        imm | 0xffff_e000
    } else {
        imm
    }) as i32
}

fn sign_extend_21(imm: u32) -> i32 {
    (if imm >= 0x100000 {
        imm | 0xff_e00000
    } else {
        imm
    }) as i32
}

impl RType {
    pub fn from(code: u32) -> RType {
        let rd = Reg::extract_rd(code);
        let rs1 = Reg::extract_rs1(code);
        let rs2 = Reg::extract_rs2(code);
        RType { rd, rs1, rs2 }
    }
}

impl IType {
    pub fn from(code: u32) -> IType {
        let rd = Reg::extract_rd(code);
        let rs1 = Reg::extract_rs1(code);
        let imm = (code >> 20) & I_MASK;
        let imm = sign_extend_12(imm);
        IType { rd, rs1, imm }
    }

    pub fn from_shamt(code: u32) -> IType {
        let rd = Reg::extract_rd(code);
        let rs1 = Reg::extract_rs1(code);
        let imm = ((code >> 20) & I_SHAMT_MASK) as i32;
        IType { rd, rs1, imm }
    }
}

impl SType {
    pub fn from(code: u32) -> SType {
        let rs1 = Reg::extract_rs1(code);
        let rs2 = Reg::extract_rs2(code);
        let imm = (code >> 7) & S4_0_MASK | (code >> 20) & S11_5_MASK;
        let imm = sign_extend_12(imm);
        SType { rs1, rs2, imm }
    }
}

impl BType {
    pub fn from(code: u32) -> BType {
        let rs1 = Reg::extract_rs1(code);
        let rs2 = Reg::extract_rs2(code);
        let imm = (code >> 7) & B4_1_MASK
            | (code >> 20) & B10_5_MASK
            | (code << 4) & B11_MASK
            | (code >> 19) & B12_MASK;
        let imm = sign_extend_13(imm);
        BType { rs1, rs2, imm }
    }
}

impl UType {
    pub fn from(code: u32) -> UType {
        let rd = Reg::extract_rd(code);
        let imm = (code & U_MASK) as i32;
        UType { rd, imm }
    }
}

impl JType {
    pub fn from(code: u32) -> JType {
        let rd = Reg::extract_rd(code);
        let imm = (code >> 20) & J10_1_MASK
            | (code >> 9) & J11_MASK
            | code & J19_12_MASK
            | (code >> 11) & J20_MASK;
        let imm = sign_extend_21(imm);
        JType { rd, imm }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_r_type(r_type: RType, rd: u8, rs1: u8, rs2: u8) {
        let rd = Reg::new(rd);
        let rs1 = Reg::new(rs1);
        let rs2 = Reg::new(rs2);
        assert_eq!(r_type, RType { rd, rs1, rs2 });
    }

    fn assert_i_type(i_type: IType, rd: u8, rs1: u8, imm: i32) {
        let rd = Reg::new(rd);
        let rs1 = Reg::new(rs1);
        assert_eq!(i_type, IType { rd, rs1, imm });
    }

    fn assert_s_type(s_type: SType, rs1: u8, rs2: u8, imm: i32) {
        let rs1 = Reg::new(rs1);
        let rs2 = Reg::new(rs2);
        assert_eq!(s_type, SType { rs1, rs2, imm });
    }

    fn assert_b_type(b_type: BType, rs1: u8, rs2: u8, imm: i32) {
        let rs1 = Reg::new(rs1);
        let rs2 = Reg::new(rs2);
        assert_eq!(b_type, BType { rs1, rs2, imm });
    }

    fn assert_u_type(u_type: UType, rd: u8, imm: i32) {
        let rd = Reg::new(rd);
        assert_eq!(u_type, UType { rd, imm });
    }

    fn assert_j_type(j_type: JType, rd: u8, imm: i32) {
        let rd = Reg::new(rd);
        assert_eq!(j_type, JType { rd, imm });
    }

    #[test]
    #[rustfmt::skip]
    fn test_decode_r_type() {
        assert_r_type(RType::from(0b1001010_00101_01110_111_01111_0110011), 0b01111, 0b01110, 0b00101);
        assert_r_type(RType::from(0b1000000_10110_00110_100_01001_1001110), 0b01001, 0b00110, 0b10110);
        assert_r_type(RType::from(0b1111010_10000_01111_000_11001_0001000), 0b11001, 0b01111, 0b10000);
        assert_r_type(RType::from(0b0001011_11000_10001_001_01010_0001000), 0b01010, 0b10001, 0b11000);
        assert_r_type(RType::from(0b1100101_11111_11010_010_11101_0111010), 0b11101, 0b11010, 0b11111);
    }

    #[test]
    #[rustfmt::skip]
    fn test_decode_i_type() {
        assert_i_type(IType::from(0b1001001_00101_01110_111_01111_0110011), 0b01111, 0b01110, 0b11111111_11111111_1111_1001001_00101u32 as i32);
        assert_i_type(IType::from(0b1000000_10110_00110_100_01001_1001110), 0b01001, 0b00110, 0b11111111_11111111_1111_1000000_10110u32 as i32);
        assert_i_type(IType::from(0b1111010_10000_01111_000_11001_0001000), 0b11001, 0b01111, 0b11111111_11111111_1111_1111010_10000u32 as i32);
        assert_i_type(IType::from(0b0001011_11000_10001_001_01010_0001000), 0b01010, 0b10001, 0b00000000_00000000_0000_0001011_11000u32 as i32);
        assert_i_type(IType::from(0b1100101_11111_11010_010_11101_0111010), 0b11101, 0b11010, 0b11111111_11111111_1111_1100101_11111u32 as i32);
    }

    #[test]
    #[rustfmt::skip]
    fn test_decode_i_shamt() {
        assert_i_type(IType::from_shamt(0b1001001_00101_01110_111_01111_0110011), 0b01111, 0b01110, 0b00101);
        assert_i_type(IType::from_shamt(0b1000000_10110_00110_100_01001_1001110), 0b01001, 0b00110, 0b10110);
        assert_i_type(IType::from_shamt(0b1111010_10000_01111_000_11001_0001000), 0b11001, 0b01111, 0b10000);
        assert_i_type(IType::from_shamt(0b0001011_11000_10001_001_01010_0001000), 0b01010, 0b10001, 0b11000);
        assert_i_type(IType::from_shamt(0b1100101_11111_11010_010_11101_0111010), 0b11101, 0b11010, 0b11111);
    }

    #[test]
    #[rustfmt::skip]
    fn test_decode_s_type() {
        assert_s_type(SType::from(0b1001001_00101_01110_111_01111_0110011), 0b01110, 0b00101, 0b11111111_11111111_1111_1001001_01111u32 as i32);
        assert_s_type(SType::from(0b1000000_10110_00110_100_01001_1001110), 0b00110, 0b10110, 0b11111111_11111111_1111_1000000_01001u32 as i32);
        assert_s_type(SType::from(0b1111010_10000_01111_000_11001_0001000), 0b01111, 0b10000, 0b11111111_11111111_1111_1111010_11001u32 as i32);
        assert_s_type(SType::from(0b0001011_11000_10001_001_01010_0001000), 0b10001, 0b11000, 0b00000000_00000000_0000_0001011_01010u32 as i32);
        assert_s_type(SType::from(0b1100101_11111_11010_010_11101_0111010), 0b11010, 0b11111, 0b11111111_11111111_1111_1100101_11101u32 as i32);
    }

    #[test]
    #[rustfmt::skip]
    fn test_decode_b_type() {
        assert_b_type(BType::from(0b1001001_00101_01110_111_01111_0110011), 0b01110, 0b00101, 0b11111111_11111111_111_1_1_001001_0111_0u32 as i32);
        assert_b_type(BType::from(0b1000000_10110_00110_100_01001_1001110), 0b00110, 0b10110, 0b11111111_11111111_111_1_1_000000_0100_0u32 as i32);
        assert_b_type(BType::from(0b1111010_10000_01111_000_11001_0001000), 0b01111, 0b10000, 0b11111111_11111111_111_1_1_111010_1100_0u32 as i32);
        assert_b_type(BType::from(0b0001011_11000_10001_001_01010_0001000), 0b10001, 0b11000, 0b00000000_00000000_000_0_0_001011_0101_0u32 as i32);
        assert_b_type(BType::from(0b1100101_11111_11010_010_11101_0111010), 0b11010, 0b11111, 0b11111111_11111111_111_1_1_100101_1110_0u32 as i32);
    }

    #[test]
    #[rustfmt::skip]
    fn test_decode_u_type() {
        assert_u_type(UType::from(0b1001001_00101_01110_111_01111_0110011), 0b01111, 0b1001001_00101_01110_111_0000_00000000u32 as i32);
        assert_u_type(UType::from(0b1000000_10110_00110_100_01001_1001110), 0b01001, 0b1000000_10110_00110_100_0000_00000000u32 as i32);
        assert_u_type(UType::from(0b1111010_10000_01111_000_11001_0001000), 0b11001, 0b1111010_10000_01111_000_0000_00000000u32 as i32);
        assert_u_type(UType::from(0b0001011_11000_10001_001_01010_0001000), 0b01010, 0b0001011_11000_10001_001_0000_00000000u32 as i32);
        assert_u_type(UType::from(0b1100101_11111_11010_010_11101_0111010), 0b11101, 0b1100101_11111_11010_010_0000_00000000u32 as i32);
    }

    #[test]
    #[rustfmt::skip]
    fn test_decode_j_type() {
        assert_j_type(JType::from(0b1001001_00101_01110_111_01111_0110011), 0b01111, 0b11111111_111_1_01110_111_1_001001_0010_0u32 as i32);
        assert_j_type(JType::from(0b1000000_10110_00110_100_01001_1001110), 0b01001, 0b11111111_111_1_00110_100_0_000000_1011_0u32 as i32);
        assert_j_type(JType::from(0b1111010_10000_01111_000_11001_0001000), 0b11001, 0b11111111_111_1_01111_000_0_111010_1000_0u32 as i32);
        assert_j_type(JType::from(0b0001011_11000_10001_001_01010_0001000), 0b01010, 0b00000000_000_0_10001_001_0_001011_1100_0u32 as i32);
        assert_j_type(JType::from(0b1100101_11111_11010_010_11101_0111010), 0b11101, 0b11111111_111_1_11010_010_1_100101_1111_0u32 as i32);
    }
}
