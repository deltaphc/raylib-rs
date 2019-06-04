/* raylib-sys
   build.rs - Cargo build script

Copyright (c) 2018-2019 Paul Clement (@deltaphc)

This software is provided "as-is", without any express or implied warranty. In no event will the authors be held liable for any damages arising from the use of this software.

Permission is granted to anyone to use this software for any purpose, including commercial applications, and to alter it and redistribute it freely, subject to the following restrictions:

  1. The origin of this software must not be misrepresented; you must not claim that you wrote the original software. If you use this software in a product, an acknowledgment in the product documentation would be appreciated but is not required.

  2. Altered source versions must be plainly marked as such, and must not be misrepresented as being the original software.

  3. This notice may not be removed or altered from any source distribution.
*/

use std::path::{PathBuf};
use std::{env, fs};

/// latest version on github's release page as of time or writing
const LATEST_RAYLIB_VERSION: &str = "2.5.0";
const LATEST_RAYLIB_API_VERSION: &str = "2";


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
        _ => {
            panic!("raylib-rs not supported on your platform")
        }
    }
}

fn main() {
    gen_bindings();
    
    if cfg!(target_os = "windows") {
        println!("cargo:rustc-link-lib=dylib=gdi32");
        println!("cargo:rustc-link-lib=dylib=user32");
    }
    if cfg!(target_os = "linux") {
        println!("cargo:rustc-link-search=/usr/local/lib");
        println!("cargo:rustc-link-lib=X11");
    }
    if cfg!(target_os = "macos") {
        println!("cargo:rustc-link-search=native=/usr/local/lib");
        println!("cargo:rustc-link-lib=framework=OpenGL");
        println!("cargo:rustc-link-lib=framework=Cocoa");
        println!("cargo:rustc-link-lib=framework=IOKit");
        println!("cargo:rustc-link-lib=framework=CoreFoundation");
        println!("cargo:rustc-link-lib=framework=CoreVideo");
    }
    println!("cargo:rustc-link-lib=static=raylib");
}

fn platform_from_target(target: &str) -> (Platform, PlatformOS) {
    let PLATFORM = if target.contains("wasm32") {
        // make sure cmake knows that it should bundle glfw in
        // Cargo web takes care of this but better safe than sorry
        env::set_var("EMMAKEN_CFLAGS", "-s USE_GLFW=3");
        Platform::Web
    } else if target.contains("armv7-unknown-linux") {
        Platform::RPI
    } else {
        Platform::Desktop
    };

    let PLATFORM_OS = if PLATFORM == Platform::Desktop {
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
    } else if PLATFORM == Platform::RPI {
        let un: &str = &uname();
        if un == "Linux" {
            PlatformOS::Linux
        } else {
            PlatformOS::Unknown
        }
    } else {
        PlatformOS::Unknown
    };
    
    (PLATFORM, PLATFORM_OS)
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
