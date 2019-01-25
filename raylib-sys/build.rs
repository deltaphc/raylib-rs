/* raylib-sys
   build.rs - Cargo build script

Copyright (c) 2018-2019 Paul Clement (@deltaphc)

This software is provided "as-is", without any express or implied warranty. In no event will the authors be held liable for any damages arising from the use of this software.

Permission is granted to anyone to use this software for any purpose, including commercial applications, and to alter it and redistribute it freely, subject to the following restrictions:

  1. The origin of this software must not be misrepresented; you must not claim that you wrote the original software. If you use this software in a product, an acknowledgment in the product documentation would be appreciated but is not required.

  2. Altered source versions must be plainly marked as such, and must not be misrepresented as being the original software.

  3. This notice may not be removed or altered from any source distribution.
*/

extern crate bindgen;
extern crate curl;
extern crate flate2;
extern crate pkg_config;
extern crate url;
extern crate zip;

use std::env;
use std::fs;
use std::fs::File;
use std::io;
use std::io::BufWriter;
use std::io::Write;
use std::path::PathBuf;

use curl::easy::Easy;
use flate2::read::GzDecoder;
use tar::Archive;
use url::Url;

fn main() {
    if cfg!(target_os = "windows") {
        println!("cargo:rustc-link-lib=dylib=user32");
        println!("cargo:rustc-link-lib=dylib=gdi32");
    }
    if cfg!(target_os = "linux") {
        println!("cargo:rustc-link-lib=X11");
    }
    if cfg!(target_os = "macos") {
        println!("cargo:rustc-link-lib=framework=OpenGL");
        println!("cargo:rustc-link-lib=framework=Cocoa");
        println!("cargo:rustc-link-lib=framework=IOKit");
        println!("cargo:rustc-link-lib=framework=CoreFoundation");
        println!("cargo:rustc-link-lib=framework=CoreVideo");
    }

    if pkg_config::Config::new()
        .atleast_version("2.0.0")
        .probe("raylib")
        .is_ok()
    {
        println!("cargo:rustc-link-lib=static=raylib");
    } else {
        let binary_url = if cfg!(all(target_os = "linux", target_arch = "x86_64")) {
            "https://github.com/raysan5/raylib/releases/download/2.0.0/raylib-2.0.0-Linux-amd64.tar.gz"
        } else if cfg!(all(target_os = "linux", target_arch = "x86")) {
            "https://github.com/raysan5/raylib/releases/download/2.0.0/raylib-2.0.0-Linux-i386.tar.gz"
        } else if cfg!(target_os = "macos") {
            "https://github.com/raysan5/raylib/releases/download/2.0.0/raylib-2.0.0-macOS.tar.gz"
        } else if cfg!(all(
            target_os = "windows",
            target_env = "gnu",
            target_arch = "x86"
        )) {
            "https://github.com/raysan5/raylib/releases/download/2.0.0/raylib-2.0.0-Win32-mingw.zip"
        } else if cfg!(all(
            target_os = "windows",
            target_env = "gnu",
            target_arch = "x86_64"
        )) {
            "https://github.com/raysan5/raylib/releases/download/2.0.0/raylib-2.0.0-Win64-mingw.zip"
        } else if cfg!(all(
            target_os = "windows",
            target_env = "msvc",
            target_arch = "x86"
        )) {
            "https://github.com/raysan5/raylib/releases/download/2.0.0/raylib-2.0.0-Win32-msvc15.zip"
        } else if cfg!(all(
            target_os = "windows",
            target_env = "msvc",
            target_arch = "x86_64"
        )) {
            "https://github.com/raysan5/raylib/releases/download/2.0.0/raylib-2.0.0-Win64-msvc15.zip"
        } else {
            panic!("unknown target_os")
        };

        let binary_url = Url::parse(binary_url).unwrap();

        let download_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("target");

        if !download_dir.exists() {
            fs::create_dir(&download_dir).unwrap();
        }

        let file_name = binary_url.path_segments().unwrap().last().unwrap();
        let download_file = download_dir.join(file_name);

        // Download the tarball.
        if !download_file.exists() {
            let f = File::create(&download_file).unwrap();
            let mut writer = BufWriter::new(f);
            let mut easy = Easy::new();
            easy.url(binary_url.as_str()).unwrap();
            easy.follow_location(true).unwrap();
            easy.write_function(move |data| Ok(writer.write(data).unwrap()))
                .unwrap();
            easy.perform().unwrap();

            let response_code = easy.response_code().unwrap();
            if response_code != 200 {
                panic!(
                    "Unexpected response code {} for {}",
                    response_code, binary_url
                );
            }
        }

        // extract and link library
        let extract_dir = if cfg!(unix) {
            let extract_dir = PathBuf::from((&download_dir).join(file_name.replace(".tar.gz", "")));
            if !extract_dir.exists() {
                // unzip
                let file = File::open(download_file).unwrap();
                let unzipped = GzDecoder::new(file);
                let mut archive = Archive::new(unzipped);
                archive.unpack(download_dir).unwrap();
            }

            extract_dir
        } else if cfg!(windows) {
            // unpack zip file
            let extract_dir = PathBuf::from((&download_dir).join(file_name.replace(".zip", "")));

            if !extract_dir.exists() {
                let file = fs::File::open(&download_file).unwrap();
                let mut archive = zip::ZipArchive::new(file).unwrap();
                for i in 0..archive.len() {
                    let mut file = archive.by_index(i).unwrap();
                    let outpath = download_dir.join(file.sanitized_name());

                    if (&*file.name()).ends_with('/') {
                        println!(
                            "File {} extracted to \"{}\"",
                            i,
                            outpath.as_path().display()
                        );
                        fs::create_dir_all(&outpath).unwrap();
                    } else {
                        println!(
                            "File {} extracted to \"{}\" ({} bytes)",
                            i,
                            outpath.as_path().display(),
                            file.size()
                        );
                        if let Some(p) = outpath.parent() {
                            if !p.exists() {
                                fs::create_dir_all(&p).unwrap();
                            }
                        }
                        let mut outfile = fs::File::create(&outpath).unwrap();
                        io::copy(&mut file, &mut outfile).unwrap();
                    }
                }
            }

            extract_dir
        } else {
            unreachable!("should have failed trying to download")
        };

        println!(
            "cargo:rustc-link-search=native={}",
            extract_dir.join("lib").display()
        );

        println!("cargo:rustc-link-lib=static=raylib");
    }
}
