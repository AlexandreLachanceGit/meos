#![no_std]
#![no_main]

extern crate alloc;

mod interrupts;
mod process;
mod time;

use core::arch::{asm, global_asm};
use core::panic::PanicInfo;
use drivers::{DriverManager, UartDriver};
use dtb_reader::DtbReader;
use log::{add_logger, error, info};

use allocator::{BumpAllocator, GlobalAllocator};

use crate::process::ProcessManager;
use crate::time::Time;

#[global_allocator]
static GLOBAL_ALLOCATOR: GlobalAllocator<BumpAllocator> = GlobalAllocator::new();

global_asm!(include_str!("asm/riscv64/entry.s"));
unsafe extern "C" {
    static mut _KERNEL_END: usize;
}

global_asm!(include_str!("asm/riscv64/switch.s"));
unsafe extern "C" {
    pub fn switch_context(prev_context_sp: *mut usize, next_context_sp: *const usize);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn main(hw_thread_id: usize, dtb_ptr: *const u32) -> ! {
    // Single threaded for now
    if hw_thread_id != 0 {
        loop {
            core::hint::spin_loop();
        }
    }

    let dtb = unsafe { DtbReader::new(dtb_ptr).expect("failed to parse DTB") };
    let dtb_root = dtb.root_node();

    init_allocator(dtb_root);

    let mut driver_manager = DriverManager::default();
    driver_manager.load_drivers(&dtb_root);

    let chosen = dtb_root.get_child("chosen").expect("no chosen node in DTB");
    let stdout_path = chosen
        .get_property("stdout-path")
        .unwrap()
        .value_str()
        .unwrap();

    let stdout_uart = driver_manager
        .get_by_path::<dyn UartDriver>(stdout_path)
        .unwrap();
    add_logger(stdout_uart);

    info!("Stdout Path: {stdout_path}");

    info!("Initializing process manager...");
    let process_manager = ProcessManager::default();
    info!("Process manager initialized.");

    interrupts::setup();

    loop {
        delay();
        let available_ram = GLOBAL_ALLOCATOR.get_available() / 1024;

        info!("RAM available: {available_ram} KB");
        info!("Cycle: {}", riscv::register::cycle::read64());
        info!("Time: {}", Time::get().as_millis());
    }
}

fn init_allocator(dtb_root: dtb_reader::DeviceTreeNode) {
    for node in dtb_root.children() {
        if let Some(device_type) = node.get_property("device_type")
            && device_type.value_str().unwrap() == "memory"
        {
            let prop = node.get_property("reg").unwrap();
            let reg = prop.raw_value();
            let address = usize::from_be_bytes(reg[0..8].try_into().unwrap());
            let size = usize::from_be_bytes(reg[8..16].try_into().unwrap());

            let kernel_end = unsafe { _KERNEL_END };

            let end = address + size;

            let start = if kernel_end > address && kernel_end < end {
                kernel_end
            } else {
                address
            };

            GLOBAL_ALLOCATOR.init(start, end);
            break;
        }
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
        error!(
            "KERNEL PANIC @ line {}, col {} in {}: \nDetails:\n\t{}",
            location.line(),
            location.column(),
            location.file(),
            info.message()
        );
    } else {
        error!("KERNEL PANIC:\nDetails:\n\t{}\n", info.message());
    }

    loop {}
}
