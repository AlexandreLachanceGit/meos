#[macro_export]
macro_rules! csr_read {
    ($csr_num:literal) => {{
        let value;
        unsafe {
            core::arch::asm!(
                concat!("csrr {0}, ", $csr_num),
                out(reg) value,
                options(nomem, nostack, preserves_flags)
            );
        }
        value
    }};
}

pub enum Csr {
    Cycle,
    Time,
}

impl Csr {
    pub fn read(&self) -> usize {
        match self {
            Csr::Cycle => csr_read!("cycle"),
            Csr::Time => csr_read!("time"),
        }
    }
}
