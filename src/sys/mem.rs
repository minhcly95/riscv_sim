use std::fmt::Debug;

use crate::Exception;

pub struct Memory {
    buf: Vec<u8>,
}

impl Memory {
    pub fn new(size: usize) -> Memory {
        let buf = vec![0; size];
        Memory { buf }
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
    pub fn read_u8(&self, addr: u32) -> Result<u8, Exception> {
        let addr = addr as usize;
        if addr >= self.buf.len() {
            Err(Exception::LoadAccessFault)
        } else {
            Ok(self.buf[addr])
        }
    }

    pub fn read_u16(&self, addr: u32) -> Result<u16, Exception> {
        let addr = addr as usize;
        if addr + 1 >= self.buf.len() {
            Err(Exception::LoadAccessFault)
        } else if addr & 0b1 != 0 {
            Err(Exception::LoadAddrMisaligned)
        } else {
            Ok(u16::from_le_bytes([self.buf[addr], self.buf[addr + 1]]))
        }
    }

    pub fn read_u32(&self, addr: u32) -> Result<u32, Exception> {
        let addr = addr as usize;
        if addr + 3 >= self.buf.len() {
            Err(Exception::LoadAccessFault)
        } else if addr & 0b11 != 0 {
            Err(Exception::LoadAddrMisaligned)
        } else {
            Ok(u32::from_le_bytes([
                self.buf[addr],
                self.buf[addr + 1],
                self.buf[addr + 2],
                self.buf[addr + 3],
            ]))
        }
    }

    // Write
    pub fn write_u8(&mut self, addr: u32, val: u8) -> Result<(), Exception> {
        let addr = addr as usize;
        if addr >= self.buf.len() {
            Err(Exception::StoreAccessFault)
        } else {
            self.buf[addr] = val;
            Ok(())
        }
    }

    pub fn write_u16(&mut self, addr: u32, val: u16) -> Result<(), Exception> {
        let addr = addr as usize;
        if addr + 1 >= self.buf.len() {
            Err(Exception::StoreAccessFault)
        } else if addr & 0b1 != 0 {
            Err(Exception::StoreAddrMisaligned)
        } else {
            let bytes = val.to_le_bytes();
            self.buf[addr] = bytes[0];
            self.buf[addr + 1] = bytes[1];
            Ok(())
        }
    }

    pub fn write_u32(&mut self, addr: u32, val: u32) -> Result<(), Exception> {
        let addr = addr as usize;
        if addr + 3 >= self.buf.len() {
            Err(Exception::StoreAccessFault)
        } else if addr & 0b11 != 0 {
            Err(Exception::StoreAddrMisaligned)
        } else {
            let bytes = val.to_le_bytes();
            self.buf[addr] = bytes[0];
            self.buf[addr + 1] = bytes[1];
            self.buf[addr + 2] = bytes[2];
            self.buf[addr + 3] = bytes[3];
            Ok(())
        }
    }

    // Instruction fetch
    pub fn fetch(&self, addr: u32) -> Result<u32, Exception> {
        let addr = addr as usize;
        if addr + 3 >= self.buf.len() {
            Err(Exception::InstrAccessFault)
        } else if addr & 0b11 != 0 {
            Err(Exception::InstrAddrMisaligned)
        } else {
            Ok(u32::from_le_bytes([
                self.buf[addr],
                self.buf[addr + 1],
                self.buf[addr + 2],
                self.buf[addr + 3],
            ]))
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
    use rand::{self, Rng};
    use Exception::*;

    const MEM_SIZE: u32 = 0x400;
    const MASK: u32 = 0xffff_fffc;

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
    fn test_read_fault() {
        let mem = Memory::new(MEM_SIZE as usize); // 1 kB

        assert_eq!(mem.read_u8(MEM_SIZE), Err(LoadAccessFault));
        assert_eq!(mem.read_u8(0xffff_ffff), Err(LoadAccessFault));

        assert_eq!(mem.read_u16(MEM_SIZE), Err(LoadAccessFault));
        assert_eq!(mem.read_u16(0xffff_ffff), Err(LoadAccessFault));

        assert_eq!(mem.read_u32(MEM_SIZE), Err(LoadAccessFault));
        assert_eq!(mem.read_u32(0xffff_ffff), Err(LoadAccessFault));
    }

    #[test]
    fn test_read_misaligned() {
        let mem = Memory::new(MEM_SIZE as usize); // 1 kB

        assert_eq!(mem.read_u16(1), Err(LoadAddrMisaligned));
        assert_eq!(mem.read_u16(3), Err(LoadAddrMisaligned));

        assert_eq!(mem.read_u16(MEM_SIZE + 1), Err(LoadAccessFault));
        assert_eq!(mem.read_u16(MEM_SIZE + 3), Err(LoadAccessFault));

        assert_eq!(mem.read_u32(1), Err(LoadAddrMisaligned));
        assert_eq!(mem.read_u32(2), Err(LoadAddrMisaligned));
        assert_eq!(mem.read_u32(3), Err(LoadAddrMisaligned));

        assert_eq!(mem.read_u32(MEM_SIZE + 1), Err(LoadAccessFault));
        assert_eq!(mem.read_u32(MEM_SIZE + 2), Err(LoadAccessFault));
        assert_eq!(mem.read_u32(MEM_SIZE + 3), Err(LoadAccessFault));
    }

    #[test]
    fn test_write_fault() {
        let mut mem = Memory::new(MEM_SIZE as usize); // 1 kB

        assert_eq!(mem.write_u8(MEM_SIZE, 0), Err(StoreAccessFault));
        assert_eq!(mem.write_u8(0xffff_ffff, 0), Err(StoreAccessFault));

        assert_eq!(mem.write_u16(MEM_SIZE, 0), Err(StoreAccessFault));
        assert_eq!(mem.write_u16(0xffff_ffff, 0), Err(StoreAccessFault));

        assert_eq!(mem.write_u32(MEM_SIZE, 0), Err(StoreAccessFault));
        assert_eq!(mem.write_u32(0xffff_ffff, 0), Err(StoreAccessFault));
    }

    #[test]
    fn test_write_misaligned() {
        let mut mem = Memory::new(MEM_SIZE as usize); // 1 kB

        assert_eq!(mem.write_u16(1, 0), Err(StoreAddrMisaligned));
        assert_eq!(mem.write_u16(3, 0), Err(StoreAddrMisaligned));

        assert_eq!(mem.write_u16(MEM_SIZE + 1, 0), Err(StoreAccessFault));
        assert_eq!(mem.write_u16(MEM_SIZE + 3, 0), Err(StoreAccessFault));

        assert_eq!(mem.write_u32(1, 0), Err(StoreAddrMisaligned));
        assert_eq!(mem.write_u32(2, 0), Err(StoreAddrMisaligned));
        assert_eq!(mem.write_u32(3, 0), Err(StoreAddrMisaligned));

        assert_eq!(mem.write_u32(MEM_SIZE + 1, 0), Err(StoreAccessFault));
        assert_eq!(mem.write_u32(MEM_SIZE + 2, 0), Err(StoreAccessFault));
        assert_eq!(mem.write_u32(MEM_SIZE + 3, 0), Err(StoreAccessFault));
    }

    #[test]
    fn test_exec_fault() {
        let mem = Memory::new(MEM_SIZE as usize); // 1 kB

        assert_eq!(mem.fetch(MEM_SIZE), Err(InstrAccessFault));
        assert_eq!(mem.fetch(0xffff_ffff), Err(InstrAccessFault));
    }

    #[test]
    fn test_exec_misaligned() {
        let mem = Memory::new(MEM_SIZE as usize); // 1 kB

        assert_eq!(mem.fetch(1), Err(InstrAddrMisaligned));
        assert_eq!(mem.fetch(2), Err(InstrAddrMisaligned));
        assert_eq!(mem.fetch(3), Err(InstrAddrMisaligned));

        assert_eq!(mem.fetch(MEM_SIZE + 1), Err(InstrAccessFault));
        assert_eq!(mem.fetch(MEM_SIZE + 2), Err(InstrAccessFault));
        assert_eq!(mem.fetch(MEM_SIZE + 3), Err(InstrAccessFault));
    }
}
