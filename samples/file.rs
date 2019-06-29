mod options;

fn main() {
    let opt = options::Opt::new();
    let (_rl, _thread) = opt.open_window("File");
    let (_w, _h) = (opt.width, opt.height);

    println!(
        "Working Dir {:?}",
        raylib::get_directory_files(&raylib::get_working_directory())
    );
}
