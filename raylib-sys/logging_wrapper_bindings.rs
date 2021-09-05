use std::os::raw::{ c_char, c_int };

extern {
    pub fn set_trace_log_callback(c: extern fn(c_int, *mut c_char));
} 
