#[cfg(test)]
mod callback_tests {
    use std::ffi::c_void;

    use crate::tests::*;
    use colored::Colorize;
    use raylib::{ffi::__va_list_tag, prelude::*};

    fn custom_callback(log_level: TraceLogLevel, st: &str) {
        let (prefix, string) = match log_level {
            TraceLogLevel::LOG_ALL => ("".white().bold(), st.white()),
            TraceLogLevel::LOG_TRACE => ("Trace: ".white().bold(), st.white()),
            TraceLogLevel::LOG_DEBUG => ("Debug: ".white().bold(), st.white()),
            TraceLogLevel::LOG_INFO => ("Info: ".white().bold(), st.white()),
            TraceLogLevel::LOG_WARNING => ("Warning: ".yellow().bold(), st.yellow()),
            TraceLogLevel::LOG_ERROR => ("Error: ".red().bold(), st.red()),
            TraceLogLevel::LOG_FATAL => ("Fatal: ".red().bold(), st.red().bold()),
            TraceLogLevel::LOG_NONE => ("".white().bold(), st.white()),
        };
        println!("{}{}", prefix, string);
    }
    ray_test!(callback_test);

    fn callback_test(thread: &RaylibThread) {
        println!(
            "\n{}\n",
            "Setting custom logger. The rest of the test should be using this custom logger."
                .bold()
                .underline(),
        );
        let mut handle = TEST_HANDLE.write().unwrap();
        let rl = handle.as_mut().unwrap();
        {
            rl.set_trace_log_callback(custom_callback).unwrap();
            for _ in 0..5 {
                let noise = Image::gen_image_white_noise(10, 10, 1.0);
                let _ = rl.load_texture_from_image(&thread, &noise).unwrap();
            }
        }
    }
}
