#![no_std]

extern crate alloc;

use alloc::{sync::Arc, vec::Vec};
use core::fmt::Write;
use spin::{Mutex, Once};

pub use crate::level::LogLevel;
pub mod internal;
pub mod level;

type Logger = Arc<Mutex<dyn Write + Send + Sync>>;
static LOGGERS: Once<Mutex<Vec<Logger>>> = Once::new();

/// Add logger to the global list
pub fn add_logger(logger: Arc<Mutex<dyn Write + Send + Sync>>) {
    LOGGERS
        .call_once(|| Mutex::new(Vec::new()))
        .lock()
        .push(logger);
}

const fn parse_log_level() -> u8 {
    match option_env!("LOG_LEVEL") {
        Some(level_str) => {
            let bytes = level_str.as_bytes();
            if bytes.is_empty() {
                return 2;
            }

            let first_char = bytes[0] as char;
            if first_char.is_ascii_digit() {
                (first_char as u8).saturating_sub(b'0')
            } else {
                2
            }
        }
        None => 2,
    }
}

pub const LOG_LEVEL: u8 = parse_log_level();

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {{
        const LEVEL: u8 = 0; // Error = 0
        const SHOULD_LOG: bool = LEVEL <= $crate::LOG_LEVEL;
        if SHOULD_LOG {
            $crate::internal::log($crate::LogLevel::Error, ::alloc::format!($($arg)*));
        }
    }};
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {{
        const LEVEL: u8 = 1; // Warn = 1
        const SHOULD_LOG: bool = LEVEL <= $crate::LOG_LEVEL;
        if SHOULD_LOG {
            $crate::internal::log($crate::LogLevel::Warn, ::alloc::format!($($arg)*));
        }
    }};
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {{
        const LEVEL: u8 = 2; // Info = 2
        const SHOULD_LOG: bool = LEVEL <= $crate::LOG_LEVEL;
        if SHOULD_LOG {
            $crate::internal::log($crate::LogLevel::Info, ::alloc::format!($($arg)*));
        }
    }};
}

#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {{
        const LEVEL: u8 = 3; // Debug = 3
        const SHOULD_LOG: bool = LEVEL <= $crate::LOG_LEVEL;
        if SHOULD_LOG {
            $crate::internal::log($crate::LogLevel::Debug, ::alloc::format!($($arg)*));
        }
    }};
}

