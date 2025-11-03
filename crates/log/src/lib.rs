#![no_std]

use core::ptr;

pub fn log<S: AsRef<str>>(message: S) {
    const UART: *mut u8 = 0x10000000 as *mut u8;

    for c in message.as_ref().chars() {
        unsafe {
            ptr::write_volatile(UART, c as u8);
        }
    }
}

