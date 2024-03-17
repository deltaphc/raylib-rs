use crate::test::{TestDescAndFn, TestFn};
use colored::Colorize;
use lazy_static::lazy_static;
use raylib::prelude::*;
use std::sync::RwLock;
use test::test::parse_opts;

pub const TEST_WIDTH: i32 = 640;
pub const TEST_HEIGHT: i32 = 480;

lazy_static! {
    pub static ref TEST_HANDLE: RwLock<Option<RaylibHandle>> = RwLock::new(None);
}

/// Bunch of iniialized assets for used in drawing
pub struct TestAssets {
    pub font: Font,
    pub font_ex: Font,
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

pub fn initialize_globals() -> (RaylibThread, TestAssets) {
    let mut handle = TEST_HANDLE.write().unwrap();
    let (rl, thread) = raylib::init()
        .size(TEST_WIDTH, TEST_HEIGHT)
        .title("Hello, World")
        .build();
    *handle = Some(rl);
    let asset = TestAssets {
        font: handle
            .as_mut()
            .unwrap()
            .load_font(&thread, "resources/alagard.png")
            .expect("couldn't load font"),
        font_ex: handle
            .as_mut()
            .unwrap()
            .load_font_ex(&thread, "resources/pixeloid.ttf", 32, None)
            .expect("couldn't load font"),
    };
    (thread, asset)
}

#[cfg(feature = "automation_event_test")]
pub fn test_runner(tests: &[&dyn Testable]) {
    use crate::automation::automation_test::automation_test;

    let (thread, assets) = initialize_globals();
    let args = std::env::args().collect::<Vec<_>>();
    let opts = match parse_opts(&args) {
        Some(Ok(o)) => o,
        Some(Err(msg)) => panic!("{:?}", msg),
        None => return,
    };

    automation_test(&thread);
}

#[cfg(feature = "custom_frame_control")]
pub fn test_runner(tests: &[&dyn Testable]) {
    use crate::manual::manual_test::test_manual;

    let (thread, assets) = initialize_globals();
    let args = std::env::args().collect::<Vec<_>>();
    let opts = match parse_opts(&args) {
        Some(Ok(o)) => o,
        Some(Err(msg)) => panic!("{:?}", msg),
        None => return,
    };

    test_manual(&thread);
}
#[cfg(not(feature = "automation_event_test"))]
#[cfg(not(feature = "custom_frame_control"))]
pub fn test_runner(tests: &[&dyn Testable]) {
    use crate::callbacks;

    let (thread, assets) = initialize_globals();

    callbacks::callback_tests::set_logger(&thread);
    callbacks::callback_tests::set_file_data_saver(&thread);
    callbacks::callback_tests::set_file_text_saver(&thread);
    callbacks::callback_tests::set_file_data_loader(&thread);
    callbacks::callback_tests::set_file_text_loader(&thread);

    let args = std::env::args().collect::<Vec<_>>();
    let opts = match parse_opts(&args) {
        Some(Ok(o)) => o,
        Some(Err(msg)) => panic!("{:?}", msg),
        None => return,
    };

    let mut par_test: Vec<TestDescAndFn> = Vec::new();
    let mut seq_test: Vec<&RayTest> = Vec::new();
    let mut draw_test: Vec<&RayDrawTest> = Vec::new();
    let mut draw_test_3d: Vec<&Ray3DDrawTest> = Vec::new();

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
            TestType::Draw3D(test) => {
                draw_test_3d.push(test);
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
        (t.test)(&thread);
    }

    let mut handle = TEST_HANDLE.write().unwrap();
    let mut rl = handle.take().unwrap();

    rl.set_target_fps(120);
    // let sleep_time = std::time::Duration::from_millis(1000); // about 60 fps
    {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);
    }
    for t in &draw_test {
        if opts.nocapture {
            println!("running draw test: {}", t.name);
        }
        {
            let mut d = rl.begin_drawing(&thread);
            (t.test)(&mut d, &assets);
        }

        // take_screenshot takes the last frames screenshot
        rl.take_screenshot(&thread, &format!("{}.png", t.name));
        {
            let mut d = rl.begin_drawing(&thread);
            d.clear_background(Color::WHITE);
        }
        //assert!(std::path::Path::new(&format!("{}.png", t.name)).exists());
    }
    let camera = Camera3D::orthographic(
        Vector3::new(-125.0, 125.0, 125.0),
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(0.0, 1.0, 0.0),
        90.0,
    );
    for t in &draw_test_3d {
        if opts.nocapture {
            println!("running draw test: {}", t.name);
        }
        {
            let mut d_ = rl.begin_drawing(&thread);
            let mut d = d_.begin_mode3D(&camera);
            d.clear_background(Color::GRAY);
            (t.test)(&mut d, &thread, &assets);
        }
        // take_screenshot takes the last frames screenshot
        rl.take_screenshot(&thread, &format!("{}.png", t.name));
        {
            let mut d_ = rl.begin_drawing(&thread);
            let mut d = d_.begin_mode3D(&camera);
            d.clear_background(Color::GRAY);
        }
        //assert!(std::path::Path::new(&format!("{}.png", t.name)).exists());
    }
    println!("{}","Test has succeeded! You will see that the test has failed due to a segfault, this is a known bug. If you are seeing this message then it definitely has succeeded!".green().bold());
}

pub enum TestType<'a> {
    Normal(&'a TestDescAndFn),
    /// Force this test to be run on the same thread as the window
    Local(&'a RayTest),
    /// take screenshot after test
    Draw(&'a RayDrawTest),
    /// take screenshot after test (3D),
    Draw3D(&'a Ray3DDrawTest),
}

pub struct RayTest {
    pub name: &'static str,
    pub test: fn(&RaylibThread),
}

pub struct RayDrawTest {
    pub name: &'static str,
    pub test: fn(&mut RaylibDrawHandle, &TestAssets),
}

pub struct Ray3DDrawTest {
    pub name: &'static str,
    pub test: fn(&mut RaylibMode3D<RaylibDrawHandle>, &RaylibThread, &TestAssets),
}

macro_rules! ray_test {
    ($name:ident) => {
        #[test_case]
        #[allow(non_upper_case_globals)]
        static $name: RayTest = RayTest {
            name: stringify!($name),
            test: $name,
        };
    };
}

macro_rules! ray_draw_test {
    ($name:ident) => {
        #[test_case]
        #[allow(non_upper_case_globals)]
        static $name: RayDrawTest = RayDrawTest {
            name: stringify!($name),
            test: $name,
        };
    };
}

macro_rules! ray_3d_draw_test {
    ($name:ident) => {
        #[test_case]
        #[allow(non_upper_case_globals)]
        static $name: Ray3DDrawTest = Ray3DDrawTest {
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

impl Testable for Ray3DDrawTest {
    fn get_test(&self) -> TestType {
        TestType::Draw3D(self)
    }
}
