use core::sync::atomic::{AtomicU64, Ordering};

use log::debug;
use riscv::{
    ExceptionNumber, InterruptNumber,
    interrupt::{Exception, Interrupt},
    register::{scause, sepc, stval, stvec::Stvec},
};

use crate::traps::trap_frame::{Register, TrapFrame};

const TICK_RATE: u64 = 100;
static TICK_INTERVAL: AtomicU64 = AtomicU64::new(0);

pub unsafe fn setup(timebase_frequency: u64) {
    TICK_INTERVAL.store(timebase_frequency / TICK_RATE, Ordering::Relaxed);

    unsafe {
        unsafe extern "C" {
            fn trap_entry();
        }

        riscv::register::stvec::write(Stvec::new(
            trap_entry as usize,
            riscv::register::stvec::TrapMode::Direct,
        ));

        riscv::register::sstatus::set_sie(); // Enable Supervisor Interrupt
        riscv::register::sie::set_stimer(); // Enable Supervisor Timer Interrupt
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn trap_handler(tf: &mut TrapFrame) {
    let cause = scause::read();
    let stval = stval::read();
    let sepc = sepc::read();

    if cause.is_interrupt() {
        handle_interrupt(
            Interrupt::from_number(cause.code()).expect("received invalid interrupt code"),
            tf,
        );
    } else {
        handle_exception(
            Exception::from_number(cause.code()).expect("received invalid exception code"),
            stval,
            sepc,
            tf,
        );
    }
}

fn handle_interrupt(interrupt: Interrupt, _tf: &mut TrapFrame) {
    match interrupt {
        Interrupt::SupervisorTimer => {
            // Use SBI to clear the next interrupt time
            let interval = TICK_INTERVAL.load(Ordering::Relaxed);
            let next_interval = riscv::register::time::read64() + interval;
            sbi_rt::set_timer(next_interval);
        }
        _ => panic!("Unhandled interrupt: {:?}", interrupt),
    }
}

fn handle_exception(exception: Exception, val: usize, epc: usize, tf: &mut TrapFrame) {
    match exception {
        Exception::UserEnvCall => {
            debug!("Syscall: number={}", tf[Register::A7]);

            // For syscalls, we need to increment sepc otherwise infinite loop
            unsafe {
                sepc::write(epc + 4);
            }
        }
        _ => {
            panic!(
                "Unhandled Exception: code={:?}, val={:#x}, epc={:#x}",
                exception, val, epc
            );
        }
    }
}
