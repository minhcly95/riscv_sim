use std::fmt::Debug;

pub struct Ram {
    buf: Vec<u8>,
}

impl Ram {
    pub fn new(size: usize) -> Ram {
        let buf = vec![0; size];
        Ram {
            buf,
        }
    }

    pub fn size(&self) -> u64 {
        self.buf.len() as u64
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
}

impl Debug for Ram {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} bytes)", self.buf.len())
    }
}
