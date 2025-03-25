/* raylib-sys
   build.rs - Cargo build script

Copyright (c) 2018-2019 Paul Clement (@deltaphc)

This software is provided "as-is", without any express or implied warranty. In no event will the authors be held liable for any damages arising from the use of this software.

Permission is granted to anyone to use this software for any purpose, including commercial applications, and to alter it and redistribute it freely, subject to the following restrictions:

  1. The origin of this software must not be misrepresented; you must not claim that you wrote the original software. If you use this software in a product, an acknowledgment in the product documentation would be appreciated but is not required.

  2. Altered source versions must be plainly marked as such, and must not be misrepresented as being the original software.

  3. This notice may not be removed or altered from any source distribution.
*/
#![allow(dead_code)]

extern crate bindgen;

use std::collections::HashSet;
use std::env;
use std::path::{Path, PathBuf};

/// latest version on github's release page as of time or writing
const LATEST_RAYLIB_VERSION: &str = "5.0.0";
const LATEST_RAYLIB_API_VERSION: &str = "5";

#[derive(Debug)]
struct IgnoreMacros(HashSet<String>);

impl bindgen::callbacks::ParseCallbacks for IgnoreMacros {
    fn will_parse_macro(&self, name: &str) -> bindgen::callbacks::MacroParsingBehavior {
        if self.0.contains(name) {
            bindgen::callbacks::MacroParsingBehavior::Ignore
        } else {
            bindgen::callbacks::MacroParsingBehavior::Default
        }
    }
}
#[cfg(feature = "nobuild")]
fn build_with_cmake(_src_path: &str) {}

