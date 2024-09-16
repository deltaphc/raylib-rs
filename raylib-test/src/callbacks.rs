#[cfg(test)]
pub mod callback_tests {
    use std::{
        fs::OpenOptions,
        io::{Read, Write},
    };

    use crate::tests::*;
    use colored::Colorize;
    use raylib::prelude::*;

    #[cfg(not(target_os = "windows"))]
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
    fn custom_save_file_data_callback(name: &str, data: &[u8]) -> bool {
        println!("saving data file {}", name);
        let mut f = OpenOptions::new()
            .create(true)
            .write(true)
            .open(name)
            .unwrap();
        match f.write(data) {
            Ok(_) => true,
            Err(err) => {
                println!("{}: {}", "Error".red().bold(), err.to_string().red());
                false
            }
        }
    }

    fn custom_save_file_text_callback(name: &str, data: &str) -> bool {
        println!("saving text file {}", name);
        let mut f = OpenOptions::new()
            .create(true)
            .write(true)
            .open(name)
            .unwrap();
        match f.write(data.as_bytes()) {
            Ok(_) => true,
            Err(err) => {
                println!("{}: {}", "Error".red().bold(), err.to_string().red());
                false
            }
        }
    }

    fn custom_read_file_data_callback(name: &str) -> Vec<u8> {
        println!("reading data file {}", name);
        match OpenOptions::new().read(true).open(name) {
            Ok(mut f) => {
                let mut bytes = vec![];
                f.read_to_end(&mut bytes).unwrap();
                bytes
            }
            Err(err) => {
                println!("{}: {}", "Error".red().bold(), err.to_string().red());
                return vec![];
            }
        }
    }

    fn custom_read_file_text_callback(name: &str) -> String {
        println!("reading text file {}", name);
        match OpenOptions::new().read(true).open(name) {
            Ok(mut f) => {
                let mut string = String::new();
                match f.read_to_string(&mut string) {
                    Ok(a) => return a.to_string(),
                    Err(err) => {
                        println!("{}: {}", "Error".red().bold(), err.to_string().red());
                        return String::new();
                    }
                }
            }
            Err(err) => {
                println!("{}: {}", "Error".red().bold(), err.to_string().red());
                return String::new();
            }
        }
    }
    pub fn set_logger(thread: &RaylibThread) {
        #[cfg(not(target_os = "windows"))]
        {
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

    pub fn set_file_data_saver(_: &RaylibThread) {
        println!(
            "\n{}\n",
            "Setting file data saver callback".bold().underline(),
        );
        let mut handle = TEST_HANDLE.write().unwrap();
        let rl = handle.as_mut().unwrap();
        {
            rl.set_save_file_data_callback(custom_save_file_data_callback)
                .unwrap();
        }
    }

    pub fn set_file_text_saver(_: &RaylibThread) {
        println!(
            "\n{}\n",
            "Setting file text saver callback".bold().underline(),
        );
        let mut handle = TEST_HANDLE.write().unwrap();
        let rl = handle.as_mut().unwrap();
        {
            rl.set_save_file_text_callback(custom_save_file_text_callback)
                .unwrap();
        }
    }

    pub fn set_file_data_loader(_: &RaylibThread) {
        println!(
            "\n{}\n",
            "Setting file data loader callback".bold().underline(),
        );
        let mut handle = TEST_HANDLE.write().unwrap();
        let rl = handle.as_mut().unwrap();
        {
            rl.set_load_file_data_callback(custom_read_file_data_callback)
                .unwrap();
        }
    }

    pub fn set_file_text_loader(_: &RaylibThread) {}
}
