use crate::core::*;
use lazy_static::lazy_static;
use std::sync::RwLock;

lazy_static! {
    pub static ref TEST_HANDLE: RwLock<Option<RaylibHandle>> = RwLock::new(Some(
        crate::core::init()
            .size(640, 480)
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
