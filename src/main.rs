#![no_std]
#![no_main]

use core::arch::global_asm;
use core::panic::PanicInfo;
use core::ptr;

global_asm!(include_str!("entry.s"));

fn uart_print(message: &str) {
    const UART: *mut u8 = 0x10000000 as *mut u8;

    for c in message.chars() {
        unsafe {
            ptr::write_volatile(UART, c as u8);
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn main() -> ! {
    uart_print("Hello, world!\n");

    let mut ctr = 1;

    loop {
        for _ in 0..5000000 {}

        ctr += 1;
        uart_print("Loop\n");
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    uart_print("KERNEL PANIC");
    loop {}
}
