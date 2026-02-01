use core::{
    sync::atomic::{AtomicU64, Ordering},
    time::Duration,
};

const NANOS_PER_SECOND: u64 = 1_000_000_000;
static NANOS_PER_TICK: AtomicU64 = AtomicU64::new(0);

pub unsafe fn setup(timebase_frequency: u64) {
    NANOS_PER_TICK.store(NANOS_PER_SECOND / timebase_frequency, Ordering::Relaxed);
}

pub struct Time;

impl Time {
    pub fn get() -> Duration {
        let ticks = riscv::register::time::read64();
        Duration::from_nanos(ticks * NANOS_PER_TICK.load(Ordering::Relaxed))
    }
}
