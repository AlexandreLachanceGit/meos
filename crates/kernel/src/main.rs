#![no_std]
#![no_main]

extern crate alloc;

mod interupts;
mod log;
mod process;
mod time;

use alloc::format;
use core::arch::{asm, global_asm};
use core::panic::PanicInfo;
use dtb_reader::DtbReader;

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
pub extern "C" fn main(hw_thread_id: usize, dtb_ptr: *const u32) -> ! {
    // Single threaded for now
    if hw_thread_id != 0 {
        loop {
            delay();
        }
    }

    log("Initializing global allocator...\n");
    unsafe {
        GLOBAL_ALLOCATOR.init(_HEAP_START, _HEAP_END);
    }
    log("Global allocator initialized.\n");

    let dtb = DtbReader::new(dtb_ptr).expect("failed to parse DTB");

    log(format!("DTB Header: {:?}\n", dtb.fdt_header));

    log("Initializing process manager...\n");
    let process_manager = ProcessManager::default();
    log("Process manager initialized.\n");

    interupts::setup();

    loop {
        delay();
        let available_ram = GLOBAL_ALLOCATOR.get_available() / 1024;

        log(format!("RAM available: {available_ram} KB\n"));
        log(format!("Cycle: {}\n", riscv::register::cycle::read64()));
        log(format!("Time: {}\n", Time::get().as_millis()));
    }
}

fn delay() {
    for _ in 0..i32::MAX {
        unsafe { asm!("nop") };
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    if let Some(location) = info.location() {
        log(format!(
            "KERNEL PANIC @ line {}, col {} in {}: \nDetails:\n\t{}\n",
            location.line(),
            location.column(),
            location.file(),
            info.message()
        ));
    } else {
        log(format!("KERNEL PANIC:\nDetails:\n\t{}\n", info.message()));
    }

    loop {}
}
