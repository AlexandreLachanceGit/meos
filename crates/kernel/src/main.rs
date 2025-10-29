#![no_std]
#![no_main]

extern crate alloc;

mod log;
mod process;
mod time;

use alloc::format;
use core::arch::{asm, global_asm};
use core::panic::PanicInfo;

use allocator::{BumpAllocator, GlobalAllocator};

use crate::log::log;
use crate::process::ProcessManager;
use crate::time::Time;

#[global_allocator]
static GLOBAL_ALLOCATOR: GlobalAllocator<BumpAllocator> = GlobalAllocator::new();

global_asm!(include_str!("asm/riscv64/entry.s"));
unsafe extern "C" {
    static mut _HEAP_START: usize;
    static mut _HEAP_END: usize;
}

global_asm!(include_str!("asm/riscv64/switch.s"));
unsafe extern "C" {
    pub fn switch_context(prev_context_sp: *mut usize, next_context_sp: *const usize);
}

#[unsafe(no_mangle)]
pub extern "C" fn main() -> ! {
    log("Initializing global allocator...\n");
    unsafe {
        GLOBAL_ALLOCATOR.init(_HEAP_START, _HEAP_END);
    }
    log("Global allocator initialized.\n");

    log("Initializing process manager...");
    let process_manager = ProcessManager::default();
    log("Process manager initialized.\n");

    loop {
        delay();
        let available_ram = GLOBAL_ALLOCATOR.get_available() / 1024;

        log(&format!("RAM available: {available_ram} KB\n"));
        log(&format!("Cycle: {}\n", riscv::register::cycle::read64()));
        log(&format!("Time: {}\n", Time::get().as_millis()));
    }
}

fn delay() {
    for _ in 0..i32::MAX {
        unsafe { asm!("nop") };
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    log("KERNEL PANIC: ");
    if let Some(message) = info.message().as_str() {
        log(message);
    } else {
        log("Unknown reason");
    }
    log("\n");
    loop {}
}
