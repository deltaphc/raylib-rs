<p align="center">
    
![rust](https://img.shields.io/badge/rust-1.77+-orange.svg?style=flat-square&logo=rust)
[![crates.io](https://img.shields.io/crates/v/raylib.svg?style=flat-square)](https://crates.io/crates/raylib)
[![docs](https://docs.rs/raylib/badge.svg)](https://docs.rs/raylib)
[![discord](https://img.shields.io/discord/426912293134270465)](https://discord.gg/VkzNHUE)

</p>

<table border="0">
<tr>
<td>

![logo](logo/raylib-rust_256x256.png)

</td>
<td>

# raylib-rs

raylib-rs is a Rust binding for [raylib](http://www.raylib.com/) 5.0. It currently targets the _stable_ Rust toolchain, version 1.78 or higher.

Please checkout the showcase directory to find usage examples!

Though this binding tries to stay close to the simple C API, it makes some changes to be more idiomatic for Rust.

</td>
</tr>
</table>


- Resources are automatically cleaned up when they go out of scope (or when `std::mem::drop` is called). This is essentially RAII. This means that "Unload" functions are not exposed (and not necessary unless you obtain a `Weak` resource using make_weak()).
- Most of the Raylib API is exposed through `RaylibHandle`, which is for enforcing that Raylib is only initialized once, and for making sure the window is closed properly. RaylibHandle has no size and goes away at compile time. Because of mutability rules, Raylib-rs is thread safe!
- A `RaylibHandle` and `RaylibThread` are obtained through `raylib::init_window(...)` or through the newer `init()` function which will allow you to `build` up some window options before initialization (replaces `set_config_flags`). RaylibThread should not be sent to any other threads, or used in a any syncronization primitives (Mutex, Arc) etc.
- Manually closing the window is unnecessary, because `CloseWindow` is automatically called when `RaylibHandle` goes out of scope.
- `Model::set_material`, `Material::set_shader`, and `MaterialMap::set_texture` methods were added since one cannot set the fields directly. Also enforces correct ownership semantics.
- `Font::from_data`, `Font::set_chars`, and `Font::set_texture` methods were added to create a `Font` from loaded `CharInfo` data.
- `SubText` and `FormatText` are omitted, and are instead covered by Rust's string slicing and Rust's `format!` macro, respectively.

# Installation

## Supported Platforms

| API    | Windows            | Linux              | macOS              | Web                | Android | 
| ------ | ------------------ | ------------------ | ------------------ | --------------     | ------- |
| core   | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: | :x:     |
| rgui   | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: | ❔                 | :x:     |
| physac | :construction:     | :construction:     | :construction:     | ❔                 | :x:     |
| rlgl   | :heavy_check_mark: | :x:                | :x:                | ❔                 | :x:     |

## Build Dependencies

Requires glfw, cmake, and curl. Tips on making things work smoothly on all platforms is appreciated.
Follow instructions for building raylib for your platform [here](https://github.com/raysan5/raylib/wiki)

1. Add the dependency to your `Cargo.toml`:

```toml
[dependencies]
raylib = { version = "5.0" }
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

- Structs holding resources have RAII/move semantics, including: `Image`, `Texture2D`, `RenderTexture2D`, `Font`, `Mesh`, `Shader`, `Material`, and `Model`.
- `Wave`, `Sound`, `Music`, and `AudioStream` have lifetimes bound to `AudioHandle`. 
- Functions dealing with string data take in `&str` and/or return an owned `String`, for the sake of safety. The exception to this is the gui draw functions which take &CStr to avoid per frame allocations. The `rstr!` macro helps make this easy.
- In C, `LoadFontData` returns a pointer to a heap-allocated array of `CharInfo` structs. In this Rust binding, said array is copied into an owned `Vec<CharInfo>`, the original data is freed, and the owned Vec is returned.
- In C, `LoadDroppedFiles` returns a pointer to an array of strings owned by raylib. Again, for safety and also ease of use, this binding copies said array into a `Vec<String>` which is returned to the caller.
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
