use super::ram::Ram;
use crate::{Exception::*, Result16E, Result32E, Result8E, ResultE};

#[derive(Debug)]
pub struct MemMap {
    pub ram_base: u64,
    pub ram: Ram,
    reserved_word: Option<u64>, // For atomic lr/sc
}

impl MemMap {
    pub fn new(ram_size: usize) -> MemMap {
        let ram = Ram::new(ram_size);
        MemMap {
            ram_base: 0,
            ram,
            reserved_word: None,
        }
    }

    fn check_valid(&self, addr: u64) -> bool {
        let ram_end = self.ram_base.wrapping_add(self.ram.size());
        (self.ram_base..ram_end).contains(&addr)
    }

    // Read
    pub fn read_u8(&self, addr: u64) -> Result8E {
        if !self.check_valid(addr) {
            Err(LoadAccessFault)
        } else {
            let addr = addr.wrapping_sub(self.ram_base) as usize;
            let buf = self.ram.as_u8();
            Ok(buf[addr as usize])
        }
    }

    pub fn read_u16(&self, addr: u64) -> Result16E {
        if !self.check_valid(addr) {
            Err(LoadAccessFault)
        } else if addr & 0b1 != 0 {
            Err(LoadAddrMisaligned)
        } else {
            let addr = addr.wrapping_sub(self.ram_base) as usize;
            let buf = self.ram.as_u8();
            Ok(u16::from_le_bytes([buf[addr], buf[addr + 1]]))
        }
    }

    pub fn read_u32(&self, addr: u64) -> Result32E {
        if !self.check_valid(addr) {
            Err(LoadAccessFault)
        } else if addr & 0b11 != 0 {
            Err(LoadAddrMisaligned)
        } else {
            let addr = addr.wrapping_sub(self.ram_base) as usize;
            let buf = self.ram.as_u8();
            Ok(u32::from_le_bytes([
                buf[addr],
                buf[addr + 1],
                buf[addr + 2],
                buf[addr + 3],
            ]))
        }
    }

    // Write (also clear reservation when needed)
    pub fn write_u8(&mut self, addr: u64, val: u8) -> ResultE {
        if !self.check_valid(addr) {
            Err(StoreAccessFault)
        } else {
            self.clear_reservation_if_matched(addr);
            let addr = addr.wrapping_sub(self.ram_base) as usize;
            let buf = self.ram.as_u8_mut();
            buf[addr] = val;
            Ok(())
        }
    }

    pub fn write_u16(&mut self, addr: u64, val: u16) -> ResultE {
        if !self.check_valid(addr) {
            Err(StoreAccessFault)
        } else if addr & 0b1 != 0 {
            Err(StoreAddrMisaligned)
        } else {
            self.clear_reservation_if_matched(addr);
            let addr = addr.wrapping_sub(self.ram_base) as usize;
            let buf = self.ram.as_u8_mut();
            let bytes = val.to_le_bytes();
            buf[addr] = bytes[0];
            buf[addr + 1] = bytes[1];
            Ok(())
        }
    }

    pub fn write_u32(&mut self, addr: u64, val: u32) -> ResultE {
        println!("Write to {addr:09x} with {val:08x}");
        if !self.check_valid(addr) {
            Err(StoreAccessFault)
        } else if addr & 0b11 != 0 {
            Err(StoreAddrMisaligned)
        } else {
            self.clear_reservation_if_matched(addr);
            let addr = addr.wrapping_sub(self.ram_base) as usize;
            let buf = self.ram.as_u8_mut();
            let bytes = val.to_le_bytes();
            buf[addr] = bytes[0];
            buf[addr + 1] = bytes[1];
            buf[addr + 2] = bytes[2];
            buf[addr + 3] = bytes[3];
            Ok(())
        }
    }

    pub fn check_write_u32(&mut self, addr: u64) -> ResultE {
        if !self.check_valid(addr) {
            Err(StoreAccessFault)
        } else if addr & 0b11 != 0 {
            Err(StoreAddrMisaligned)
        } else {
            Ok(())
        }
    }

