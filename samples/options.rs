// #[macro_use]
// extern crate structopt;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "example", about = "An example of StructOpt usage.")]
pub struct Opt {
    #[structopt(short = "w", long = "width", default_value = "800")]
    pub width: i32,
    #[structopt(short = "h", long = "height", default_value = "450")]
    pub height: i32,
}

impl Opt {
    pub fn open_window(&self, name: &str) -> (raylib::RaylibHandle, raylib::RaylibThread) {
        raylib::init()
            .size(self.width, self.height)
            .title(name)
            .build()
    }
}
