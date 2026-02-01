use core::ops::{Index, IndexMut};

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Register {
    Ra = 0,
    Sp,
    Gp,
    Tp,
    T0,
    T1,
    T2,
    S0,
    S1,
    A0,
    A1,
    A2,
    A3,
    A4,
    A5,
    A6,
    A7,
    S2,
    S3,
    S4,
    S5,
    S6,
    S7,
    S8,
    S9,
    S10,
    S11,
    T3,
    T4,
    T5,
    T6,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct TrapFrame {
    regs: [usize; 31],
}

impl Index<Register> for TrapFrame {
    type Output = usize;
    fn index(&self, reg: Register) -> &Self::Output {
        &self.regs[reg as usize]
    }
}

impl IndexMut<Register> for TrapFrame {
    fn index_mut(&mut self, reg: Register) -> &mut Self::Output {
        &mut self.regs[reg as usize]
    }
}
