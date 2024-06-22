use riscv_sim::{self, decode, execute, System};

fn main() {
    let instr = decode(0x80000fb7).unwrap();
    println!("{:#?}", instr);
    let mut sys = System::new(0x100000); // 1 MB of memory
    execute(&mut sys, &instr).unwrap();
    println!("{:#?}", sys);
}
