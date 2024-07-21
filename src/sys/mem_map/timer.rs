use std::fmt::Debug;

#[derive(Debug)]
pub struct Timer {
    pub time: u64,
    pub timecmp: u64,
}

const MASK_LO: u64 = 0x00000000_ffffffffu64;
const MASK_HI: u64 = 0xffffffff_00000000u64;

impl Timer {
    pub fn new() -> Timer {
        Timer {
            time: 0,
            timecmp: u64::max_value(),
        }
    }

    pub fn is_interrupt_set(&self) -> bool {
        self.time >= self.timecmp
    }

    pub fn read_time(&self, addr: u64) -> u32 {
        let res = match addr {
            0 => self.time as u32,
            4 => (self.time >> 32) as u32,
            _ => panic!("invalid addr for Timer::read_time (addr = {addr})"),
        };
        // println!("Read time[{addr}] = 0x{res:08x}");
        res
    }

    pub fn write_time(&mut self, addr: u64, val: u32) {
        match addr {
            0 => {
                self.time &= !MASK_LO;
                self.time |= val as u64;
            }
            4 => {
                self.time &= !MASK_HI;
                self.time |= (val as u64) << 32;
            }
            _ => panic!("invalid addr for Timer::write_time (addr = {addr})"),
        };
        println!("Write time[{addr}] = 0x{val:08x}");
    }

    pub fn read_timecmp(&self, addr: u64) -> u32 {
        let res = match addr {
            0 => self.timecmp as u32,
            4 => (self.timecmp >> 32) as u32,
            _ => panic!("invalid addr for Timer::read_timecmp (addr = {addr})"),
        };
        println!("Read timecmp[{addr}] = 0x{res:08x}");
        res
    }

    pub fn write_timecmp(&mut self, addr: u64, val: u32) {
        match addr {
            0 => {
                self.timecmp &= !MASK_LO;
                self.timecmp |= val as u64;
            }
            4 => {
                self.timecmp &= !MASK_HI;
                self.timecmp |= (val as u64) << 32;
            }
            _ => panic!("invalid addr for Timer::write_timecmp (addr = {addr})"),
        };
        println!("Write timecmp[{addr}] = 0x{val:08x}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_time() {
        let mut timer = Timer::new();

        timer.time = 0x51290ce3_bcfec832_u64;

        assert_eq!(timer.read_time(0), 0xbcfec832_u32);
        assert_eq!(timer.read_time(4), 0x51290ce3_u32);
    }

    #[test]
    fn test_write_time() {
        let mut timer = Timer::new();

        timer.write_time(0, 0xbcfec832_u32);
        timer.write_time(4, 0x51290ce3_u32);

        assert_eq!(timer.time, 0x51290ce3_bcfec832_u64);
    }

    #[test]
    fn test_read_timecmp() {
        let mut timer = Timer::new();

        timer.timecmp = 0x51290ce3_bcfec832_u64;

        assert_eq!(timer.read_timecmp(0), 0xbcfec832_u32);
        assert_eq!(timer.read_timecmp(4), 0x51290ce3_u32);
    }

    #[test]
    fn test_write_timecmp() {
        let mut timer = Timer::new();

        timer.write_timecmp(0, 0xbcfec832_u32);
        timer.write_timecmp(4, 0x51290ce3_u32);

        assert_eq!(timer.timecmp, 0x51290ce3_bcfec832_u64);
    }
}
