# raylib-rs Changelog

## 0.10.0 (WIP)

- Basic macOS support. Currently untested.
- Changed several key and gamepad functions to use `u32`, making it more ergonomic with key/gamepad constants.
- Fixed unnecessary `&mut` in `load_image_ex` and `draw_poly_ex`.

## 0.9.1

- Fixed docs.rs build by removing use of a uniform module path. This also keeps the crate compatible with Rust 1.31+.

## 0.9.0

- Initial crates.io release