#[cfg(not(feature = "nobuild"))]
fn build_with_cmake(src_path: &str) {
    // CMake uses different lib directories on different systems.
    // I do not know how CMake determines what directory to use,
    // so we will check a few possibilities and use whichever is present.
    fn join_cmake_lib_directory(path: PathBuf) -> PathBuf {
        let possible_cmake_lib_directories = ["lib", "lib64", "lib32"];
        for lib_directory in &possible_cmake_lib_directories {
            let lib_path = path.join(lib_directory);
            if lib_path.exists() {
                return lib_path;
            }
        }
        path
    }

    let target = env::var("TARGET").expect("Cargo build scripts always have TARGET");

    let (platform, platform_os) = platform_from_target(&target);

    let mut conf = cmake::Config::new(src_path);
    let mut builder;
    let mut profile = "";
    #[cfg(debug_assertions)]
    {
        builder = conf.profile("Debug");
        builder = builder.define("CMAKE_BUILD_TYPE", "Debug");
        profile = "Debug";
    }

    #[cfg(not(debug_assertions))]
    {
        builder = conf.profile("Release");
        builder = builder.define("CMAKE_BUILD_TYPE", "Release");
        profile = "Release";
    }

    builder
        .define("BUILD_EXAMPLES", "OFF")
        .define("CMAKE_BUILD_TYPE", profile)
        // turn off until this is fixed
        .define("SUPPORT_BUSY_WAIT_LOOP", "OFF")
        .define("SUPPORT_FILEFORMAT_JPG", "ON");

    #[cfg(feature = "custom_frame_control")]
    {
        builder.define("SUPPORT_CUSTOM_FRAME_CONTROL", "ON");
    }

    // Enable wayland cmake flag if feature is specified
    #[cfg(feature = "wayland")]
    {
        builder.define("USE_WAYLAND", "ON");
        builder.define("USE_EXTERNAL_GLFW", "ON"); // Necessary for wayland support in my testing
    }

    // This seems redundant, but I felt it was needed incase raylib changes it's default
    #[cfg(not(feature = "wayland"))]
    builder.define("USE_WAYLAND", "OFF");

    // Scope implementing flags for forcing OpenGL version
    // See all possible flags at https://github.com/raysan5/raylib/wiki/CMake-Build-Options
    {
        #[cfg(feature = "opengl_33")]
        builder.define("OPENGL_VERSION", "3.3");

        #[cfg(feature = "opengl_21")]
        builder.define("OPENGL_VERSION", "2.1");

        // #[cfg(feature = "opengl_11")]
        // builder.define("OPENGL_VERSION", "1.1");

        #[cfg(feature = "opengl_es_20")]
        {
            builder.define("OPENGL_VERSION", "ES 2.0");
            println!("cargo:rustc-link-lib=GLESv2");
            println!("cargo:rustc-link-lib=GLdispatch");
        }

        #[cfg(feature = "opengl_es_30")]
        {
            builder.define("OPENGL_VERSION", "ES 3.0");
            println!("cargo:rustc-link-lib=GLESv2");
            println!("cargo:rustc-link-lib=GLdispatch");
        }

        // Once again felt this was necessary incase a default was changed :)
        #[cfg(not(any(
            feature = "opengl_33",
            feature = "opengl_21",
            // feature = "opengl_11",
            feature = "opengl_es_20",
            feature = "opengl_es_30"
        )))]
        builder.define("OPENGL_VERSION", "OFF");
    }

    // Allows disabling the default maping of screenshot and gif recording in raylib
    {
        #[cfg(any(feature = "noscreenshot", feature = "nogif"))]
        builder.define("CUSTOMIZE_BUILD", "ON");

        #[cfg(feature = "noscreenshot")]
        builder.define("SUPPORT_SCREEN_CAPTURE", "OFF");
        #[cfg(feature = "nogif")]
        builder.define("SUPPORT_GIF_RECORDING", "OFF");

        // Once again felt this was necessary incase a default was changed :)
        #[cfg(not(feature = "noscreenshot"))]
        builder.define("SUPPORT_SCREEN_CAPTURE", "ON");
        #[cfg(not(feature = "nogif"))]
        builder.define("SUPPORT_GIF_RECORDING", "ON");
    }

    match platform {
        Platform::Desktop => {
            #[cfg(feature = "sdl")]
            {
                println!("cargo:rustc-link-lib=SDL2");
                conf.define("PLATFORM", "SDL")
            }
            #[cfg(not(feature = "sdl"))]
            {
                conf.define("PLATFORM", "Desktop")
            }
        }
        Platform::Web => conf.define("PLATFORM", "Web"),
        Platform::RPI => conf.define("PLATFORM", "Raspberry Pi"),
        Platform::Android => {
            // get required env variables
            let android_ndk_home = env::var("ANDROID_NDK_HOME")
                .expect("Please set the environment variable: ANDROID_NDK_HOME:(e.g /home/u/Android/Sdk/ndk/VXXX/)");
            let android_platform = target.split("-").last().expect("fail to parse the android version of the target triple, example:'aarch64-linux-android25'");
            let abi_version = android_platform
                .split("-")
                .last()
                .expect("Could not get abi version. Is ANDROID_PLATFORM valid?");
            let toolchain_file =
                format!("{}/build/cmake/android.toolchain.cmake", &android_ndk_home);
            // Detect ANDROID_ABI using the target triple
            let android_arch_abi = match target.as_str() {
                "aarch64-linux-android" => "arm64-v8a",
                "armv7-linux-androideabi" => "armeabi-v7a",
                _ => panic!("Unsupported target triple for Android"),
            };
            // we'll set as many variables as possible according to:
            // https://developer.android.com/ndk/guides/cmake#command-line_1
            // https://cmake.org/cmake/help/v3.31/manual/cmake-toolchains.7.html#cross-compiling-for-android-with-the-ndk
            // how to build:
            // 0) set the correct linker in your game project's Cargo.toml (note: platform number should match):
            // [target.aarch64-linux-android]
            // linker = "/home/u/Android/Sdk/ndk/28.0.12433566/toolchains/llvm/prebuilt/linux-x86_64/bin/aarch64-linux-android<PLATFORM_NUMBER>-clang"

            // 1) set env variable ANDROID_NDK_HOME
            // 2) compile with: `cargo ndk -t <ARCH> -p <PLATFORM_NUMBER> -o ./jniLibs build`
            // example(cargo ndk -t arm64-v8a -p 25 -o ./jniLibs build)

            conf.define("CMAKE_SYSTEM_NAME", "Android")
                .define("PLATFORM", "Android")
                .define("CMAKE_SYSTEM_VERSION", abi_version)
                .define("ANDROID_ABI", android_arch_abi)
                .define("CMAKE_ANDROID_ARCH_ABI", android_arch_abi)
                .define("CMAKE_ANDROID_NDK", &android_ndk_home)
                .define("ANDROID_PLATFORM", android_platform)
                .define("CMAKE_TOOLCHAIN_FILE", &toolchain_file)
        }
    };

    let dst = conf.build();
    let dst_lib = join_cmake_lib_directory(dst);
    // on windows copy the static library to the proper file name
    if platform_os == PlatformOS::Windows {
        if Path::new(&dst_lib.join("raylib.lib")).exists() {
            // DO NOTHING
        } else if Path::new(&dst_lib.join("raylib_static.lib")).exists() {
            std::fs::copy(
                dst_lib.join("raylib_static.lib"),
                dst_lib.join("raylib.lib"),
            )
            .expect("failed to create windows library");
        } else if Path::new(&dst_lib.join("libraylib_static.a")).exists() {
            std::fs::copy(
                dst_lib.join("libraylib_static.a"),
                dst_lib.join("libraylib.a"),
            )
            .expect("failed to create windows library");
        } else if Path::new(&dst_lib.join("libraylib.a")).exists() {
            // DO NOTHING
        } else {
            panic!("failed to create windows library");
        }
    } // on web copy libraylib.bc to libraylib.a
    if platform == Platform::Web && !Path::new(&dst_lib.join("libraylib.a")).exists() {
        std::fs::copy(dst_lib.join("libraylib.bc"), dst_lib.join("libraylib.a"))
            .expect("failed to create wasm library");
    }
    // println!("cmake build {}", c.display());
    println!("cargo:rustc-link-search=native={}", dst_lib.display());
    if platform == Platform::Android {
        println!("cargo:rustc-link-lib=log");
        println!("cargo:rustc-link-lib=android");
        println!("cargo:rustc-link-lib=EGL");
        println!("cargo:rustc-link-lib=GLESv2");
        println!("cargo:rustc-link-lib=OpenSLES");
        println!("cargo:rustc-link-lib=c");
        println!("cargo:rustc-link-lib=m");
    }
}

