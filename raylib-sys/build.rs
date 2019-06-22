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

use std::path::{Path, PathBuf};
use std::{env, fs};

/// latest version on github's release page as of time or writing
const LATEST_RAYLIB_VERSION: &str = "2.5.0";
const LATEST_RAYLIB_API_VERSION: &str = "2";

fn build_with_cmake(src_path: &str) {
    let target = env::var("TARGET").expect("Cargo build scripts always have TARGET");
    let (platform, platform_os) = platform_from_target(&target);

    let mut conf = cmake::Config::new(src_path);
    conf.profile("Debug")
        .define("BUILD_EXAMPLES", "OFF")
        .define("BUILD_GAMES", "OFF")
        .define("CMAKE_BUILD_TYPE", "Release")
        // turn off until this is fixed
        .define("SUPPORT_BUSY_WAIT_LOOP", "OFF")
        .define("STATIC", "TRUE");

    match platform {
        Platform::Desktop => conf.define("PLATFORM", "Desktop"),
        Platform::Web => conf.define("PLATFORM", "Web"),
        Platform::RPI => conf.define("PLATFORM", "Raspberry Pi"),
    };

    let c = conf.build();
    // on windows copy the static library to the proper file name
    if platform_os == PlatformOS::Windows {
        std::fs::copy(c.join("lib/raylib_static.lib"), c.join("lib/raylib.lib"))
            .expect("filed to create windows library");
    } // on web copy libraylib.bc to libraylib.a
    if platform == Platform::Web {
        std::fs::copy(c.join("lib/libraylib.bc"), c.join("lib/libraylib.a"))
            .expect("filed to create wasm library");
    }
    println!("cmake build {}", c.display());
    println!("cargo:rustc-link-search=native={}", c.join("lib").display());
}

fn gen_bindings() {
    let target = env::var("TARGET").expect("Cargo build scripts always have TARGET");
    let out_dir =
        PathBuf::from(env::var("OUT_DIR").expect("Cargo build scripts always have an OUT_DIR"));

    let (platform, platform_os) = platform_from_target(&target);

    // Generate bindings
    match (platform, platform_os) {
        (_, PlatformOS::Windows) => {
            fs::write(
                out_dir.join("bindings.rs"),
                include_str!("bindings_windows.rs"),
            )
            .expect("failed to write bindings");
        }
        (_, PlatformOS::Linux) => {
            fs::write(
                out_dir.join("bindings.rs"),
                include_str!("bindings_linux.rs"),
            )
            .expect("failed to write bindings");
        }
        (_, PlatformOS::OSX) => {
            fs::write(out_dir.join("bindings.rs"), include_str!("bindings_osx.rs"))
                .expect("failed to write bindings");
        }
        (Platform::Web, _) => {
            fs::write(out_dir.join("bindings.rs"), include_str!("bindings_web.rs"))
                .expect("failed to write bindings");
        }
        // for other platforms use bindgen and hope it works
        _ => panic!("raylib-rs not supported on your platform"),
    }
}

fn main() {
    let target = env::var("TARGET").expect("Cargo build scripts always have TARGET");
    let (platform, platform_os) = platform_from_target(&target);

    // Donwload raylib source
    let src = download_raylib();
    println!("src path {}", src.display());
    build_with_cmake(src.to_str().expect("failed to download raylib"));

    gen_bindings();
    println!("platform, platform_os {:?}, {:?}", platform, platform_os);

    match platform_os {
        PlatformOS::Windows => {
            println!("cargo:rustc-link-lib=dylib=winmm");
            println!("cargo:rustc-link-lib=dylib=gdi32");
            println!("cargo:rustc-link-lib=dylib=user32");
            println!("cargo:rustc-link-lib=dylib=shell32");
        }
        PlatformOS::Linux => {
            println!("cargo:rustc-link-search=/usr/local/lib");
            println!("cargo:rustc-link-lib=X11");
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
    }

    // if cfg!(target_os = "windows") {
    //     println!("cargo:rustc-link-lib=dylib=gdi32");
    //     println!("cargo:rustc-link-lib=dylib=user32");
    //     println!("cargo:rustc-link-lib=dylib=shell32");
    // }
    // if cfg!(target_os = "linux") {
    //     println!("cargo:rustc-link-search=/usr/local/lib");
    //     println!("cargo:rustc-link-lib=X11");
    // }
    // if cfg!(target_os = "macos") {
    //     println!("cargo:rustc-link-search=native=/usr/local/lib");
    //     println!("cargo:rustc-link-lib=framework=OpenGL");
    //     println!("cargo:rustc-link-lib=framework=Cocoa");
    //     println!("cargo:rustc-link-lib=framework=IOKit");
    //     println!("cargo:rustc-link-lib=framework=CoreFoundation");
    //     println!("cargo:rustc-link-lib=framework=CoreVideo");
    // }

    println!("cargo:rustc-link-lib=static=raylib");
}

/// download_raylib downloads raylib
fn download_raylib() -> PathBuf {
    let out_dir = env::var("OUT_DIR").unwrap();

    let raylib_archive_name = format!("{}.tar.gz", LATEST_RAYLIB_VERSION);
    let raylib_archive_url = format!(
        "https://codeload.github.com/raysan5/raylib/tar.gz/{}",
        LATEST_RAYLIB_VERSION
    );
    println!("out: {:?}", out_dir);
    println!("url: {:?}", raylib_archive_url);

    let raylib_archive_path = Path::new(&out_dir).join(raylib_archive_name);
    let raylib_build_path = Path::new(&out_dir).join(format!("raylib-{}", LATEST_RAYLIB_VERSION));

    // avoid re-downloading the archive if it already exist
    if !raylib_build_path.exists() {
        download_to(
            &raylib_archive_url,
            raylib_archive_path
                .to_str()
                .expect("Download path not stringable"),
        );
    }

    // Uncomment when we go back to tar.gz
    let reader =
        flate2::read::GzDecoder::new(fs::File::open(&raylib_archive_path).unwrap()).unwrap();
    let mut ar = tar::Archive::new(reader);
    ar.unpack(&out_dir).unwrap();

    raylib_build_path
}

/// download_to uses powershell or curl to download raylib to the output directory.
fn download_to(url: &str, dest: &str) {
    run_command("curl", &[url, "-o", dest]);
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
    let platform = if target.contains("wasm32") {
        // make sure cmake knows that it should bundle glfw in
        // Cargo web takes care of this but better safe than sorry
        env::set_var("EMMAKEN_CFLAGS", "-s USE_GLFW=3");
        Platform::Web
    } else if target.contains("armv7-unknown-linux") {
        Platform::RPI
    } else {
        Platform::Desktop
    };

    let platform_os = if platform == Platform::Desktop {
        // Determine PLATFORM_OS in case PLATFORM_DESKTOP selected
        if env::var("OS")
            .unwrap_or("".to_owned())
            .contains("Windows_NT")
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

#[derive(Debug, PartialEq)]
enum Platform {
    Web,
    Desktop,
    RPI, // raspberry pi
}

#[derive(Debug, PartialEq)]
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
