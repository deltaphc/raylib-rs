#![allow(non_snake_case)]

use std::path::{Path, PathBuf};
use std::{env, fs, io};

// latest version on github's release page
// Unused until 2.5 released
const LATEST_RAYLIB_VERSION: &str = "2.0.0";
const LATEST_RAYLIB_API_VERSION: &str = "2";

/// compile the given src file
fn compile_obj<P>(
    src_dir: &Path,
    name: &str,
    compiler: &Option<&str>,
    glfw_cflags: &Vec<&str>,
    cflags: &Vec<&str>,
    cdefines: &Vec<&str>,
    include_paths: &Vec<P>,
    platform: &Platform,
    graphics: &Option<&str>,
) where
    P: AsRef<Path>,
{
    let mut builder = cc::Build::new();

    if let Some(c) = compiler {
        builder.compiler(c);
    }

    // set glfw flags
    for flag in glfw_cflags {
        builder.flag(flag);
    }

    builder.file(src_dir.join(format!("{}.c", name)));

    for flag in cflags {
        builder.flag(flag);
    }

    for def in cdefines {
        builder.define(def, None);
    }

    for path in include_paths {
        builder.include(path);
    }

    let _ = match platform {
        Platform::Desktop => builder.define("PLATFORM_DESKTOP", None),
        Platform::Web => builder.define("PLATFORM_WEB", None),
        Platform::RPI => builder.define("PLATFORM_RPI", None),
    };

    if let Some(g) = graphics {
        builder.define(g, None);
        builder.define("GL_SILENCE_DEPRECATION", None);
    }
    // We link elsewhere
    builder.cargo_metadata(false);
    builder.warnings(false);
    builder.extra_warnings(false);
    builder.compile(name);
}

