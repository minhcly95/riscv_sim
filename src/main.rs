use riscv_sim::{self, decode};

fn main() {
    println!("{:?}", decode(0x80000fb7));
    println!("{:?}", decode(0xbcfed0b7));
    println!("{:?}", decode(0x89b08193));
    println!("{:?}", decode(0x89b0a193));
    println!("{:?}", decode(0));
}
