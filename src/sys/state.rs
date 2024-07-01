use crate::instr::reg::Reg;

#[derive(Debug)]
pub struct State {
    pub pc: u32,
    pub regs: [i32; 32],
}

impl State {
    pub fn new() -> State {
        State {
            pc: 0,
            regs: [0; 32],
        }
    }

    pub fn reg(&self, r: &Reg) -> i32 {
        if r.index() == 0 {
            0
        } else {
            self.regs[r.index() as usize]
        }
    }

    pub fn reg_mut(&mut self, r: &Reg) -> &mut i32 {
        &mut self.regs[r.index() as usize]
    }

    pub fn pc(&self) -> u32 {
        self.pc
    }

    pub fn pc_mut(&mut self) -> &mut u32 {
        &mut self.pc
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand;

    #[test]
    fn test_reg_zero() {
        let mut state = State::new();
        assert_eq!(state.reg(&Reg::zero()), 0);

        *state.reg_mut(&Reg::zero()) = rand::random();
        assert_eq!(state.reg(&Reg::zero()), 0);
    }

    #[test]
    fn test_reg_write() {
        let mut state = State::new();
        let data: [i32; 32] = rand::random();
        for i in 1..32 {
            *state.reg_mut(&Reg::new(i)) = data[i as usize];
        }
        for i in 1..32 {
            assert_eq!(state.reg(&Reg::new(i)), data[i as usize]);
        }
    }

    #[test]
    fn test_pc_write() {
        let mut state = State::new();
        let data: u32 = rand::random();

        *state.pc_mut() = data;
        assert_eq!(state.pc(), data);
    }
}
