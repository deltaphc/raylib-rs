use raylib::prelude::*;

mod options;

fn main() {
    let opt = options::Opt::new();
    let (mut rl, thread) = opt.open_window("File");
    let (w, h) = (opt.width, opt.height);

    println!(
        "Working Dir {:?}",
        raylib::get_directory_files(&raylib::get_working_directory())
    );
}
