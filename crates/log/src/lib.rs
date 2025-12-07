#![no_std]

extern crate alloc;

use core::fmt::Write;

use alloc::{sync::Arc, vec::Vec};
use spin::{Mutex, Once};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogLevel {
    Error,
    Warn,
    Info,
    Debug,
}

impl LogLevel {
    pub fn as_str(&self) -> &'static str {
        match self {
            LogLevel::Error => "ERROR",
            LogLevel::Warn => "WARN",
            LogLevel::Info => "INFO",
            LogLevel::Debug => "DEBUG",
        }
    }

    pub const fn as_u8(&self) -> u8 {
        match self {
            LogLevel::Error => 0,
            LogLevel::Warn => 1,
            LogLevel::Info => 2,
            LogLevel::Debug => 3,
        }
    }
}

type Logger = Arc<Mutex<dyn Write + Send + Sync>>;
static LOGGERS: Once<Mutex<Vec<Logger>>> = Once::new();

/// Add logger to the global list
pub fn add_logger(logger: Arc<Mutex<dyn Write + Send + Sync>>) {
    LOGGERS
        .call_once(|| Mutex::new(Vec::new()))
        .lock()
        .push(logger);
}

pub mod internal {
    use alloc::format;

    use crate::{LOGGERS, LogLevel};

    /// # Warning: Do not call directly!
    ///
    /// Use the macros instead: `error!()`, `warn!()`, `info!()`, `debug!()`
    pub fn log<S: AsRef<str>>(level: LogLevel, message: S) {
        if let Some(loggers) = LOGGERS.get() {
            let message = format!("[{}] {}\n", level.as_str(), message.as_ref());
            for logger in &mut *loggers.lock() {
                // TODO: Don't panic on write failure
                logger.lock().write_str(message.as_ref()).unwrap();
            }
        }
    }
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
                (first_char as u8) - b'0'
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

