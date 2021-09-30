//! Contains utilities for replacing the raylib logger with the `log` create.

use std::{convert::TryInto, ffi::c_void};

use num_traits::FromPrimitive;
use printf::printf;

/// Direct mapping of raylib's log levels
/// See: https://github.com/raysan5/raylib/blob/d875891a3c2621ab40733ca3569eca9e054a6506/parser/raylib_api.json#L985-L1026
#[derive(FromPrimitive)]
enum RaylibLogLevel {
    All = 0,
    Trace = 1,
    Debug = 2,
    Info = 3,
    Warning = 4,
    Error = 5,
    Fatal = 6,
    None = 7,
}

/// Logging callback that is passed through to raylib over the ffi boundary.
#[no_mangle]
pub unsafe extern "C" fn log_callback(
    level: i32,
    message: *const i8,
    args: *mut crate::ffi::__va_list_tag,
) {
    // Get the message as a string. This is calling back to C code with a reasonably safe sprintf implementation.
    let formatted_message = printf(message, args as *mut c_void);

    // Handle the log level and fall back on info!
    match RaylibLogLevel::from_u32(level.try_into().unwrap()) {
        Some(level) => match level {
            RaylibLogLevel::Trace => log::trace!(target:"raylib", "{}", formatted_message),
            RaylibLogLevel::Debug => log::debug!(target:"raylib", "{}", formatted_message),
            RaylibLogLevel::Warning => log::warn!(target:"raylib", "{}", formatted_message),
            RaylibLogLevel::Error => log::error!(target:"raylib", "{}", formatted_message),
            RaylibLogLevel::Fatal => log::error!(target:"raylib", "{}", formatted_message),
            _ => log::info!(target:"raylib", "{}", formatted_message),
        },
        None => {
            log::info!(target:"raylib", "{}", formatted_message)
        }
    }
}