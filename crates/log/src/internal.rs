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
