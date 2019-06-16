use crate::core::*;
use crate::test::{TestDescAndFn, TestFn};
use lazy_static::lazy_static;
use std::sync::RwLock;

pub const TEST_WIDTH: i32 = 640;
pub const TEST_HEIGHT: i32 = 480;

lazy_static! {
    pub static ref TEST_HANDLE: RwLock<Option<RaylibHandle>> = RwLock::new(None);
}

/// We need to build our own slice of test descriptors to pass to `test::test_main`. We cannot
/// clone `TestFn`, so we do it via matching on variants. Not sure how to handle `Dynamic*` variants,
/// but we seem not to get them here anyway?.
fn clone_testfn(testfn: &TestFn) -> TestFn {
    match testfn {
        TestFn::StaticTestFn(func) => TestFn::StaticTestFn(*func),
        TestFn::StaticBenchFn(bench) => TestFn::StaticBenchFn(*bench),
        _ => unimplemented!("only static functions are supported"),
    }
}

pub fn test_runner(tests: &[&Testable]) {
    let local = {
        // I know this is necessary but I don't know why it is necessary. Maybe the few nanoseconds it takes to lock this mutex help?
        let mut handle = TEST_HANDLE.write().unwrap();
        let (rl, local) = crate::core::init()
            .size(TEST_WIDTH, TEST_HEIGHT)
            .title("Hello, World")
            .build();
        *handle = Some(rl);
        local
    };

    let args = std::env::args().collect::<Vec<_>>();
    let mut opts = match test::parse_opts(&args) {
        Some(Ok(o)) => o,
        Some(Err(msg)) => panic!("{:?}", msg),
        None => return,
    };

    let mut par_test: Vec<TestDescAndFn> = Vec::new();
    let mut seq_test: Vec<&RayTest> = Vec::new();
    let mut draw_test: Vec<&RayDrawTest> = Vec::new();

    for t in tests {
        match t.get_test() {
            TestType::Normal(test) => par_test.push(TestDescAndFn {
                desc: test.desc.clone(),
                testfn: clone_testfn(&test.testfn),
            }),
            TestType::Local(test) => {
                seq_test.push(test);
            }
            TestType::Draw(test) => {
                draw_test.push(test);
            }
        }
    }

    match crate::test::run_tests_console(&opts, par_test) {
        Ok(true) => {}
        Ok(false) => panic!("Some tests failed"),
        Err(e) => panic!("io error when running tests: {:?}", e),
    }

    // Run seq test manually
    // TODO properly handle test functions
    for t in seq_test {
        if opts.nocapture {
            println!("running {}", t.name);
        }
        (t.test)(&local);
    }

    let mut handle = TEST_HANDLE.write().unwrap();
    let mut rl = handle.take().unwrap();

    rl.set_target_fps(120);
    rl.unhide_window();
    let sleep_time = std::time::Duration::from_millis(1000); // about 60 fps
    rl.with_draw(&local, |d| {
        d.clear_background(Color::WHITE);
    });
    for t in &draw_test {
        if opts.nocapture {
            println!("running draw test: {}", t.name);
        }
        rl.with_draw(&local, |mut d| {
            (t.test)(&mut d);
        });
        rl.with_draw(&local, |d| {
            d.clear_background(Color::WHITE);
        });
        // take_screenshot takes the last frames screenshot
        rl.take_screenshot(&local, &format!("test_out/{}.png", t.name));
    }
}

pub enum TestType<'a> {
    Normal(&'a TestDescAndFn),
    /// Force this test to be run on the same thread as the window
    Local(&'a RayTest),
    /// take screenshot after test
    Draw(&'a RayDrawTest),
}

pub struct RayTest {
    pub name: &'static str,
    pub test: fn(&RaylibThread),
}

pub struct RayDrawTest {
    pub name: &'static str,
    pub test: fn(&mut RaylibDrawHandle),
}

macro_rules! ray_test {
    ($name:ident) => {
        #[test_case]
        static $name: RayTest = RayTest {
            name: stringify!($name),
            test: $name,
        };
    };
}

macro_rules! ray_draw_test {
    ($name:ident) => {
        #[test_case]
        static $name: RayDrawTest = RayDrawTest {
            name: stringify!($name),
            test: $name,
        };
    };
}

pub trait Testable {
    fn get_test(&self) -> TestType;
}

impl Testable for TestDescAndFn {
    fn get_test(&self) -> TestType {
        TestType::Normal(self)
    }
}

impl Testable for RayTest {
    fn get_test(&self) -> TestType {
        TestType::Local(self)
    }
}

impl Testable for RayDrawTest {
    fn get_test(&self) -> TestType {
        TestType::Draw(self)
    }
}