/// compile_raylib by doing exactly what the current
/// Makefile does in a rusty way so none unix platforms work
/// we can divide logic into features in the future
fn compile_raylib(raylib_src_path: &Path, target: &str, release: bool) -> BuildSettings {
    dbg!(target);
    // Set compiler defaults
    let mut compiler = None;
    let mut GLFW_CFLAGS = Vec::new();
    let mut CFLAGS = Vec::new();
    let mut CDEFINES = Vec::new();
    let mut INCLUDE_PATHS = Vec::new();
    let mut LDFLAGS = Vec::new();

    // Set configruation defaults
    let SHARED = false; // TODO enabled shared libraries
    let mut bundle_rglfw = false;
    let INCLUDE_AUDIO_MODULE = true;
    let build_raudio_object = INCLUDE_AUDIO_MODULE;
    let build_mini_al_object = false;

    let PLATFORM = if target.contains("wasm32") {
        // make sure cmake knows that it should bundle glfw in
        // Cargo web takes care of this but better safe than sorry
        env::set_var("EMMAKEN_CFLAGS", "-s USE_GLFW=3");
        Platform::Web
    } else {
        Platform::Desktop
    };

    // Library type used for raylib: STATIC (.a) or SHARED (.so/.dll)
    let _RAYLIB_LIBTYPE = LibType::Static;

    // Build mode for library: DEBUG or RELEASE
    // Unused since cc automatically picks between release and debug mode
    let _RAYLIB_BUILD_MODE = if release {
        BuildMode::Release
    } else {
        BuildMode::Debug
    };

    // Included raylib audio module on compilation
    // NOTE: Some programs like tools could not require audio support
    let _INCLUDE_AUDIO_MODULE = true;

    // Use Wayland display server protocol on Linux desktop
    // by default it uses X11 windowing system
    let mut _USE_WAYLAND_DISPLAY = false;

    // TODO BUNCH OF RASPBERRY PI STUFF IN MAKEFILE
    // MAKEFILE LINE 97

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
        PlatformOS::Linux
    } else {
        PlatformOS::Unknown
    };

      dbg!(&PLATFORM);
    dbg!(&PLATFORM_OS);

    // Use external GLFW library instead of rglfw module
    // TODO replace with a feature once target specific features are merged
    // https://github.com/rust-lang/cargo/issues/1197
    let USE_EXTERNAL_GLFW = match PLATFORM_OS {
        PlatformOS::OSX => false,
        PlatformOS::Windows => false,
        _ => true,
    };

    // TODO BUNCH OF ANDROID STUFF IN MAKEFILE
    // MAKEFILE LINE 210

    //Define raylib graphics api depending on selected platform
    let GRAPHICS = match PLATFORM {
        Platform::Desktop => "GRAPHICS_API_OPENGL_33",
        Platform::RPI => "GRAPHICS_API_OPENGL_ES2",
        Platform::Web => "GRAPHICS_API_OPENGL_ES2",
    };

    // raylib prefers clang over cc for building
    if PLATFORM == Platform::Desktop {
        if PLATFORM_OS == PlatformOS::OSX {
            compiler = Some("clang");
            GLFW_CFLAGS.push("-x");
            GLFW_CFLAGS.push("objective-c");
        }
        if PLATFORM_OS == PlatformOS::BSD {
            compiler = Some("clang");
        }
    }

    // TODO BUNCH OF RASPBERRY PI STUFF IN MAKEFILE
    // MAKEFILE LINE 254

    // LINE 254
    if PLATFORM == Platform::Web {
        compiler = Some("emcc");
    }

    // ton of warnings just compiling raylib
    // skip warnings for a bit cleaner output
    // Raysan pls
    CFLAGS.append(&mut vec![
        "-O1",
    ]);
    if PLATFORM_OS == PlatformOS::BSD {
        CFLAGS.push("-Wno-everything");
    }
    if PLATFORM_OS == PlatformOS::Linux {
        CFLAGS.push("-w");
    }
    CDEFINES.append(&mut vec!["_DEFAULT_SOURCE"]);

    // cc crate takes care of this. turn on if we ever use something else
    if !release {
        // CFLAGS.push("-g");
    }

    if PLATFORM == Platform::Desktop && PLATFORM_OS != PlatformOS::Windows {
        CFLAGS.push("-Werror=implicit-function-declaration");
    }

    // Set reasonable defaults here.
    // Cargo Web allows these to be overriden in the final
    // binary.
    if PLATFORM == Platform::Web {
        // custom flags
        CFLAGS.append(&mut vec!["-s", "TOTAL_MEMORY=16777216"]);
        CFLAGS.append(&mut vec![
            "-s",
            "USE_GLFW=3",
            "-s",
            "ASSERTIONS=1",
            "--profiling",
        ]);
        // for bindgen
        CFLAGS.append(&mut vec![
            "-s",
            "WASM=1",
            "-s",
            "RELOCATABLE=1",
            "-s",
            "EMULATED_FUNCTION_POINTERS=1",
        ])
    }

    // MAKEFILE LINE 305
    // TODO BUNCH OF ANDROID STUFF IN MAKEFILE
    // MAKEFILE LINE 322

    // TODO: determine how to handle shared libraries
    if SHARED {
        // CFLAGS.append(&mut vec!["-fPIC", "-DBUILD_LIBTYPE_SHARED"]);
    }

    // Use Wayland display on Linux desktop
    if PLATFORM == Platform::Desktop && PLATFORM_OS == PlatformOS::Linux {
        _USE_WAYLAND_DISPLAY = true;
        CDEFINES.push("_GLFW_WAYLAND");
    }

    // Define include paths for required headers
    // NOTE: Several external required libraries (stb and others)
    INCLUDE_PATHS.push(raylib_src_path.to_owned());
    INCLUDE_PATHS.push(raylib_src_path.join("external/glfw/include"));
    INCLUDE_PATHS.push(raylib_src_path.join("external/glfw/deps/mingw"));

    if PLATFORM == Platform::Desktop {
        if USE_EXTERNAL_GLFW {
            LDFLAGS.push("-lglfw")
        }
    }

    // TODOBUNCH OF ANDROID/PI STUFF IN MAKEFILE  # Define additional directories containing required header files
    // MAKEFILE LINE 371

    if PLATFORM == Platform::Desktop && !USE_EXTERNAL_GLFW {
        bundle_rglfw = true;
    }

    dbg!(bundle_rglfw);
    dbg!(build_raudio_object);
    dbg!(build_mini_al_object);
    // Actually compile stuff.
    let modules = vec![
        "core", "shapes", "textures", "text", "models", "utils", "rglfw", "raudio", "mini_al",
    ];
    let OBJS: Vec<_> = modules
        .iter()
        // filter modules we don't want
        .filter_map(|module| {
            if module == &"rglfw" {
                if bundle_rglfw {
                    return Some((module, GLFW_CFLAGS.clone()));
                } else {
                    return None;
                }
            }
            if module == &"raudio" && !build_raudio_object {
                return None;
            }
            if module == &"mini_al" && !build_mini_al_object {
                return None;
            }
            Some((module, Vec::new()))
        })
        .map(|(module, glfw_cflags)| {
            compile_obj(
                raylib_src_path,
                module,
                &compiler,
                &glfw_cflags,
                &CFLAGS,
                &CDEFINES,
                &INCLUDE_PATHS,
                &PLATFORM,
                &Some(GRAPHICS),
            );
            raylib_src_path.join(format!("{}.o", module))
        })
        .collect();

    // use all the object files and build the static or shared library

    let mut builder = cc::Build::new();
    for obj in OBJS {
        builder.object(obj);
    }

    if let Some(c) = compiler {
        builder.compiler(c);
    }

    builder
        .flag("-compatibility_version")
        .flag(LATEST_RAYLIB_API_VERSION)
        .flag("-current_version")
        .flag(LATEST_RAYLIB_VERSION);
    builder.compile("raylib");

    BuildSettings {
        platform: PLATFORM,
        platform_os: PLATFORM_OS,
        bundled_glfw: bundle_rglfw,
    }
}