fn gen_bindings() {
    let target = env::var("TARGET").expect("Cargo build scripts always have TARGET");
    let (platform, os) = platform_from_target(&target);

    let plat = match platform {
        Platform::Desktop => "-DPLATFORM_DESKTOP",
        Platform::RPI => "-DPLATFORM_RPI",
        Platform::Android => "-DPLATFORM_ANDROID",
        Platform::Web => "-DPLATFORM_WEB",
    };

    let ignored_macros = IgnoreMacros(
        vec![
            "FP_INFINITE".into(),
            "FP_NAN".into(),
            "FP_NORMAL".into(),
            "FP_SUBNORMAL".into(),
            "FP_ZERO".into(),
            "IPPORT_RESERVED".into(),
        ]
        .into_iter()
        .collect(),
    );

    let mut builder = bindgen::Builder::default()
        .header("binding/binding.h")
        .rustified_enum(".+")
        .clang_arg("-std=c99")
        .clang_arg(plat)
        .parse_callbacks(Box::new(ignored_macros));

    #[cfg(feature = "imgui")]
    {
        builder = builder
            .clang_arg("-I./binding/imgui/decoy")
            .clang_arg("-I./raylib/src")
            .header("binding/rlImGui/rlImGui.h");
    }

    if platform == Platform::Desktop && os == PlatformOS::Windows {
        // odd workaround for booleans being broken
        builder = builder.clang_arg("-D__STDC__");
    }

    if platform == Platform::Web {
        builder = builder
            .clang_arg("-fvisibility=default")
            .clang_arg("--target=wasm32-emscripten");
    }

    // Build
    let bindings = builder.generate().expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

fn gen_rgui() {
    // Compile the code and link with cc crate
    #[cfg(target_os = "windows")]
    {
        cc::Build::new()
            .files(vec!["binding/rgui_wrapper.cpp", "binding/utils_log.cpp"])
            .include("binding")
            .warnings(false)
            // .flag("-std=c99")
            .extra_warnings(false)
            .compile("rgui");
    }
    #[cfg(not(target_os = "windows"))]
    {
        cc::Build::new()
            .files(vec!["binding/rgui_wrapper.c", "binding/utils_log.c"])
            .include("binding")
            .warnings(false)
            // .flag("-std=c99")
            .extra_warnings(false)
            .compile("rgui");
    }
}

fn gen_imgui() {
    println!("cargo:rustc-link-lib=dylib=stdc++");

    cc::Build::new()
        .define("NO_FONT_AWESOME", "1")
        .files(vec!["binding/rlImGui/rlImGui.cpp"])
        .include("binding/imgui")
        .include("raylib/src")
        .warnings(false)
        .extra_warnings(false)
        .compile("rlImGui");
}

#[cfg(feature = "nobuild")]
fn link(_platform: Platform, _platform_os: PlatformOS) {}

#[cfg(not(feature = "nobuild"))]
fn link(platform: Platform, platform_os: PlatformOS) {
    match platform_os {
        PlatformOS::Windows => {
            println!("cargo:rustc-link-lib=dylib=winmm");
            println!("cargo:rustc-link-lib=dylib=gdi32");
            println!("cargo:rustc-link-lib=dylib=user32");
            println!("cargo:rustc-link-lib=dylib=shell32");
        }
        PlatformOS::Linux => {
            // X11 linking
            #[cfg(not(feature = "wayland"))]
            {
                println!("cargo:rustc-link-search=/usr/local/lib");
                println!("cargo:rustc-link-lib=X11");
            }

            // Wayland linking
            #[cfg(feature = "wayland")]
            {
                println!("cargo:rustc-link-search=/usr/local/lib");
                println!("cargo:rustc-link-lib=wayland-client");
                println!("cargo:rustc-link-lib=glfw"); // Link against locally installed glfw
            }
        }
        PlatformOS::OSX => {
            println!("cargo:rustc-link-search=native=/usr/local/lib");
            println!("cargo:rustc-link-lib=framework=OpenGL");
            println!("cargo:rustc-link-lib=framework=Cocoa");
            println!("cargo:rustc-link-lib=framework=IOKit");
            println!("cargo:rustc-link-lib=framework=CoreFoundation");
            println!("cargo:rustc-link-lib=framework=CoreVideo");
        }
        _ => (),
    }
    if platform == Platform::Web {
        println!("cargo:rustc-link-lib=glfw");
    } else if platform == Platform::RPI {
        println!("cargo:rustc-link-search=/opt/vc/lib");
        println!("cargo:rustc-link-lib=bcm_host");
        println!("cargo:rustc-link-lib=brcmEGL");
        println!("cargo:rustc-link-lib=brcmGLESv2");
        println!("cargo:rustc-link-lib=vcos");
    }

    println!("cargo:rustc-link-lib=static=raylib");
}

#[cfg(not(feature = "nobuild"))]
fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=./binding/binding.h");
    let target = env::var("TARGET").expect("Cargo build scripts always have TARGET");

    if target.contains("wasm32-unknown-emscripten") {
        if let Err(e) = env::var("EMCC_CFLAGS") {
            if e == std::env::VarError::NotPresent {
                panic!("\nYou must have to set EMCC_CFLAGS yourself to compile for WASM.\n{}{}\"\n",{
                    #[cfg(target_family = "windows")]
                    {"set EMCC_CFLAGS="}
                    #[cfg(not(target_family = "windows"))]
                    {"export EMCC_CFLAGS="}
                },"\"-O3 -sUSE_GLFW=3 -sASSERTIONS=1 -sWASM=1 -sASYNCIFY -sGL_ENABLE_GET_PROC_ADDRESS=1\"");
            } else {
                panic!("\nError regarding EMCC_CFLAGS: {:?}\n", e);
            }
        }
    }

    let (platform, platform_os) = platform_from_target(&target);

    let raylib_src = "./raylib";
    if is_directory_empty(raylib_src) {
        panic!("raylib source does not exist in: `raylib-sys/raylib`. Please copy it in");
    }
    build_with_cmake(raylib_src);

    gen_bindings();

    link(platform, platform_os);

    gen_rgui();

    #[cfg(feature = "imgui")]
    gen_imgui();
}

