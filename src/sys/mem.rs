use std::fmt::Debug;

use crate::{Exception::*, Result, Result16, Result32, Result8, Trap};

pub struct Memory {
    buf: Vec<u8>,
    reserved_word: Option<u32>, // For atomic lr/sc
}

impl Memory {
    pub fn new(size: usize) -> Memory {
        let buf = vec![0; size];
        Memory {
            buf,
            reserved_word: None,
        }
    }

    // Views
    pub fn as_u8(&self) -> &[u8] {
        &self.buf[..]
    }

    pub fn as_u8_mut(&mut self) -> &mut [u8] {
        &mut self.buf[..]
    }

    pub fn as_u16(&self) -> &[u16] {
        let len = self.buf.len();
        let ptr = self.buf.as_ptr() as *const u16;
        unsafe {
            return std::slice::from_raw_parts(ptr, len / 2);
        }
    }

    pub fn as_u32(&self) -> &[u32] {
        let len = self.buf.len();
        let ptr = self.buf.as_ptr() as *const u32;
        unsafe {
            return std::slice::from_raw_parts(ptr, len / 4);
        }
    }

    // Read
    pub fn read_u8(&self, addr: u32) -> Result8 {
        let addr = addr as usize;
        if addr >= self.buf.len() {
            Err(Trap::from_exception(LoadAccessFault, addr as u32))
        } else {
            Ok(self.buf[addr])
        }
    }

    pub fn read_u16(&self, addr: u32) -> Result16 {
        let addr = addr as usize;
        if addr >= self.buf.len() {
            Err(Trap::from_exception(LoadAccessFault, addr as u32))
        } else if addr & 0b1 != 0 {
            Err(Trap::from_exception(LoadAddrMisaligned, addr as u32))
        } else {
            Ok(u16::from_le_bytes([self.buf[addr], self.buf[addr + 1]]))
        }
    }

    pub fn read_u32(&self, addr: u32) -> Result32 {
        let addr = addr as usize;
        if addr >= self.buf.len() {
            Err(Trap::from_exception(LoadAccessFault, addr as u32))
        } else if addr & 0b11 != 0 {
            Err(Trap::from_exception(LoadAddrMisaligned, addr as u32))
        } else {
            Ok(u32::from_le_bytes([
                self.buf[addr],
                self.buf[addr + 1],
                self.buf[addr + 2],
                self.buf[addr + 3],
            ]))
        }
    }

    // Write (also clear reservation when needed)
    pub fn write_u8(&mut self, addr: u32, val: u8) -> Result {
        let addr = addr as usize;
        if addr >= self.buf.len() {
            Err(Trap::from_exception(StoreAccessFault, addr as u32))
        } else {
            self.buf[addr] = val;
            self.clear_reservation_if_matched(addr as u32);
            Ok(())
        }
    }

    pub fn write_u16(&mut self, addr: u32, val: u16) -> Result {
        let addr = addr as usize;
        if addr >= self.buf.len() {
            Err(Trap::from_exception(StoreAccessFault, addr as u32))
        } else if addr & 0b1 != 0 {
            Err(Trap::from_exception(StoreAddrMisaligned, addr as u32))
        } else {
            let bytes = val.to_le_bytes();
            self.buf[addr] = bytes[0];
            self.buf[addr + 1] = bytes[1];
            self.clear_reservation_if_matched(addr as u32);
            Ok(())
        }
    }

    pub fn write_u32(&mut self, addr: u32, val: u32) -> Result {
        let addr = addr as usize;
        if addr >= self.buf.len() {
            Err(Trap::from_exception(StoreAccessFault, addr as u32))
        } else if addr & 0b11 != 0 {
            Err(Trap::from_exception(StoreAddrMisaligned, addr as u32))
        } else {
            let bytes = val.to_le_bytes();
            self.buf[addr] = bytes[0];
            self.buf[addr + 1] = bytes[1];
            self.buf[addr + 2] = bytes[2];
            self.buf[addr + 3] = bytes[3];
            self.clear_reservation_if_matched(addr as u32);
            Ok(())
        }
    }

