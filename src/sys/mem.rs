use std::fmt::Debug;

pub struct Memory {
    buf: Vec<u8>,
}

impl Memory {
    pub fn new(size: usize) -> Memory {
        let buf = vec![0; size];
        Memory { buf }
    }

    pub fn u8(&self) -> &[u8] {
        &self.buf[..]
    }

    pub fn u8_mut(&mut self) -> &mut [u8] {
        &mut self.buf[..]
    }

    pub fn u16(&self) -> &[u16] {
        let len = self.buf.len();
        let ptr = self.buf.as_ptr() as *const u16;
        unsafe {
            return std::slice::from_raw_parts(ptr, len / 2);
        }
    }

    pub fn u16_mut(&mut self) -> &mut [u16] {
        let len = self.buf.len();
        let ptr = self.buf.as_mut_ptr() as *mut u16;
        unsafe {
            return std::slice::from_raw_parts_mut(ptr, len / 2);
        }
    }

    pub fn u32(&self) -> &[u32] {
        let len = self.buf.len();
        let ptr = self.buf.as_ptr() as *const u32;
        unsafe {
            return std::slice::from_raw_parts(ptr, len / 4);
        }
    }

    pub fn u32_mut(&mut self) -> &mut [u32] {
        let len = self.buf.len();
        let ptr = self.buf.as_mut_ptr() as *mut u32;
        unsafe {
            return std::slice::from_raw_parts_mut(ptr, len / 4);
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

    const MEM_SIZE: usize = 0x400;

    #[test]
    fn test_u8_write() {
        let mut mem = Memory::new(MEM_SIZE);   // 1 kB

        for _ in 0..10 {
            let addr = rand::thread_rng().gen_range(0..MEM_SIZE);
            let data: u8 = rand::random();
            mem.u8_mut()[addr] = data;
            assert_eq!(mem.u8()[addr], data);
        }
    }

    #[test]
    fn test_u16_write() {
        let mut mem = Memory::new(MEM_SIZE);   // 1 kB

        for _ in 0..10 {
            let addr = rand::thread_rng().gen_range(0..MEM_SIZE) / 2;
            let data: u16 = rand::random();
            mem.u16_mut()[addr] = data;
            let bytes = data.to_ne_bytes();
            assert_eq!(mem.u8()[2 * addr], bytes[0]);
            assert_eq!(mem.u8()[2 * addr + 1], bytes[1]);
        }
    }

    #[test]
    fn test_u32_write() {
        let mut mem = Memory::new(MEM_SIZE);   // 1 kB

        for _ in 0..10 {
            let addr = rand::thread_rng().gen_range(0..MEM_SIZE) / 4;
            let data: u32 = rand::random();
            mem.u32_mut()[addr] = data;
            let bytes = data.to_ne_bytes();
            assert_eq!(mem.u8()[4 * addr], bytes[0]);
            assert_eq!(mem.u8()[4 * addr + 1], bytes[1]);
            assert_eq!(mem.u8()[4 * addr + 2], bytes[2]);
            assert_eq!(mem.u8()[4 * addr + 3], bytes[3]);
        }
    }

    #[test]
    fn test_u16_read() {
        let mut mem = Memory::new(MEM_SIZE);   // 1 kB

        for _ in 0..10 {
            let addr = rand::thread_rng().gen_range(0..MEM_SIZE) / 2;
            let data: u16 = rand::random();
            let bytes = data.to_ne_bytes();
            mem.u8_mut()[2 * addr] = bytes[0];
            mem.u8_mut()[2 * addr + 1] = bytes[1];
            assert_eq!(mem.u16()[addr], data);
        }
    }

    #[test]
    fn test_u32_read() {
        let mut mem = Memory::new(MEM_SIZE);   // 1 kB

        for _ in 0..10 {
            let addr = rand::thread_rng().gen_range(0..MEM_SIZE) / 4;
            let data: u32 = rand::random();
            let bytes = data.to_ne_bytes();
            mem.u8_mut()[4 * addr] = bytes[0];
            mem.u8_mut()[4 * addr + 1] = bytes[1];
            mem.u8_mut()[4 * addr + 2] = bytes[2];
            mem.u8_mut()[4 * addr + 3] = bytes[3];
            assert_eq!(mem.u32()[addr], data);
        }
    }
}
