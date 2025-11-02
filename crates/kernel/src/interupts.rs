pub fn setup() {
    unsafe {
        // riscv::register::mstatus::set_mie(); // Enable Machine Interrupt
        // riscv::register::mstatus::set_sie(); // Enable Supervisor Interrupt
        //
        // riscv::register::mie::set_mtimer(); // Enable Machine Timer Interrupt
        // riscv::register::mie::set_stimer(); // Enable Supervisor Timer Interrupt
    }
}