#[cfg(feature = "nobuild")]
fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=./binding/binding.h");
    let target = env::var("TARGET").expect("Cargo build scripts always have TARGET");

    if target.contains("wasm32-unknown-emscripten") {
        if let Err(e) = env::var("EMCC_CFLAGS") {
            if e == std::env::VarError::NotPresent {
                panic!("\nYou must have to set EMCC_CFLAGS yourself to compile for WASM.\n{}{}\"\n",{
                    #[cfg(target_family = "windows")]
                    {"set EMCC_CFLAGS="}
                    #[cfg(not(target_family = "windows"))]
                    {"export EMCC_CFLAGS="}
                },"\"-O3 -sUSE_GLFW=3 -sASSERTIONS=1 -sWASM=1 -sASYNCIFY -sGL_ENABLE_GET_PROC_ADDRESS=1\"");
            } else {
                panic!("\nError regarding EMCC_CFLAGS: {:?}\n", e);
            }
        }
    }

    #[cfg(feature = "bindgen")]
    gen_bindings();

    gen_rgui();

    #[cfg(feature = "imgui")]
    gen_imgui();
}

#[must_use]
/// returns false if the directory does not exist
fn is_directory_empty(path: &str) -> bool {
    match std::fs::read_dir(path) {
        Ok(mut dir) => dir.next().is_none(),
        Err(_) => true,
    }
}