fn download_raylib() -> PathBuf {
    let out_dir = env::var("OUT_DIR").unwrap();

    // TODO change this back when 2.5 is released (or whatever the next release is)
    // let raylib_archive_name = format!("{}.tar.gz", LATEST_RAYLIB_VERSION);
    let raylib_archive_name = format!("{}.tar.gz", "master");
    // Unfortunately 2.0.0 fails to build on mac for me due to CMAKE not finding pthread
    // This is fixed in master. As soon as we get a new release we'll change it.
    // It's also a bit less confusing to users of the cheatsheet. 2.0.0 differs somewhat from the website.
    let raylib_archive_url = format!(
        "https://github.com/raysan5/raylib/archive/{}",
        raylib_archive_name
    );

    let raylib_archive_path = Path::new(&out_dir).join(raylib_archive_name);
    // let raylib_build_path = Path::new(&out_dir).join(format!("raylib-{}", LATEST_RAYLIB_VERSION));
    let raylib_build_path = Path::new(&out_dir).join(format!("raylib-{}", "master"));

    // avoid re-downloading the archive if it already exist
    if !raylib_build_path.exists() {
        let raylib_archive = fs::File::create(&raylib_archive_path).unwrap();
        download_to(&raylib_archive_url, &raylib_archive);
    }

    // Uncomment when we go back to tar.gz
    let reader =
        flate2::read::GzDecoder::new(fs::File::open(&raylib_archive_path).unwrap()).unwrap();
    let mut ar = tar::Archive::new(reader);
    ar.unpack(&out_dir).unwrap();

    raylib_build_path
}

fn download_to<T: io::Write>(url: &str, mut dest: T) {
    use io::BufRead;

    dbg!(url);
    let resp = reqwest::get(url).expect(&format!("Failed to GET resource: {:?}", url));
    if !resp.status().is_success() {
        panic!("Download failed with status: {:?}", resp.status());
    }

    let mut src = io::BufReader::new(resp);
    loop {
        let n = {
            let mut buf = src.fill_buf().unwrap();
            dest.write_all(&mut buf).unwrap();
            buf.len()
        };
        if n == 0 {
            break;
        }
        src.consume(n);
    }
}

fn main() {
    let target = env::var("TARGET").expect("Cargo build scripts always have TARGET");
    let release = env::var("PROFILE")
        .expect("Cargo build scripts always have a PROFILE")
        .contains("release");
    let out_dir =
        PathBuf::from(env::var("OUT_DIR").expect("Cargo build scripts always have an OUT_DIR"));

    // TODO if we ever have a shared feature determine whether
    // we download and compile from source here

    let raylib_src_path = download_raylib();
    let BuildSettings {
        platform,
        platform_os,
        bundled_glfw,
    } = compile_raylib(&raylib_src_path.as_path().join("src"), &target, release);

    // Generate bindings
    match (cfg!(feature = "genbindings"), &platform, &platform_os) {
        (false, _, PlatformOS::Windows) => {
            fs::write(
                out_dir.join("bindings.rs"),
                include_str!("bindings_windows.rs"),
            )
            .expect("failed to write bindings");
        }
        (false, _, PlatformOS::Linux) => {
            fs::write(
                out_dir.join("bindings.rs"),
                include_str!("bindings_linux.rs"),
            )
            .expect("failed to write bindings");
        }
        (false, Platform::Web, _) => {
            fs::write(out_dir.join("bindings.rs"), include_str!("bindings_web.rs"))
                .expect("failed to write bindings");
        }
        // for other platforms use bindgen and hope it works
        _ => {
            bindgen::Builder::default()
                .rustfmt_bindings(true)
                .header(format!(
                    "{}",
                    raylib_src_path.join("src/raylib.h").display()
                ))
                .constified_enum_module("*")
                .generate()
                .expect("Failed to generate bindings")
                .write_to_file(out_dir.join("bindings.rs"))
                .expect("Failed to write bindings");
        }
    }

    // Generate cargo metadata for linking to raylib
    if platform == Platform::Desktop {
        if platform_os == PlatformOS::Windows {
            println!(
                "cargo:rustc-link-search=native={}",
                out_dir.join("lib").display()
            );
            println!("cargo:rustc-link-lib=static=raylib");
            println!("cargo:rustc-link-lib=gdi32");
            println!("cargo:rustc-link-lib=user32");
        } else if platform_os == PlatformOS::OSX {
            // On other platforms read raylib.pc with pkg-config
            fs::write(out_dir.join("raylib.pc"), include_str!("raylib.pc"))
                .expect("failed to write pkg-config");
            env::set_var("PKG_CONFIG_PATH", &out_dir);
            pkg_config::Config::new()
                .atleast_version(LATEST_RAYLIB_VERSION)
                .statik(true)
                .arg(format!("--define-variable=prefix={}", out_dir.display()))
                .probe("raylib")
                .unwrap();
        }
        if !bundled_glfw {
            println!("cargo:rustc-link-lib=glfw");
        }
    }
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