    pub fn check_write_u32(&mut self, addr: u32) -> Result {
        let addr = addr as usize;
        if addr >= self.buf.len() {
            Err(Trap::from_exception(StoreAccessFault, addr as u32))
        } else if addr & 0b11 != 0 {
            Err(Trap::from_exception(StoreAddrMisaligned, addr as u32))
        } else {
            Ok(())
        }
    }

    // Instruction fetch
    pub fn fetch(&self, addr: u32) -> Result32 {
        let addr = addr as usize;
        if addr >= self.buf.len() {
            Err(Trap::from_exception(InstrAccessFault, addr as u32))
        } else if addr & 0b11 != 0 {
            Err(Trap::from_exception(InstrAddrMisaligned, addr as u32))
        } else {
            Ok(u32::from_le_bytes([
                self.buf[addr],
                self.buf[addr + 1],
                self.buf[addr + 2],
                self.buf[addr + 3],
            ]))
        }
    }

    // Reservation
    pub fn reserve(&mut self, addr: u32) {
        self.reserved_word = Some(addr >> 2);
    }

    pub fn is_reserved(&self, addr: u32) -> bool {
        self.reserved_word.map_or(false, |word| word == addr >> 2)
    }

    pub fn clear_reservation(&mut self) {
        self.reserved_word = None;
    }

    pub fn clear_reservation_if_matched(&mut self, addr: u32) {
        if let Some(word) = self.reserved_word {
            if word == addr >> 2 {
                self.reserved_word = None;
            }
        }
    }
}

