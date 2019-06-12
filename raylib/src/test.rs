use crate::core::*;
use lazy_static::lazy_static;
use std::sync::RwLock;

pub const TEST_WIDTH: i32 = 640;
pub const TEST_HEIGHT: i32 = 480;

lazy_static! {
    pub static ref TEST_HANDLE: RwLock<Option<RaylibHandle>> = RwLock::new(Some(
        crate::core::init()
            .size(TEST_WIDTH, TEST_HEIGHT)
            .title("Hello, World")
            .build()
    ));
}

pub fn test_runner(ts: &[&dyn Fn() -> ()]) {
    println!("Running {} raylib test in custom framework", ts.len());
    // TODO run test in parallel
    for t in ts {
        t();
    }
    let _rl = TEST_HANDLE.write().unwrap().take();
    // drop handle here
}
