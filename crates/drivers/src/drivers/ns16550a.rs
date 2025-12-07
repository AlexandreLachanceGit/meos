use core::{fmt::Write, ptr};

use alloc::sync::Arc;
use dtb_reader::DeviceTreeNode;
use spin::Mutex;

use crate::{DriverManager, driver::Driver, driver_capabilities::UartDriver};

#[derive(Debug)]
pub struct Ns16550a {
    address: usize,
}

impl Driver for Ns16550a {
    fn try_initialize(node: &DeviceTreeNode, path: &str, manager: &mut DriverManager) -> bool {
        let address: Option<usize> = node.get_property("reg").and_then(|prop| {
            let bytes = prop.raw_value();
            if bytes.len() >= 4 {
                let addr_bytes: [u8; 4] = bytes[4..8].try_into().ok()?;
                Some(u32::from_be_bytes(addr_bytes) as usize)
            } else {
                None
            }
        });

        if let Some(address) = address {
            let concrete_driver = Ns16550a { address };
            let shared_driver = Arc::new(Mutex::new(concrete_driver));

            let as_uart: Arc<Mutex<dyn UartDriver>> = shared_driver;
            manager.register_capability::<dyn UartDriver>(path, as_uart);

            true
        } else {
            false
        }
    }

    fn compatible() -> &'static [&'static str] {
        &["ns16550a"]
    }
}

impl UartDriver for Ns16550a {
    fn put_char(&mut self, c: char) {
        unsafe {
            ptr::write_volatile(self.address as *mut u8, c as u8);
        }
    }

    fn get_char(&mut self) -> Option<char> {
        todo!()
    }

    fn set_baud(&mut self, _baud: u32) {
        todo!()
    }
}

impl Write for Ns16550a {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for char in s.chars() {
            self.put_char(char);
        }
        Ok(())
    }
}