impl Debug for Memory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} bytes)", self.buf.len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Exception;
    use rand::{self, Rng};

    const MEM_SIZE: u32 = 0x400;
    const MASK: u32 = 0xffff_fffc;

    fn trap(ex: Exception, addr: u32) -> Trap {
        Trap::from_exception(ex, addr)
    }

    #[test]
    fn test_u8_write() {
        let mut mem = Memory::new(MEM_SIZE as usize); // 1 kB

        for _ in 0..10 {
            let addr = rand::thread_rng().gen_range(0..MEM_SIZE) & MASK;
            let data: u8 = rand::random();
            mem.write_u8(addr, data).unwrap();
            assert_eq!(mem.read_u8(addr).unwrap(), data);
        }
    }

    #[test]
    fn test_u16_write() {
        let mut mem = Memory::new(MEM_SIZE as usize); // 1 kB

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
        let mut mem = Memory::new(MEM_SIZE as usize); // 1 kB

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
        let mut mem = Memory::new(MEM_SIZE as usize); // 1 kB

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
        let mut mem = Memory::new(MEM_SIZE as usize); // 1 kB

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
        let mem = Memory::new(MEM_SIZE as usize); // 1 kB

        assert_eq!(mem.read_u8(MEM_SIZE).unwrap_err(), trap(LoadAccessFault, MEM_SIZE));
        assert_eq!(mem.read_u8(0xffff_ffff).unwrap_err(), trap(LoadAccessFault, 0xffff_ffff));

        assert_eq!(mem.read_u16(MEM_SIZE).unwrap_err(), trap(LoadAccessFault, MEM_SIZE));
        assert_eq!(mem.read_u16(0xffff_ffff).unwrap_err(), trap(LoadAccessFault, 0xffff_ffff));

        assert_eq!(mem.read_u32(MEM_SIZE).unwrap_err(), trap(LoadAccessFault, MEM_SIZE));
        assert_eq!(mem.read_u32(0xffff_ffff).unwrap_err(), trap(LoadAccessFault, 0xffff_ffff));
    }

    #[test]
    #[rustfmt::skip]
    fn test_read_misaligned() {
        let mem = Memory::new(MEM_SIZE as usize); // 1 kB

        assert_eq!(mem.read_u16(1).unwrap_err(), trap(LoadAddrMisaligned, 1));
        assert_eq!(mem.read_u16(3).unwrap_err(), trap(LoadAddrMisaligned, 3));

        assert_eq!(mem.read_u16(MEM_SIZE + 1).unwrap_err(), trap(LoadAccessFault, MEM_SIZE + 1));
        assert_eq!(mem.read_u16(MEM_SIZE + 3).unwrap_err(), trap(LoadAccessFault, MEM_SIZE + 3));

        assert_eq!(mem.read_u32(1).unwrap_err(), trap(LoadAddrMisaligned, 1));
        assert_eq!(mem.read_u32(2).unwrap_err(), trap(LoadAddrMisaligned, 2));
        assert_eq!(mem.read_u32(3).unwrap_err(), trap(LoadAddrMisaligned, 3));

        assert_eq!(mem.read_u32(MEM_SIZE + 1).unwrap_err(), trap(LoadAccessFault, MEM_SIZE + 1));
        assert_eq!(mem.read_u32(MEM_SIZE + 2).unwrap_err(), trap(LoadAccessFault, MEM_SIZE + 2));
        assert_eq!(mem.read_u32(MEM_SIZE + 3).unwrap_err(), trap(LoadAccessFault, MEM_SIZE + 3));
    }

    #[test]
    #[rustfmt::skip]
    fn test_write_fault() {
        let mut mem = Memory::new(MEM_SIZE as usize); // 1 kB

        assert_eq!(mem.write_u8(MEM_SIZE, 0).unwrap_err(), trap(StoreAccessFault, MEM_SIZE));
        assert_eq!(mem.write_u8(0xffff_ffff, 0).unwrap_err(), trap(StoreAccessFault, 0xffff_ffff));

        assert_eq!(mem.write_u16(MEM_SIZE, 0).unwrap_err(), trap(StoreAccessFault, MEM_SIZE));
        assert_eq!(mem.write_u16(0xffff_ffff, 0).unwrap_err(), trap(StoreAccessFault, 0xffff_ffff));

        assert_eq!(mem.write_u32(MEM_SIZE, 0).unwrap_err(), trap(StoreAccessFault, MEM_SIZE));
        assert_eq!(mem.write_u32(0xffff_ffff, 0).unwrap_err(), trap(StoreAccessFault, 0xffff_ffff));
    }

    #[test]
    #[rustfmt::skip]
    fn test_write_misaligned() {
        let mut mem = Memory::new(MEM_SIZE as usize); // 1 kB

        assert_eq!(mem.write_u16(1, 0).unwrap_err(), trap(StoreAddrMisaligned, 1));
        assert_eq!(mem.write_u16(3, 0).unwrap_err(), trap(StoreAddrMisaligned, 3));

        assert_eq!(mem.write_u16(MEM_SIZE + 1, 0).unwrap_err(), trap(StoreAccessFault, MEM_SIZE + 1));
        assert_eq!(mem.write_u16(MEM_SIZE + 3, 0).unwrap_err(), trap(StoreAccessFault, MEM_SIZE + 3));

        assert_eq!(mem.write_u32(1, 0).unwrap_err(), trap(StoreAddrMisaligned, 1));
        assert_eq!(mem.write_u32(2, 0).unwrap_err(), trap(StoreAddrMisaligned, 2));
        assert_eq!(mem.write_u32(3, 0).unwrap_err(), trap(StoreAddrMisaligned, 3));

        assert_eq!(mem.write_u32(MEM_SIZE + 1, 0).unwrap_err(), trap(StoreAccessFault, MEM_SIZE + 1));
        assert_eq!(mem.write_u32(MEM_SIZE + 2, 0).unwrap_err(), trap(StoreAccessFault, MEM_SIZE + 2));
        assert_eq!(mem.write_u32(MEM_SIZE + 3, 0).unwrap_err(), trap(StoreAccessFault, MEM_SIZE + 3));
    }

    #[test]
    #[rustfmt::skip]
    fn test_exec_fault() {
        let mem = Memory::new(MEM_SIZE as usize); // 1 kB

        assert_eq!(mem.fetch(MEM_SIZE).unwrap_err(), trap(InstrAccessFault, MEM_SIZE));
        assert_eq!(mem.fetch(0xffff_ffff).unwrap_err(), trap(InstrAccessFault, 0xffff_ffff));
    }

    #[test]
    #[rustfmt::skip]
    fn test_exec_misaligned() {
        let mem = Memory::new(MEM_SIZE as usize); // 1 kB

        assert_eq!(mem.fetch(1).unwrap_err(), trap(InstrAddrMisaligned, 1));
        assert_eq!(mem.fetch(2).unwrap_err(), trap(InstrAddrMisaligned, 2));
        assert_eq!(mem.fetch(3).unwrap_err(), trap(InstrAddrMisaligned, 3));

        assert_eq!(mem.fetch(MEM_SIZE + 1).unwrap_err(), trap(InstrAccessFault, MEM_SIZE + 1));
        assert_eq!(mem.fetch(MEM_SIZE + 2).unwrap_err(), trap(InstrAccessFault, MEM_SIZE + 2));
        assert_eq!(mem.fetch(MEM_SIZE + 3).unwrap_err(), trap(InstrAccessFault, MEM_SIZE + 3));
    }

    fn assert_word_reserved(mem: &Memory, addr: u32, expect: bool) {
        assert_eq!(mem.is_reserved(addr), expect);
        assert_eq!(mem.is_reserved(addr + 1), expect);
        assert_eq!(mem.is_reserved(addr + 2), expect);
        assert_eq!(mem.is_reserved(addr + 3), expect);
    }

    #[test]
    fn test_reserve() {
        let mut mem = Memory::new(MEM_SIZE as usize); // 1 kB

        for i in 0..=3 {
            mem.reserve(i);
            assert_word_reserved(&mem, 0, true);
            assert!(!mem.is_reserved(4));
            assert!(!mem.is_reserved(u32::MAX));
        }
    }

    #[test]
    fn test_double_reserve() {
        let mut mem = Memory::new(MEM_SIZE as usize); // 1 kB

        for i in 0..=3 {
            mem.reserve(i);
            mem.reserve(4 + i);
            assert_word_reserved(&mem, 0, false);
            assert_word_reserved(&mem, 4, true);
        }
    }

    #[test]
    fn test_clear_reservation() {
        let mut mem = Memory::new(MEM_SIZE as usize); // 1 kB

        for i in 0..=3 {
            mem.reserve(i);
            mem.clear_reservation();
            assert_word_reserved(&mem, 0, false);
        }
    }

    #[test]
    fn test_clear_reservation_if_matched() {
        let mut mem = Memory::new(MEM_SIZE as usize); // 1 kB

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
        let mut mem = Memory::new(MEM_SIZE as usize); // 1 kB

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
        let mut mem = Memory::new(MEM_SIZE as usize); // 1 kB

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
        let mut mem = Memory::new(MEM_SIZE as usize); // 1 kB

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
        let mut mem = Memory::new(MEM_SIZE as usize); // 1 kB

        for i in 0..=3 {
            mem.reserve(i);
            mem.write_u32(0, 0).unwrap();
            assert_word_reserved(&mem, 0, false);
        }
    }

    #[test]
    fn test_keep_reservation_if_write_elsewhere() {
        let mut mem = Memory::new(MEM_SIZE as usize); // 1 kB

        for i in 0..=3 {
            for j in 0..=3 {
                mem.reserve(i);
                mem.write_u8(4 + j, 0).unwrap();
                assert_word_reserved(&mem, 0, true);
            }
        }
    }
}
