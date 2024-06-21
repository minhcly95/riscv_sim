use std::fmt::{Debug, Display};

const REG_MASK: u32 = (1 << 5) - 1;
const DISPLAY_NAMES: [&str; 32] = [
    "zero", "ra", "sp", "gp", "tp", "t0", "t1", "t2", "s0", "s1", "a0", "a1", "a2", "a3", "a4",
    "a5", "a6", "a7", "s2", "s3", "s4", "s5", "s6", "s7", "s8", "s9", "s10", "s11", "t3", "t4",
    "t5", "t6"
];

#[derive(PartialEq, Eq)]
pub struct Reg {
    index: u8,
}

impl Reg {
    pub fn new(index: u8) -> Reg {
        assert!(index < 32, "Register index must be less than 32");
        Reg { index }
    }

    pub unsafe fn unchecked_new(index: u8) -> Reg {
        Reg { index }
    }

    pub fn extract_rd(code: u32) -> Reg {
        unsafe { Reg::unchecked_new(((code >> 7) & REG_MASK) as u8) }
    }

    pub fn extract_rs1(code: u32) -> Reg {
        unsafe { Reg::unchecked_new(((code >> 15) & REG_MASK) as u8) }
    }

    pub fn extract_rs2(code: u32) -> Reg {
        unsafe { Reg::unchecked_new(((code >> 20) & REG_MASK) as u8) }
    }

    pub fn index(&self) -> u8 {
        self.index
    }
}

impl Display for Reg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", DISPLAY_NAMES[self.index as usize])
    }
}

impl Debug for Reg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} (x{})", DISPLAY_NAMES[self.index as usize], self.index)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_in_range() {
        Reg::new(0);
        Reg::new(1);
        Reg::new(15);
        Reg::new(31);
    }

    #[test]
    #[should_panic]
    fn test_new_32() {
        Reg::new(32);
    }

    #[test]
    #[should_panic]
    fn test_new_255() {
        Reg::new(255);
    }

    #[test]
    fn test_extract_rd() {
        assert_eq!(
            Reg::extract_rd(0b1001001_00101_01110_111_01111_0110011).index(),
            0b01111
        );
        assert_eq!(
            Reg::extract_rd(0b1000000_10110_00110_100_01001_1001110).index(),
            0b01001
        );
        assert_eq!(
            Reg::extract_rd(0b1111010_10000_01111_000_11001_0001000).index(),
            0b11001
        );
        assert_eq!(
            Reg::extract_rd(0b0001011_11000_10001_001_01010_0001000).index(),
            0b01010
        );
        assert_eq!(
            Reg::extract_rd(0b1100101_11111_11010_010_11101_0111010).index(),
            0b11101
        );
    }

    #[test]
    fn test_extract_rs1() {
        assert_eq!(
            Reg::extract_rs1(0b1001001_00101_01110_111_01111_0110011).index(),
            0b01110
        );
        assert_eq!(
            Reg::extract_rs1(0b1000000_10110_00110_100_01001_1001110).index(),
            0b00110
        );
        assert_eq!(
            Reg::extract_rs1(0b1111010_10000_01111_000_11001_0001000).index(),
            0b01111
        );
        assert_eq!(
            Reg::extract_rs1(0b0001011_11000_10001_001_01010_0001000).index(),
            0b10001
        );
        assert_eq!(
            Reg::extract_rs1(0b1100101_11111_11010_010_11101_0111010).index(),
            0b11010
        );
    }

    #[test]
    fn test_extract_rs2() {
        assert_eq!(
            Reg::extract_rs2(0b1001001_00101_01110_111_01111_0110011).index(),
            0b00101
        );
        assert_eq!(
            Reg::extract_rs2(0b1000000_10110_00110_100_01001_1001110).index(),
            0b10110
        );
        assert_eq!(
            Reg::extract_rs2(0b1111010_10000_01111_000_11001_0001000).index(),
            0b10000
        );
        assert_eq!(
            Reg::extract_rs2(0b0001011_11000_10001_001_01010_0001000).index(),
            0b11000
        );
        assert_eq!(
            Reg::extract_rs2(0b1100101_11111_11010_010_11101_0111010).index(),
            0b11111
        );
    }
}
