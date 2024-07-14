use super::ram::Ram;
use crate::{
    Exception::{self, *},
    Result16E, Result32E, Result8E, ResultE,
};
use std::ops::Range;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum AccessType {
    Instr,
    Load,
    Store,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum AccessWidth {
    Byte,
    HalfWord,
    Word,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct AccessAttr {
    pub atype: AccessType,
    pub width: AccessWidth,
    pub lrsc: bool,
    pub amo: bool,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum MemTarget {
    Ram(u64),
}

#[derive(Debug)]
pub struct MemMap {
    pub ram: Ram,
    pub ram_range: Range<u64>,
    reserved_word: Option<u64>, // For atomic lr/sc
}

impl MemMap {
    pub fn new(ram_size: u64) -> MemMap {
        let ram = Ram::new(ram_size as usize);
        MemMap {
            ram,
            ram_range: Range {
                start: 0,
                end: ram_size,
            },
            reserved_word: None,
        }
    }

    pub fn check_and_translate(&self, addr: u64, attr: AccessAttr) -> Result<MemTarget, Exception> {
        if !self.ram_range.contains(&addr) {
            return Err(access_fault(attr.atype));
        }
        match attr.width {
            AccessWidth::HalfWord => {
                if addr & 0b1 != 0 {
                    return Err(misaligned_fault(attr.atype));
                }
            }
            AccessWidth::Word => {
                if addr & 0b11 != 0 {
                    return Err(misaligned_fault(attr.atype));
                }
            }
            _ => (),
        }
        Ok(MemTarget::Ram(addr - self.ram_range.start))
    }

    // Read
    pub fn read_u8(&self, addr: u64, attr: AccessAttr) -> Result8E {
        match self.check_and_translate(addr, attr)? {
            MemTarget::Ram(ram_addr) => {
                let buf = self.ram.as_u8();
                Ok(buf[ram_addr as usize])
            }
        }
    }

    pub fn read_u16(&self, addr: u64, attr: AccessAttr) -> Result16E {
        match self.check_and_translate(addr, attr)? {
            MemTarget::Ram(ram_addr) => {
                let ram_addr = ram_addr as usize;
                let buf = self.ram.as_u8();
                Ok(u16::from_le_bytes([buf[ram_addr], buf[ram_addr + 1]]))
            }
        }
    }

    pub fn read_u32(&self, addr: u64, attr: AccessAttr) -> Result32E {
        match self.check_and_translate(addr, attr)? {
            MemTarget::Ram(ram_addr) => {
                let ram_addr = ram_addr as usize;
                let buf = self.ram.as_u8();
                Ok(u32::from_le_bytes([
                    buf[ram_addr],
                    buf[ram_addr + 1],
                    buf[ram_addr + 2],
                    buf[ram_addr + 3],
                ]))
            }
        }
    }

    // Write (also clear reservation when needed)
    pub fn write_u8(&mut self, addr: u64, val: u8, attr: AccessAttr) -> ResultE {
        match self.check_and_translate(addr, attr)? {
            MemTarget::Ram(ram_addr) => {
                self.clear_reservation_if_matched(addr);
                let buf = self.ram.as_u8_mut();
                buf[ram_addr as usize] = val;
                Ok(())
            }
        }
    }

    pub fn write_u16(&mut self, addr: u64, val: u16, attr: AccessAttr) -> ResultE {
        match self.check_and_translate(addr, attr)? {
            MemTarget::Ram(ram_addr) => {
                self.clear_reservation_if_matched(addr);
                let ram_addr = ram_addr as usize;
                let buf = self.ram.as_u8_mut();
                let bytes = val.to_le_bytes();
                buf[ram_addr] = bytes[0];
                buf[ram_addr + 1] = bytes[1];
                Ok(())
            }
        }
    }

    pub fn write_u32(&mut self, addr: u64, val: u32, attr: AccessAttr) -> ResultE {
        match self.check_and_translate(addr, attr)? {
            MemTarget::Ram(ram_addr) => {
                self.clear_reservation_if_matched(addr);
                let ram_addr = ram_addr as usize;
                let buf = self.ram.as_u8_mut();
                let bytes = val.to_le_bytes();
                buf[ram_addr] = bytes[0];
                buf[ram_addr + 1] = bytes[1];
                buf[ram_addr + 2] = bytes[2];
                buf[ram_addr + 3] = bytes[3];
                Ok(())
            }
        }
    }

    // Reservation
    pub fn reserve(&mut self, addr: u64) {
        self.reserved_word = Some(addr >> 2);
    }

    pub fn is_reserved(&self, addr: u64) -> bool {
        self.reserved_word.map_or(false, |word| word == addr >> 2)
    }

    pub fn clear_reservation(&mut self) {
        self.reserved_word = None;
    }

    pub fn clear_reservation_if_matched(&mut self, addr: u64) {
        if let Some(word) = self.reserved_word {
            if word == addr >> 2 {
                self.reserved_word = None;
            }
        }
    }
}

pub fn misaligned_fault(access_type: AccessType) -> Exception {
    match access_type {
        AccessType::Instr => InstrAddrMisaligned,
        AccessType::Load => LoadAddrMisaligned,
        AccessType::Store => StoreAddrMisaligned,
    }
}

pub fn access_fault(access_type: AccessType) -> Exception {
    match access_type {
        AccessType::Instr => InstrAccessFault,
        AccessType::Load => LoadAccessFault,
        AccessType::Store => StoreAccessFault,
    }
}

pub fn page_fault(access_type: AccessType) -> Exception {
    match access_type {
        AccessType::Instr => InstrPageFault,
        AccessType::Load => LoadPageFault,
        AccessType::Store => StorePageFault,
    }
}

#[cfg(test)]
mod tests {
    use super::{AccessWidth::*, *};
    use rand::{self, Rng};

    const MEM_SIZE: u64 = 0x400;
    const MASK: u64 = !0b11;

    fn load_attr(width: AccessWidth) -> AccessAttr {
        AccessAttr {
            width,
            atype: AccessType::Load,
            amo: false,
            lrsc: false,
        }
    }

    fn store_attr(width: AccessWidth) -> AccessAttr {
        AccessAttr {
            width,
            atype: AccessType::Store,
            amo: false,
            lrsc: false,
        }
    }

    fn instr_attr() -> AccessAttr {
        AccessAttr {
            width: AccessWidth::Word,
            atype: AccessType::Instr,
            amo: false,
            lrsc: false,
        }
    }

    #[test]
    fn test_u8_write() {
        let mut mem = MemMap::new(MEM_SIZE); // 1 kB

        for _ in 0..10 {
            let addr = rand::thread_rng().gen_range(0..MEM_SIZE) & MASK;
            let data: u8 = rand::random();
            mem.write_u8(addr, data, store_attr(Byte)).unwrap();
            assert_eq!(mem.read_u8(addr, load_attr(Byte)).unwrap(), data);
        }
    }

    #[test]
    fn test_u16_write() {
        let mut mem = MemMap::new(MEM_SIZE); // 1 kB

        for _ in 0..10 {
            let addr = rand::thread_rng().gen_range(0..MEM_SIZE) & MASK;
            let data: u16 = rand::random();
            mem.write_u16(addr, data, store_attr(HalfWord)).unwrap();
            let bytes = data.to_le_bytes();
            assert_eq!(mem.read_u8(addr, load_attr(Byte)).unwrap(), bytes[0]);
            assert_eq!(mem.read_u8(addr + 1, load_attr(Byte)).unwrap(), bytes[1]);
        }
    }

    #[test]
    fn test_u32_write() {
        let mut mem = MemMap::new(MEM_SIZE); // 1 kB

        for _ in 0..10 {
            let addr = rand::thread_rng().gen_range(0..MEM_SIZE) & MASK;
            let data: u32 = rand::random();
            mem.write_u32(addr, data, store_attr(Word)).unwrap();
            let bytes = data.to_le_bytes();
            assert_eq!(mem.read_u8(addr, load_attr(Byte)).unwrap(), bytes[0]);
            assert_eq!(mem.read_u8(addr + 1, load_attr(Byte)).unwrap(), bytes[1]);
            assert_eq!(mem.read_u8(addr + 2, load_attr(Byte)).unwrap(), bytes[2]);
            assert_eq!(mem.read_u8(addr + 3, load_attr(Byte)).unwrap(), bytes[3]);
        }
    }

    #[test]
    fn test_u16_read() {
        let mut mem = MemMap::new(MEM_SIZE); // 1 kB

        for _ in 0..10 {
            let addr = rand::thread_rng().gen_range(0..MEM_SIZE) & MASK;
            let data: u16 = rand::random();
            let bytes = data.to_le_bytes();
            mem.write_u8(addr, bytes[0], store_attr(Byte)).unwrap();
            mem.write_u8(addr + 1, bytes[1], store_attr(Byte)).unwrap();
            assert_eq!(mem.read_u16(addr, load_attr(HalfWord)).unwrap(), data);
        }
    }

    #[test]
    fn test_u32_read() {
        let mut mem = MemMap::new(MEM_SIZE); // 1 kB

        for _ in 0..10 {
            let addr = rand::thread_rng().gen_range(0..MEM_SIZE) & MASK;
            let data: u32 = rand::random();
            let bytes = data.to_le_bytes();
            mem.write_u8(addr, bytes[0], store_attr(Byte)).unwrap();
            mem.write_u8(addr + 1, bytes[1], store_attr(Byte)).unwrap();
            mem.write_u8(addr + 2, bytes[2], store_attr(Byte)).unwrap();
            mem.write_u8(addr + 3, bytes[3], store_attr(Byte)).unwrap();
            assert_eq!(mem.read_u32(addr, load_attr(Word)).unwrap(), data);
        }
    }

    #[test]
    #[rustfmt::skip]
    fn test_read_fault() {
        let mem = MemMap::new(MEM_SIZE); // 1 kB

        assert_eq!(mem.read_u8(MEM_SIZE, load_attr(Byte)).unwrap_err(), LoadAccessFault);
        assert_eq!(mem.read_u8(0xffff_ffff, load_attr(Byte)).unwrap_err(), LoadAccessFault);

        assert_eq!(mem.read_u16(MEM_SIZE, load_attr(HalfWord)).unwrap_err(), LoadAccessFault);
        assert_eq!(mem.read_u16(0xffff_ffff, load_attr(HalfWord)).unwrap_err(), LoadAccessFault);

        assert_eq!(mem.read_u32(MEM_SIZE, load_attr(Word)).unwrap_err(), LoadAccessFault);
        assert_eq!(mem.read_u32(0xffff_ffff, load_attr(Word)).unwrap_err(), LoadAccessFault);
    }

    #[test]
    #[rustfmt::skip]
    fn test_read_misaligned() {
        let mem = MemMap::new(MEM_SIZE); // 1 kB

        assert_eq!(mem.read_u16(1, load_attr(HalfWord)).unwrap_err(), LoadAddrMisaligned);
        assert_eq!(mem.read_u16(3, load_attr(HalfWord)).unwrap_err(), LoadAddrMisaligned);

        assert_eq!(mem.read_u16(MEM_SIZE + 1, load_attr(HalfWord)).unwrap_err(), LoadAccessFault);
        assert_eq!(mem.read_u16(MEM_SIZE + 3, load_attr(HalfWord)).unwrap_err(), LoadAccessFault);

        assert_eq!(mem.read_u32(1, load_attr(Word)).unwrap_err(), LoadAddrMisaligned);
        assert_eq!(mem.read_u32(2, load_attr(Word)).unwrap_err(), LoadAddrMisaligned);
        assert_eq!(mem.read_u32(3, load_attr(Word)).unwrap_err(), LoadAddrMisaligned);

        assert_eq!(mem.read_u32(MEM_SIZE + 1, load_attr(Word)).unwrap_err(), LoadAccessFault);
        assert_eq!(mem.read_u32(MEM_SIZE + 2, load_attr(Word)).unwrap_err(), LoadAccessFault);
        assert_eq!(mem.read_u32(MEM_SIZE + 3, load_attr(Word)).unwrap_err(), LoadAccessFault);
    }

    #[test]
    #[rustfmt::skip]
    fn test_write_fault() {
        let mut mem = MemMap::new(MEM_SIZE); // 1 kB

        assert_eq!(mem.write_u8(MEM_SIZE, 0, store_attr(Byte)).unwrap_err(), StoreAccessFault);
        assert_eq!(mem.write_u8(0xffff_ffff, 0, store_attr(Byte)).unwrap_err(), StoreAccessFault);

        assert_eq!(mem.write_u16(MEM_SIZE, 0, store_attr(HalfWord)).unwrap_err(), StoreAccessFault);
        assert_eq!(mem.write_u16(0xffff_ffff, 0, store_attr(HalfWord)).unwrap_err(), StoreAccessFault);

        assert_eq!(mem.write_u32(MEM_SIZE, 0, store_attr(Word)).unwrap_err(), StoreAccessFault);
        assert_eq!(mem.write_u32(0xffff_ffff, 0, store_attr(Word)).unwrap_err(), StoreAccessFault);
    }

    #[test]
    #[rustfmt::skip]
    fn test_write_misaligned() {
        let mut mem = MemMap::new(MEM_SIZE); // 1 kB

        assert_eq!(mem.write_u16(1, 0, store_attr(HalfWord)).unwrap_err(), StoreAddrMisaligned);
        assert_eq!(mem.write_u16(3, 0, store_attr(HalfWord)).unwrap_err(), StoreAddrMisaligned);

        assert_eq!(mem.write_u16(MEM_SIZE + 1, 0, store_attr(HalfWord)).unwrap_err(), StoreAccessFault);
        assert_eq!(mem.write_u16(MEM_SIZE + 3, 0, store_attr(HalfWord)).unwrap_err(), StoreAccessFault);

        assert_eq!(mem.write_u32(1, 0, store_attr(Word)).unwrap_err(), StoreAddrMisaligned);
        assert_eq!(mem.write_u32(2, 0, store_attr(Word)).unwrap_err(), StoreAddrMisaligned);
        assert_eq!(mem.write_u32(3, 0, store_attr(Word)).unwrap_err(), StoreAddrMisaligned);

        assert_eq!(mem.write_u32(MEM_SIZE + 1, 0, store_attr(Word)).unwrap_err(), StoreAccessFault);
        assert_eq!(mem.write_u32(MEM_SIZE + 2, 0, store_attr(Word)).unwrap_err(), StoreAccessFault);
        assert_eq!(mem.write_u32(MEM_SIZE + 3, 0, store_attr(Word)).unwrap_err(), StoreAccessFault);
    }

    #[test]
    #[rustfmt::skip]
    fn test_exec_fault() {
        let mem = MemMap::new(MEM_SIZE); // 1 kB

        assert_eq!(mem.read_u32(MEM_SIZE, instr_attr()).unwrap_err(), InstrAccessFault);
        assert_eq!(mem.read_u32(0xffff_ffff, instr_attr()).unwrap_err(), InstrAccessFault);
    }

    #[test]
    #[rustfmt::skip]
    fn test_exec_misaligned() {
        let mem = MemMap::new(MEM_SIZE); // 1 kB

        assert_eq!(mem.read_u32(1, instr_attr()).unwrap_err(), InstrAddrMisaligned);
        assert_eq!(mem.read_u32(2, instr_attr()).unwrap_err(), InstrAddrMisaligned);
        assert_eq!(mem.read_u32(3, instr_attr()).unwrap_err(), InstrAddrMisaligned);

        assert_eq!(mem.read_u32(MEM_SIZE + 1, instr_attr()).unwrap_err(), InstrAccessFault);
        assert_eq!(mem.read_u32(MEM_SIZE + 2, instr_attr()).unwrap_err(), InstrAccessFault);
        assert_eq!(mem.read_u32(MEM_SIZE + 3, instr_attr()).unwrap_err(), InstrAccessFault);
    }

    fn assert_word_reserved(mem: &MemMap, addr: u64, expect: bool) {
        assert_eq!(mem.is_reserved(addr), expect);
        assert_eq!(mem.is_reserved(addr + 1), expect);
        assert_eq!(mem.is_reserved(addr + 2), expect);
        assert_eq!(mem.is_reserved(addr + 3), expect);
    }

    #[test]
    fn test_reserve() {
        let mut mem = MemMap::new(MEM_SIZE); // 1 kB

        for i in 0..=3 {
            mem.reserve(i);
            assert_word_reserved(&mem, 0, true);
            assert!(!mem.is_reserved(4));
            assert!(!mem.is_reserved(u64::MAX));
        }
    }

    #[test]
    fn test_double_reserve() {
        let mut mem = MemMap::new(MEM_SIZE); // 1 kB

        for i in 0..=3 {
            mem.reserve(i);
            mem.reserve(4 + i);
            assert_word_reserved(&mem, 0, false);
            assert_word_reserved(&mem, 4, true);
        }
    }

    #[test]
    fn test_clear_reservation() {
        let mut mem = MemMap::new(MEM_SIZE); // 1 kB

        for i in 0..=3 {
            mem.reserve(i);
            mem.clear_reservation();
            assert_word_reserved(&mem, 0, false);
        }
    }

    #[test]
    fn test_clear_reservation_if_matched() {
        let mut mem = MemMap::new(MEM_SIZE); // 1 kB

        for i in 0..=3 {
            for j in 0..=3 {
                mem.reserve(i);
                mem.clear_reservation_if_matched(j);
                assert_word_reserved(&mem, 0, false);
            }
        }
    }

    #[test]
    fn test_keep_reservation_if_not_matched() {
        let mut mem = MemMap::new(MEM_SIZE); // 1 kB

        for i in 0..=3 {
            for j in 0..=3 {
                mem.reserve(i);
                mem.clear_reservation_if_matched(4 + j);
                assert_word_reserved(&mem, 0, true);
            }
        }
    }

    #[test]
    fn test_clear_reservation_on_write_u8() {
        let mut mem = MemMap::new(MEM_SIZE); // 1 kB

        for i in 0..=3 {
            for j in 0..=3 {
                mem.reserve(i);
                mem.write_u8(j, 0, store_attr(Byte)).unwrap();
                assert_word_reserved(&mem, 0, false);
            }
        }
    }

    #[test]
    fn test_clear_reservation_on_write_u16() {
        let mut mem = MemMap::new(MEM_SIZE); // 1 kB

        for i in 0..=3 {
            for j in 0..=1 {
                mem.reserve(i);
                mem.write_u16(2 * j, 0, store_attr(HalfWord)).unwrap();
                assert_word_reserved(&mem, 0, false);
            }
        }
    }

    #[test]
    fn test_clear_reservation_on_write_u32() {
        let mut mem = MemMap::new(MEM_SIZE); // 1 kB

        for i in 0..=3 {
            mem.reserve(i);
            mem.write_u32(0, 0, store_attr(Word)).unwrap();
            assert_word_reserved(&mem, 0, false);
        }
    }

    #[test]
    fn test_keep_reservation_if_write_elsewhere() {
        let mut mem = MemMap::new(MEM_SIZE); // 1 kB

        for i in 0..=3 {
            for j in 0..=3 {
                mem.reserve(i);
                mem.write_u8(4 + j, 0, store_attr(Byte)).unwrap();
                assert_word_reserved(&mem, 0, true);
            }
        }
    }
}
