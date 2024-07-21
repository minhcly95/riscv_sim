use console::Term;
use core::panic;
use std::{fmt::Debug, io::{stdout, Write}};

// This is an emulator for the 8250 serial chip with:
// - Infinite-length FIFOs (never full)
// - Ignore baud-rate and tranmission modes
// - Error-free
// - No modem control signals (DTR RTS CTS DSR RI CD)
#[derive(Debug)]
pub struct Uart {
    term: Term,
    int_en: u8,
    int_pending: UartInt,
    line_control: u8,
    modem_control: u8,
    scratch: u8,
    div_latch_lo: u8,
    div_latch_hi: u8,
}

#[derive(Debug)]
enum UartInt {
    NoInt,
}

const IER_MASK: u8 = 0b0000_1111;
const MCR_MASK: u8 = 0b0001_1111;

const LCR_DLAB: u8 = 0b1000_0000; // Divisor latch access bit
const LSR_VAL: u8 = 0b0110_0001; // No error, TX always empty, RX always ready

impl Uart {
    pub fn new() -> Uart {
        Uart {
            term: Term::stdout(),
            int_en: 0,
            int_pending: UartInt::NoInt,
            line_control: 0,
            modem_control: 0,
            scratch: 0,
            div_latch_lo: 1,
            div_latch_hi: 0,
        }
    }

    fn is_dlab_set(&self) -> bool {
        self.line_control & LCR_DLAB != 0
    }

    pub fn write(&mut self, addr: u64, val: u8) {
        match addr {
            0 => {
                if !self.is_dlab_set() {
                    // THR: Tranmission holding register
                    self.term.write(&[val]).expect("cannot write from Uart");
                } else {
                    self.div_latch_lo = val; // Divisor latch
                }
            }
            1 => {
                if !self.is_dlab_set() {
                    // IER: Interrupt enable register
                    self.int_en = val & IER_MASK;
                } else {
                    self.div_latch_hi = val; //Divisor latch
                }
            }
            3 => self.line_control = val, // LCR: Line control register
            4 => self.modem_control = val & MCR_MASK, // MCR: Modem control register
            7 => self.scratch = val,      // SPR: Scratch pad register
            2 | 5 | 6 => (),
            _ => panic!("invalid addr for Uart (addr = {addr})"),
        };
        println!("Write Uart[{addr}] = 0x{val:02x}");
    }

    pub fn read(&mut self, addr: u64) -> u8 {
        print!("Read Uart[{addr}]... ");
        stdout().flush().unwrap();
        let res = match addr {
            0 => {
                if !self.is_dlab_set() {
                    // THR: Tranmission holding register
                    self.term.read_char().expect("cannot write from Uart") as u8
                } else {
                    self.div_latch_lo // Divisor latch
                }
            }
            1 => {
                if !self.is_dlab_set() {
                    self.int_en // IER: Interrupt enable register
                } else {
                    self.div_latch_hi // Divisor latch
                }
            }
            2 => {
                // ISR: Interrupt status register
                match self.int_pending {
                    UartInt::NoInt => 0b0001,
                }
            }
            3 => self.line_control,  // LCR: Line control register
            4 => self.modem_control, // MCR: Modem control register
            5 => LSR_VAL,            // LSR: Line status register
            6 => 0,                  // MSR: Modem status register
            7 => self.scratch,       // SPR: Scratch pad register
            _ => panic!("invalid addr for Uart (addr = {addr})"),
        };
        println!("Read Uart[{addr}] = 0x{res:02x}");
        res
    }
}
