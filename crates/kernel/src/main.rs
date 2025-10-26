#![no_std]
#![no_main]

extern crate alloc;

mod log;

use alloc::format;
use core::arch::{asm, global_asm};
use core::panic::PanicInfo;

use allocator::{BumpAllocator, GlobalAllocator};

use crate::log::log;

#[global_allocator]
static GLOBAL_ALLOCATOR: GlobalAllocator<BumpAllocator> = GlobalAllocator::new();

global_asm!(include_str!("entry.s"));

unsafe extern "C" {
    static mut _HEAP_START: usize;
    static mut _HEAP_END: usize;
}

#[unsafe(no_mangle)]
pub extern "C" fn main() -> ! {
    log("Initializing global allocator...\n");
    unsafe {
        GLOBAL_ALLOCATOR.init(_HEAP_START, _HEAP_END);
    }
    log("Global allocator initialized.\n");

    loop {
        delay();
        let available_ram = GLOBAL_ALLOCATOR.get_available() / 1024;

        log(&format!("RAM available: {available_ram} KB\n",));
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
