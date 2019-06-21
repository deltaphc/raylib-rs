use crate::consts::TraceLogType;
use crate::ffi;
use std::ffi::CString;

/// Set the current threshold (minimum) log level
#[inline]
pub fn set_trace_log(types: TraceLogType) {
    unsafe {
        ffi::SetTraceLogLevel((types as u32) as i32);
    }
}

/// Set the exit threshold (minimum) log level
#[inline]
pub fn set_trace_log_exit(types: TraceLogType) {
    unsafe {
        ffi::SetTraceLogExit((types as u32) as i32);
    }
}

/// Writes a trace log message (`Log::INFO`, `Log::WARNING`, `Log::ERROR`, `Log::DEBUG`).
#[inline]
pub fn trace_log(msg_type: TraceLogType, text: &str) {
    unsafe {
        let text = CString::new(text).unwrap();
        ffi::TraceLog((msg_type as u32) as i32, text.as_ptr());
    }
}

#[cfg(test)]
mod test_logging {
    use super::*;
    #[test]
    fn test_logs() {
        use crate::consts::TraceLogType::*;
        set_trace_log(LOG_ALL);
        trace_log(LOG_DEBUG, "This Is From `test_logs`");
        set_trace_log(LOG_INFO);
    }
}
