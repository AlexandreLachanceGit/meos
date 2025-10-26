#![no_std]
#![no_main]

extern crate alloc;

mod allocator;
mod log;

use alloc::format;
use core::arch::global_asm;
use core::panic::PanicInfo;

use crate::allocator::GlobalAllocator;
use crate::log::log;

#[global_allocator]
static GLOBAL_ALLOCATOR: GlobalAllocator = GlobalAllocator::new();

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
    log(&format!(
        "RAM available: {} KB\n",
        GLOBAL_ALLOCATOR.get_available() / 1024
    ));

    loop {
        for _ in 0..500000000 {}

        log(&format!(
            "RAM available: {} KB\n",
            GLOBAL_ALLOCATOR.get_available() / 1024
        ));
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
