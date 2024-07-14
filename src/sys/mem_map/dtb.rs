use std::fmt::Debug;

pub struct Dtb {
    buf: Vec<u8>,
}

impl Dtb {
    pub fn new(buf: Vec<u8>) -> Dtb {
        Dtb { buf }
    }

    pub fn size(&self) -> u64 {
        self.buf.len() as u64
    }

    pub fn as_u8(&self) -> &[u8] {
        &self.buf[..]
    }
}

impl Debug for Dtb {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Dtb ({} bytes)", self.buf.len())
    }
}
