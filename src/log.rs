use core::ptr;

pub fn log(message: &str) {
    const UART: *mut u8 = 0x10000000 as *mut u8;

    for c in message.chars() {
        unsafe {
            ptr::write_volatile(UART, c as u8);
        }
    }
}