// run_command runs a command to completion or panics. Used for running curl and powershell.
fn run_command(cmd: &str, args: &[&str]) {
    use std::process::Command;
    match Command::new(cmd).args(args).output() {
        Ok(output) => {
            if !output.status.success() {
                let error = std::str::from_utf8(&output.stderr).unwrap();
                panic!("Command '{}' failed: {}", cmd, error);
            }
        }
        Err(error) => {
            panic!("Error running command '{}': {:#}", cmd, error);
        }
    }
}

fn platform_from_target(target: &str) -> (Platform, PlatformOS) {
    let platform = if target.contains("wasm") {
        Platform::Web
    } else if target.contains("armv7-unknown-linux") {
        Platform::RPI
    } else if target.contains("android") {
        Platform::Android
    } else {
        Platform::Desktop
    };

    let platform_os = if platform == Platform::Desktop {
        // Determine PLATFORM_OS in case PLATFORM_DESKTOP selected
        if env::var("OS")
            .unwrap_or("".to_owned())
            .contains("Windows_NT")
            || env::var("TARGET")
                .unwrap_or("".to_owned())
                .contains("windows")
        {
            // No uname.exe on MinGW!, but OS=Windows_NT on Windows!
            // ifeq ($(UNAME),Msys) -> Windows
            PlatformOS::Windows
        } else {
            let un: &str = &uname();
            match un {
                "Linux" => PlatformOS::Linux,
                "FreeBSD" => PlatformOS::BSD,
                "OpenBSD" => PlatformOS::BSD,
                "NetBSD" => PlatformOS::BSD,
                "DragonFly" => PlatformOS::BSD,
                "Darwin" => PlatformOS::OSX,
                _ => panic!("Unknown platform {}", uname()),
            }
        }
    } else if platform == Platform::RPI {
        let un: &str = &uname();
        if un == "Linux" {
            PlatformOS::Linux
        } else {
            PlatformOS::Unknown
        }
    } else {
        PlatformOS::Unknown
    };

    (platform, platform_os)
}

fn uname() -> String {
    use std::process::Command;
    String::from_utf8_lossy(
        &Command::new("uname")
            .output()
            .expect("failed to run uname")
            .stdout,
    )
    .trim()
    .to_owned()
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Platform {
    Web,
    Desktop,
    Android,
    RPI, // raspberry pi
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum PlatformOS {
    Windows,
    Linux,
    BSD,
    OSX,
    Unknown,
}

#[derive(Debug, PartialEq)]
enum LibType {
    Static,
    _Shared,
}

#[derive(Debug, PartialEq)]
enum BuildMode {
    Release,
    Debug,
}

struct BuildSettings {
    pub platform: Platform,
    pub platform_os: PlatformOS,
    pub bundled_glfw: bool,
}
