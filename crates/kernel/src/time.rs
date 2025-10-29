use core::time::Duration;

const TICKS_PER_SECOND: usize = 10_000_000; // TODO: Read this from DTB
const NANOS_PER_SECOND: u64 = 1_000_000_000;

const NANOS_PER_TICK: u64 = NANOS_PER_SECOND / (TICKS_PER_SECOND as u64);

pub struct Time;

impl Time {
    pub fn get() -> Duration {
        let ticks = riscv::register::time::read64();
        Duration::from_nanos(ticks * NANOS_PER_TICK)
    }
}
