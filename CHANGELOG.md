# raylib-rs Changelog

## 0.10.0 (WIP)

- Basic macOS support. Currently untested.
- Improved ergonomics across the board:
  - Copied over and tweaked many FFI structs so that fields use proper types instead of FFI types.
  - Added `vec2`, `vec3`, `quat`, `rgb`, and `rgba` convenience functions for a middle ground between `From` conversion and `new` methods.
  - Changed several key and gamepad functions to use `u32`, making it more ergonomic with key/gamepad constants.
  - Added optional `prelude` module for conveniently bringing in all the common types and definitions.
- Fixed unnecessary `&mut` in `load_image_ex` and `draw_poly_ex`.
- Fixed linking on MSVC toolchains by including `user32`.
- Prevent `RaylibHandle` from being manually constructed. Fixes a safety soundness hole.

## 0.9.1

- Fixed docs.rs build by removing use of a uniform module path. This also keeps the crate compatible with Rust 1.31+.

## 0.9.0

- Initial crates.io release
