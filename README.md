![logo](logo/raylib-rust_256x256.png)

![rust](https://img.shields.io/badge/rust-1.31+-orange.svg?style=flat-square&logo=rust)
[![crates.io](https://img.shields.io/crates/v/raylib.svg?style=flat-square)](https://crates.io/crates/raylib)
[![docs](https://docs.rs/raylib/badge.svg)](https://docs.rs/raylib)
[discord](https://discord.gg/VkzNHUE)

# NOTE:
In order to build, you will need `ninja` in your PATH.
### WINDOWS:
On Windows it should come installed with Visual Studio. The following is where ninja is located in visual studio 2022, ![add it to your path](https://youtu.be/z84UIZy_qgE?si=j56ya4UfYkbQDgHe)
```
C:\Program Files\Microsoft Visual Studio\2022\Community\Common7\IDE\CommonExtensions\Microsoft\CMake\Ninja
```
### LINUX(ubuntu/debian):
Install it via your distros package manager and make sure it's in your path:
```
sudo apt-get install ninja-build
whereis ninja
```

# NOTE 4.x version in progress. 

Raylib 4.2 is currenlty a work in progress. In the meantime you can use a fork: https://github.com/litten2up/raylib-rs

Add this to your Cargo.toml
```toml
[dependencies.raylib]
version = "4.5.0"
git = "https://github.com/litten2up/raylib-rs"
branch = "4.5.0"
```

# raylib-rs

raylib-rs is a Rust binding for [raylib](http://www.raylib.com/) 3.5. It currently targets the _stable_ Rust toolchain, version 1.31 or higher.

Please checkout the showcase directory to find usage examples!

Though this binding tries to stay close to the simple C API, it makes some changes to be more idiomatic for Rust.

- Resources are automatically cleaned up when they go out of scope (or when `std::mem::drop` is called). This is essentially RAII. This means that "Unload" functions are not exposed (and not necessary unless you obtain a `Weak` resource using make_weak()).
- Most of the Raylib API is exposed through `RaylibHandle`, which is for enforcing that Raylib is only initialized once, and for making sure the window is closed properly. RaylibHandle has no size and goes away at compile time. Because of mutability rules, Raylib-rs is thread safe!
- A `RaylibHandle` and `RaylibThread` are obtained through `raylib::init_window(...)` or through the newer `init()` function which will allow you to `build` up some window options before initialization (replaces `set_config_flags`). RaylibThread should not be sent to any other threads, or used in a any syncronization primitives (Mutex, Arc) etc.
- Manually closing the window is unnecessary, because `CloseWindow` is automatically called when `RaylibHandle` goes out of scope.
- `Model::set_material`, `Material::set_shader`, and `MaterialMap::set_texture` methods were added since one cannot set the fields directly. Also enforces correct ownership semantics.
- `Font::from_data`, `Font::set_chars`, and `Font::set_texture` methods were added to create a `Font` from loaded `CharInfo` data.
- `SubText` and `FormatText` are omitted, and are instead covered by Rust's string slicing and Rust's `format!` macro, respectively.

**Disclaimer: I created this binding as a way to learn Rust. There may be some things I can do better, or make more ergonomic for users. Feel free to make suggestions!**

# Installation

## Supported Platforms

| API    | Windows            | Linux              | macOS              | Web            | Android | Raspberry Pi |
| ------ | ------------------ | ------------------ | ------------------ | -------------- | ------- | ------------ |
| core   | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: | :construction: |         |              |
| rgui   | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: |                |         |              |
| physac | :construction:     | :construction:     | :construction:     |                |         |              |
| rlgl   | :heavy_check_mark: | :x:                | :x:                |                |         |              |

## Build Dependencies

Requires glfw, cmake, and curl. Tips on making things work smoothly on all platforms is appreciated.
Follow instructions for building raylib for your platform [here](https://github.com/raysan5/raylib/wiki)

1. Add the dependency to your `Cargo.toml`:

```toml
[dependencies]
raylib = { version = "3.7" }
```

2. Start coding!

```rust
use raylib::prelude::*;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(640, 480)
        .title("Hello, World")
        .build();

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::WHITE);
        d.draw_text("Hello, world!", 12, 12, 20, Color::BLACK);
    }
}
```

# Tech Notes

- Structs holding resources have RAII/move semantics, including: `Image`, `Texture2D`, `RenderTexture2D`, `Font`, `Mesh`, `Shader`, `Material`, `Model`, `Wave`, `Sound`, `Music`, and `AudioStream`.
- Functions dealing with string data take in `&str` and/or return an owned `String`, for the sake of safety. The exception to this is the gui draw functions which take &CStr to avoid per frame allocations. The `rstr!` macro helps make this easy.
- In C, `LoadFontData` returns a pointer to a heap-allocated array of `CharInfo` structs. In this Rust binding, said array is copied into an owned `Vec<CharInfo>`, the original data is freed, and the owned Vec is returned.
- In C, `GetDroppedFiles` returns a pointer to an array of strings owned by raylib. Again, for safety and also ease of use, this binding copies said array into a `Vec<String>` which is returned to the caller.
- I've tried to make linking automatic, though I've only tested on Windows 10, Ubuntu, and MacOS 15. Other platforms may have other considerations.
- OpenGL 3.3, 2.1, and ES 2.0 may be forced via adding `["opengl_33"]`, `["opengl_21"]` or `["opengl_es_20]` to the `features` array in your Cargo.toml dependency definition.

## Building from source

1. Clone repository: `git clone --recurse-submodules`
2. `cargo build`

### If building for Wayland on Linux

3. Install these packages:  
`libglfw3-dev wayland-devel libxkbcommon-devel wayland-protocols wayland-protocols-devel libecm-dev`
###### Note that this may not be a comprehensive list, please add details for your distribution or expand on these packages if you believe this to be incomplete.

4. Enable wayland by adding `features=["wayland"]` to your dependency definition

## Cross-compiling using `cross`

The [@rust-embedded](https://github.com/rust-embedded) project provides a handy tool called [`cross`](https://github.com/rust-embedded/cross) that uses docker to cross-compile any cargo project to one of their many [supported platforms](https://github.com/rust-embedded/cross#supported-targets). This tool makes it easy to cross-compile `raylib-rs` for binary distribution (in cases where you are producing a pre-compiled game for example).

### Anything to Windows

Cross-compiling from other platforms to Windows is the simplest. Just build your project with this command instead of the usual `cargo build`:

```sh
cross build --target x86_64-pc-windows-gnu --release
```

It should be noted that the resulting exe will likely not run under `wine` due to an issue with Raylib's audio handling.

### Anything to Linux

Cross-compiling from any platform to Linux, or from Linux to Linux requires a little extra work since `raylib-sys` has some system dependencies not provided by `cross`. This following example assumes you are compiling for `x86_64-unknown-linux-gnu`, but it can be any Linux-y triple.

Firstly, a custom build container must be defined. The following `Dockerfile` is the minimum setup for compiling `raylib-sys`:

```Dockerfile
FROM rustembedded/cross:x86_64-unknown-linux-gnu-0.2.1

RUN apt-get update -y
RUN apt-get install libasound2-dev mesa-common-dev libx11-dev libxrandr-dev libxi-dev xorg-dev libgl1-mesa-dev libglu1-mesa-dev -y
```

With the image defined, build it locally with:

```sh
docker build -t raylib_rs_env .
```

This will produce a local docker image called `raylib_rs_env` which `cross` will use instead of the default Linux image(s). To tell `cross` to use this image, create a `Cross.toml` file beside your `Cargo.toml`, and add the following (remembering to change things to suit your setup):

```toml
[target.x86_64-unknown-linux-gnu]
image = "raylib_rs_env"
```

The Linux build can now be produced with:

```sh
cross build --target x86_64-unknown-linux-gnu --release
```

# MacOS / Darwin / IOS

`cross` does not support cross-compilation to any of Apple's operating systems as of now. Keep an eye on their repository in case this ever changes.

# Extras

- In addition to the base library, there is also a convenient `ease` module which contains various interpolation/easing functions ported from raylib's `easings.h`, as well as a `Tween` struct to assist in using these functions.
- Equivalent math and vector operations, ported from `raymath.h`, are `impl`ed on the various Vector and Matrix types. Operator overloading is used for more intuitive design.

# Testing

The raylib-test crate tests the bindings by opening a window, and checking the results of various functions. It requires nightly to use.

# Future Goals

- Port raylib examples over to Rust.
- More tests.
- More platform testing.
- Even more testing.
- Physac port?

# Contribution & Support

All contributions are welcome. Chat about raylib on [discord](https://discord.gg/VkzNHUE)
