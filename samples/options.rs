// #[macro_use]
// extern crate structopt;

pub use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "example", about = "An example of StructOpt usage.")]
pub struct Opt {
    #[structopt(short = "w", long = "width", default_value = "800")]
    pub width: i32,
    #[structopt(short = "h", long = "height", default_value = "450")]
    pub height: i32,
    #[structopt(long = "fps", default_value = "60")]
    pub fps: u32,
}

impl Opt {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Opt::from_args()
    }
    pub fn open_window(&self, name: &str) -> (raylib::RaylibHandle, raylib::RaylibThread) {
        let (mut rl, thread) = raylib::init()
            .size(self.width, self.height)
            .title(name)
            .build();
        let logo = raylib::prelude::Image::load_image("static/logo.png").unwrap();
        rl.set_window_icon(&logo);
        rl.set_target_fps(self.fps);
        (rl, thread)
    }
}
