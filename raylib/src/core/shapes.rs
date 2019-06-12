use crate::ffi;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Rectangle {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl From<ffi::Rectangle> for Rectangle {
    fn from(v: ffi::Rectangle) -> Rectangle {
        unsafe { std::mem::transmute(v) }
    }
}

impl Into<ffi::Rectangle> for Rectangle {
    fn into(self) -> ffi::Rectangle {
        unsafe { std::mem::transmute(self) }
    }
}

impl Into<ffi::Rectangle> for &Rectangle {
    fn into(self) -> ffi::Rectangle {
        ffi::Rectangle {
            x: self.x,
            y: self.y,
            width: self.width,
            height: self.height,
        }
    }
}