    // Instruction fetch
    pub fn fetch(&self, addr: u64) -> Result32E {
        if !self.check_valid(addr) {
            Err(InstrAccessFault)
        } else if addr & 0b11 != 0 {
            Err(InstrAddrMisaligned)
        } else {
            let addr = addr.wrapping_sub(self.ram_base) as usize;
            let buf = self.ram.as_u8();
            Ok(u32::from_le_bytes([
                buf[addr],
                buf[addr + 1],
                buf[addr + 2],
                buf[addr + 3],
            ]))
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

#[cfg(test)]
mod tests {
    use super::*;
    use rand::{self, Rng};

    const MEM_SIZE: u64 = 0x400;
    const MASK: u64 = !0b11;

    #[test]
    fn test_u8_write() {
        let mut mem = MemMap::new(MEM_SIZE as usize); // 1 kB

        for _ in 0..10 {
            let addr = rand::thread_rng().gen_range(0..MEM_SIZE) & MASK;
            let data: u8 = rand::random();
            mem.write_u8(addr, data).unwrap();
            assert_eq!(mem.read_u8(addr).unwrap(), data);
        }
    }

    #[test]
    fn test_u16_write() {
        let mut mem = MemMap::new(MEM_SIZE as usize); // 1 kB

        for _ in 0..10 {
            let addr = rand::thread_rng().gen_range(0..MEM_SIZE) & MASK;
            let data: u16 = rand::random();
            mem.write_u16(addr, data).unwrap();
            let bytes = data.to_le_bytes();
            assert_eq!(mem.read_u8(addr).unwrap(), bytes[0]);
            assert_eq!(mem.read_u8(addr + 1).unwrap(), bytes[1]);
        }
    }

    #[test]
    fn test_u32_write() {
        let mut mem = MemMap::new(MEM_SIZE as usize); // 1 kB

        for _ in 0..10 {
            let addr = rand::thread_rng().gen_range(0..MEM_SIZE) & MASK;
            let data: u32 = rand::random();
            mem.write_u32(addr, data).unwrap();
            let bytes = data.to_le_bytes();
            assert_eq!(mem.read_u8(addr).unwrap(), bytes[0]);
            assert_eq!(mem.read_u8(addr + 1).unwrap(), bytes[1]);
            assert_eq!(mem.read_u8(addr + 2).unwrap(), bytes[2]);
            assert_eq!(mem.read_u8(addr + 3).unwrap(), bytes[3]);
        }
    }

    #[test]
    fn test_u16_read() {
        let mut mem = MemMap::new(MEM_SIZE as usize); // 1 kB

        for _ in 0..10 {
            let addr = rand::thread_rng().gen_range(0..MEM_SIZE) & MASK;
            let data: u16 = rand::random();
            let bytes = data.to_le_bytes();
            mem.write_u8(addr, bytes[0]).unwrap();
            mem.write_u8(addr + 1, bytes[1]).unwrap();
            assert_eq!(mem.read_u16(addr).unwrap(), data);
        }
    }

    #[test]
    fn test_u32_read() {
        let mut mem = MemMap::new(MEM_SIZE as usize); // 1 kB

        for _ in 0..10 {
            let addr = rand::thread_rng().gen_range(0..MEM_SIZE) & MASK;
            let data: u32 = rand::random();
            let bytes = data.to_le_bytes();
            mem.write_u8(addr, bytes[0]).unwrap();
            mem.write_u8(addr + 1, bytes[1]).unwrap();
            mem.write_u8(addr + 2, bytes[2]).unwrap();
            mem.write_u8(addr + 3, bytes[3]).unwrap();
            assert_eq!(mem.read_u32(addr).unwrap(), data);
        }
    }

    #[test]
    #[rustfmt::skip]
    fn test_read_fault() {
        let mem = MemMap::new(MEM_SIZE as usize); // 1 kB

        assert_eq!(mem.read_u8(MEM_SIZE).unwrap_err(), LoadAccessFault);
        assert_eq!(mem.read_u8(0xffff_ffff).unwrap_err(), LoadAccessFault);

        assert_eq!(mem.read_u16(MEM_SIZE).unwrap_err(), LoadAccessFault);
        assert_eq!(mem.read_u16(0xffff_ffff).unwrap_err(), LoadAccessFault);

        assert_eq!(mem.read_u32(MEM_SIZE).unwrap_err(), LoadAccessFault);
        assert_eq!(mem.read_u32(0xffff_ffff).unwrap_err(), LoadAccessFault);
    }

    #[test]
    #[rustfmt::skip]
    fn test_read_misaligned() {
        let mem = MemMap::new(MEM_SIZE as usize); // 1 kB

        assert_eq!(mem.read_u16(1).unwrap_err(), LoadAddrMisaligned);
        assert_eq!(mem.read_u16(3).unwrap_err(), LoadAddrMisaligned);

        assert_eq!(mem.read_u16(MEM_SIZE + 1).unwrap_err(), LoadAccessFault);
        assert_eq!(mem.read_u16(MEM_SIZE + 3).unwrap_err(), LoadAccessFault);

        assert_eq!(mem.read_u32(1).unwrap_err(), LoadAddrMisaligned);
        assert_eq!(mem.read_u32(2).unwrap_err(), LoadAddrMisaligned);
        assert_eq!(mem.read_u32(3).unwrap_err(), LoadAddrMisaligned);

        assert_eq!(mem.read_u32(MEM_SIZE + 1).unwrap_err(), LoadAccessFault);
        assert_eq!(mem.read_u32(MEM_SIZE + 2).unwrap_err(), LoadAccessFault);
        assert_eq!(mem.read_u32(MEM_SIZE + 3).unwrap_err(), LoadAccessFault);
    }

    #[test]
    #[rustfmt::skip]
    fn test_write_fault() {
        let mut mem = MemMap::new(MEM_SIZE as usize); // 1 kB

        assert_eq!(mem.write_u8(MEM_SIZE, 0).unwrap_err(), StoreAccessFault);
        assert_eq!(mem.write_u8(0xffff_ffff, 0).unwrap_err(), StoreAccessFault);

        assert_eq!(mem.write_u16(MEM_SIZE, 0).unwrap_err(), StoreAccessFault);
        assert_eq!(mem.write_u16(0xffff_ffff, 0).unwrap_err(), StoreAccessFault);

        assert_eq!(mem.write_u32(MEM_SIZE, 0).unwrap_err(), StoreAccessFault);
        assert_eq!(mem.write_u32(0xffff_ffff, 0).unwrap_err(), StoreAccessFault);
    }

    #[test]
    #[rustfmt::skip]
    fn test_write_misaligned() {
        let mut mem = MemMap::new(MEM_SIZE as usize); // 1 kB

        assert_eq!(mem.write_u16(1, 0).unwrap_err(), StoreAddrMisaligned);
        assert_eq!(mem.write_u16(3, 0).unwrap_err(), StoreAddrMisaligned);

        assert_eq!(mem.write_u16(MEM_SIZE + 1, 0).unwrap_err(), StoreAccessFault);
        assert_eq!(mem.write_u16(MEM_SIZE + 3, 0).unwrap_err(), StoreAccessFault);

        assert_eq!(mem.write_u32(1, 0).unwrap_err(), StoreAddrMisaligned);
        assert_eq!(mem.write_u32(2, 0).unwrap_err(), StoreAddrMisaligned);
        assert_eq!(mem.write_u32(3, 0).unwrap_err(), StoreAddrMisaligned);

        assert_eq!(mem.write_u32(MEM_SIZE + 1, 0).unwrap_err(), StoreAccessFault);
        assert_eq!(mem.write_u32(MEM_SIZE + 2, 0).unwrap_err(), StoreAccessFault);
        assert_eq!(mem.write_u32(MEM_SIZE + 3, 0).unwrap_err(), StoreAccessFault);
    }

    #[test]
    #[rustfmt::skip]
    fn test_exec_fault() {
        let mem = MemMap::new(MEM_SIZE as usize); // 1 kB

        assert_eq!(mem.fetch(MEM_SIZE).unwrap_err(), InstrAccessFault);
        assert_eq!(mem.fetch(0xffff_ffff).unwrap_err(), InstrAccessFault);
    }

    #[test]
    #[rustfmt::skip]
    fn test_exec_misaligned() {
        let mem = MemMap::new(MEM_SIZE as usize); // 1 kB

        assert_eq!(mem.fetch(1).unwrap_err(), InstrAddrMisaligned);
        assert_eq!(mem.fetch(2).unwrap_err(), InstrAddrMisaligned);
        assert_eq!(mem.fetch(3).unwrap_err(), InstrAddrMisaligned);

        assert_eq!(mem.fetch(MEM_SIZE + 1).unwrap_err(), InstrAccessFault);
        assert_eq!(mem.fetch(MEM_SIZE + 2).unwrap_err(), InstrAccessFault);
        assert_eq!(mem.fetch(MEM_SIZE + 3).unwrap_err(), InstrAccessFault);
    }

    fn assert_word_reserved(mem: &MemMap, addr: u64, expect: bool) {
        assert_eq!(mem.is_reserved(addr), expect);
        assert_eq!(mem.is_reserved(addr + 1), expect);
        assert_eq!(mem.is_reserved(addr + 2), expect);
        assert_eq!(mem.is_reserved(addr + 3), expect);
    }

    #[test]
    fn test_reserve() {
        let mut mem = MemMap::new(MEM_SIZE as usize); // 1 kB

        for i in 0..=3 {
            mem.reserve(i);
            assert_word_reserved(&mem, 0, true);
            assert!(!mem.is_reserved(4));
            assert!(!mem.is_reserved(u64::MAX));
        }
    }

    #[test]
    fn test_double_reserve() {
        let mut mem = MemMap::new(MEM_SIZE as usize); // 1 kB

        for i in 0..=3 {
            mem.reserve(i);
            mem.reserve(4 + i);
            assert_word_reserved(&mem, 0, false);
            assert_word_reserved(&mem, 4, true);
        }
    }

    #[test]
    fn test_clear_reservation() {
        let mut mem = MemMap::new(MEM_SIZE as usize); // 1 kB

        for i in 0..=3 {
            mem.reserve(i);
            mem.clear_reservation();
            assert_word_reserved(&mem, 0, false);
        }
    }

    #[test]
    fn test_clear_reservation_if_matched() {
        let mut mem = MemMap::new(MEM_SIZE as usize); // 1 kB

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
        let mut mem = MemMap::new(MEM_SIZE as usize); // 1 kB

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
        let mut mem = MemMap::new(MEM_SIZE as usize); // 1 kB

        for i in 0..=3 {
            for j in 0..=3 {
                mem.reserve(i);
                mem.write_u8(j, 0).unwrap();
                assert_word_reserved(&mem, 0, false);
            }
        }
    }

    #[test]
    fn test_clear_reservation_on_write_u16() {
        let mut mem = MemMap::new(MEM_SIZE as usize); // 1 kB

        for i in 0..=3 {
            for j in 0..=1 {
                mem.reserve(i);
                mem.write_u16(2 * j, 0).unwrap();
                assert_word_reserved(&mem, 0, false);
            }
        }
    }

    #[test]
    fn test_clear_reservation_on_write_u32() {
        let mut mem = MemMap::new(MEM_SIZE as usize); // 1 kB

        for i in 0..=3 {
            mem.reserve(i);
            mem.write_u32(0, 0).unwrap();
            assert_word_reserved(&mem, 0, false);
        }
    }

    #[test]
    fn test_keep_reservation_if_write_elsewhere() {
        let mut mem = MemMap::new(MEM_SIZE as usize); // 1 kB

        for i in 0..=3 {
            for j in 0..=3 {
                mem.reserve(i);
                mem.write_u8(4 + j, 0).unwrap();
                assert_word_reserved(&mem, 0, true);
            }
        }
    }
}
