//! [`Color`] manipulation helpers
use std::os::raw::c_void;

use crate::core::math::{Vector3, Vector4};
use crate::ffi;

use raylib_sys::{ColorIsEqual, GetPixelColor, PixelFormat};
#[cfg(not(feature = "with_serde"))]
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(not(feature = "serde"))]
#[cfg(feature = "with_serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "with_serde")]
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use super::RaylibHandle;

#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

// A convenience function for making a new `Color`.
pub fn rcolor(r: u8, g: u8, b: u8, a: u8) -> Color {
    Color::new(r, g, b, a)
}

impl From<ffi::Color> for Color {
    fn from(v: ffi::Color) -> Color {
        unsafe { std::mem::transmute(v) }
    }
}

impl Into<ffi::Color> for Color {
    fn into(self) -> ffi::Color {
        unsafe { std::mem::transmute(self) }
    }
}

impl Into<ffi::Color> for &Color {
    fn into(self) -> ffi::Color {
        ffi::Color {
            r: self.r,
            g: self.g,
            b: self.b,
            a: self.a,
        }
    }
}

impl Into<Vector4> for Color {
    fn into(self) -> Vector4 {
        Vector4::new(
            self.r as f32 / 255.0,
            self.g as f32 / 255.0,
            self.b as f32 / 255.0,
            self.a as f32 / 255.0,
        )
    }
}

impl From<(u8, u8, u8, u8)> for Color {
    fn from(col: (u8, u8, u8, u8)) -> Color {
        Color::new(col.0, col.1, col.2, col.3)
    }
}

impl Color {
    /// Get color from HEX RGB string
    /// # Arguments
    /// * `color_hex_str` - A string slice, 6 characters long
    /// # Example
    /// ```
    ///    use raylib::prelude::*;
    ///     let color_white = Color::from_hex("FFFFFF").unwrap();
    ///     let color_black = Color::from_hex("000000").unwrap();
    ///     
    ///    assert_eq!(color_black, Color::BLACK);
    ///    assert_eq!(color_white, Color::WHITE);
    /// ```
    pub fn from_hex(color_hex_str: &str) -> Result<Color, std::num::ParseIntError> {
        let color = i32::from_str_radix(color_hex_str, 16)?;
        let b = color % 0x100;
        let g = (color - b) / 0x100 % 0x100;
        let r = (color - g) / 0x10000;

        Ok(Color {
            r: r as u8,
            g: g as u8,
            b: b as u8,
            a: 255,
        })
    }

    #[inline]
    pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Color {
        Color { r, g, b, a }
    }

    /// Returns hexadecimal value for a Color
    #[inline]
    pub fn color_to_int(&self) -> i32 {
        unsafe { ffi::ColorToInt(self.into()) }
    }

    /// Returns color normalized as float [0..1]
    #[inline]
    pub fn color_normalize(&self) -> Vector4 {
        unsafe { ffi::ColorNormalize(self.into()).into() }
    }

    /// Returns HSV values for a Color
    #[inline]
    pub fn color_to_hsv(&self) -> Vector3 {
        unsafe { ffi::ColorToHSV(self.into()).into() }
    }

    /// Returns a Color from HSV values
    #[inline]
    pub fn color_from_hsv(hue: f32, saturation: f32, value: f32) -> Color {
        unsafe { ffi::ColorFromHSV(hue, saturation, value).into() }
    }

    /// Returns color from normalized values [0..1]
    /// ```rust
    /// use raylib::prelude::*;
    /// fn main() {    
    ///     assert_eq!(Color::color_from_normalized(Vector4::new(1.0, 1.0, 1.0, 1.0)), Color::new(255, 255, 255, 255));
    /// }
    /// ```
    #[inline]
    pub fn color_from_normalized(normalized: Vector4) -> Color {
        unsafe { ffi::ColorFromNormalized(normalized.into()).into() }
    }

    /// Returns a Color struct from hexadecimal value
    #[inline]
    pub fn get_color(hex_value: u32) -> Color {
        unsafe { ffi::GetColor(hex_value).into() }
    }

    /// Get color multiplied with another color
    pub fn tint(&self, color: Self) -> Self {
        unsafe { ffi::ColorTint(self.into(), color.into()).into() }
    }
    /// Get color with brightness correction, brightness factor goes from -1.0f to 1.0f
    pub fn brightness(&self, factor: f32) -> Self {
        unsafe { ffi::ColorBrightness(self.into(), factor).into() }
    }
    /// Get color with contrast correction, contrast values between -1.0f and 1.0f
    pub fn contrast(&self, factor: f32) -> Self {
        unsafe { ffi::ColorContrast(self.into(), factor).into() }
    }
    /// Get color with alpha applied, alpha goes from 0.0f to 1.0f
    pub fn alpha(&self, alpha: f32) -> Self {
        unsafe { ffi::ColorAlpha(self.into(), alpha).into() }
    }

    /// Get color with alpha applied, alpha goes from 0.0f to 1.0f
    #[deprecated = "Use Color::alpha instead"]
    pub fn fade(&self, alpha: f32) -> Self {
        unsafe { ffi::Fade(self.into(), alpha).into() }
    }

    /// Color fade-in or fade-out, alpha goes from 0.0f to 1.0f
    #[inline]
    pub fn color_alpha_blend(dst: &Color, src: &Color, tint: &Color) -> Color {
        unsafe { ffi::ColorAlphaBlend(dst.into(), src.into(), tint.into()).into() }
    }
    /// Check if color is equal to another.
    pub fn is_equal(&self, rhs: impl Into<ffi::Color>) -> bool {
        unsafe { ffi::ColorIsEqual(self.into(), rhs.into()) }
    }

    /// Get color lerp interpolation between two colors, factor [0.0f..1.0f]
    pub fn lerp(&self, rhs: Color, factor: f32) -> Color {
        unsafe { ffi::ColorLerp(self.into(), rhs.into(), factor).into() }
    }
}

/// NOTE(IOI_XD): We manually implement PartialEq as of 5.5 to use Raylib's function. It's very unlikely it will ever
/// change or do anything different, but in the ultra rare case that it does, we want to mimick Raylib's behavior.
impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        return self.is_equal(other);
    }
}
impl Eq for Color {}

/// Color constants
impl Color {
    pub const INDIANRED: Color = Color::new(205, 92, 92, 255);
    pub const LIGHTCORAL: Color = Color::new(240, 128, 128, 255);
    pub const SALMON: Color = Color::new(250, 128, 114, 255);
    pub const DARKSALMON: Color = Color::new(233, 150, 122, 255);
    pub const LIGHTSALMON: Color = Color::new(255, 160, 122, 255);
    pub const CRIMSON: Color = Color::new(220, 20, 60, 255);
    pub const RED: Color = Color::new(255, 0, 0, 255);
    pub const FIREBRICK: Color = Color::new(178, 34, 34, 255);
    pub const DARKRED: Color = Color::new(139, 0, 0, 255);
    pub const PINK: Color = Color::new(255, 192, 203, 255);
    pub const LIGHTPINK: Color = Color::new(255, 182, 193, 255);
    pub const HOTPINK: Color = Color::new(255, 105, 180, 255);
    pub const DEEPPINK: Color = Color::new(255, 20, 147, 255);
    pub const MEDIUMVIOLETRED: Color = Color::new(199, 21, 133, 255);
    pub const PALEVIOLETRED: Color = Color::new(219, 112, 147, 255);
    pub const CORAL: Color = Color::new(255, 127, 80, 255);
    pub const TOMATO: Color = Color::new(255, 99, 71, 255);
    pub const ORANGERED: Color = Color::new(255, 69, 0, 255);
    pub const DARKORANGE: Color = Color::new(255, 140, 0, 255);
    pub const ORANGE: Color = Color::new(255, 165, 0, 255);
    pub const GOLD: Color = Color::new(255, 215, 0, 255);
    pub const YELLOW: Color = Color::new(255, 255, 0, 255);
    pub const LIGHTYELLOW: Color = Color::new(255, 255, 224, 255);
    pub const LEMONCHIFFON: Color = Color::new(255, 250, 205, 255);
    pub const LIGHTGOLDENRODYELLOW: Color = Color::new(250, 250, 210, 255);
    pub const PAPAYAWHIP: Color = Color::new(255, 239, 213, 255);
    pub const MOCCASIN: Color = Color::new(255, 228, 181, 255);
    pub const PEACHPUFF: Color = Color::new(255, 218, 185, 255);
    pub const PALEGOLDENROD: Color = Color::new(238, 232, 170, 255);
    pub const KHAKI: Color = Color::new(240, 230, 140, 255);
    pub const DARKKHAKI: Color = Color::new(189, 183, 107, 255);
    pub const LAVENDER: Color = Color::new(230, 230, 250, 255);
    pub const THISTLE: Color = Color::new(216, 191, 216, 255);
    pub const PLUM: Color = Color::new(221, 160, 221, 255);
    pub const VIOLET: Color = Color::new(238, 130, 238, 255);
    pub const ORCHID: Color = Color::new(218, 112, 214, 255);
    pub const FUCHSIA: Color = Color::new(255, 0, 255, 255);
    pub const MAGENTA: Color = Color::new(255, 0, 255, 255);
    pub const MEDIUMORCHID: Color = Color::new(186, 85, 211, 255);
    pub const MEDIUMPURPLE: Color = Color::new(147, 112, 219, 255);
    pub const REBECCAPURPLE: Color = Color::new(102, 51, 153, 255);
    pub const BLUEVIOLET: Color = Color::new(138, 43, 226, 255);
    pub const DARKVIOLET: Color = Color::new(148, 0, 211, 255);
    pub const DARKORCHID: Color = Color::new(153, 50, 204, 255);
    pub const DARKMAGENTA: Color = Color::new(139, 0, 139, 255);
    pub const PURPLE: Color = Color::new(128, 0, 128, 255);
    pub const DARKPURPLE: Color = Color::new(112, 31, 126, 255);
    pub const INDIGO: Color = Color::new(75, 0, 130, 255);
    pub const SLATEBLUE: Color = Color::new(106, 90, 205, 255);
    pub const DARKSLATEBLUE: Color = Color::new(72, 61, 139, 255);
    pub const MEDIUMSLATEBLUE: Color = Color::new(123, 104, 238, 255);
    pub const GREENYELLOW: Color = Color::new(173, 255, 47, 255);
    pub const CHARTREUSE: Color = Color::new(127, 255, 0, 255);
    pub const LAWNGREEN: Color = Color::new(124, 252, 0, 255);
    pub const LIME: Color = Color::new(0, 255, 0, 255);
    pub const LIMEGREEN: Color = Color::new(50, 205, 50, 255);
    pub const PALEGREEN: Color = Color::new(152, 251, 152, 255);
    pub const LIGHTGREEN: Color = Color::new(144, 238, 144, 255);
    pub const MEDIUMSPRINGGREEN: Color = Color::new(0, 250, 154, 255);
    pub const SPRINGGREEN: Color = Color::new(0, 255, 127, 255);
    pub const MEDIUMSEAGREEN: Color = Color::new(60, 179, 113, 255);
    pub const SEAGREEN: Color = Color::new(46, 139, 87, 255);
    pub const FORESTGREEN: Color = Color::new(34, 139, 34, 255);
    pub const GREEN: Color = Color::new(0, 128, 0, 255);
    pub const DARKGREEN: Color = Color::new(0, 100, 0, 255);
    pub const YELLOWGREEN: Color = Color::new(154, 205, 50, 255);
    pub const OLIVEDRAB: Color = Color::new(107, 142, 35, 255);
    pub const OLIVE: Color = Color::new(128, 128, 0, 255);
    pub const DARKOLIVEGREEN: Color = Color::new(85, 107, 47, 255);
    pub const MEDIUMAQUAMARINE: Color = Color::new(102, 205, 170, 255);
    pub const DARKSEAGREEN: Color = Color::new(143, 188, 139, 255);
    pub const LIGHTSEAGREEN: Color = Color::new(32, 178, 170, 255);
    pub const DARKCYAN: Color = Color::new(0, 139, 139, 255);
    pub const TEAL: Color = Color::new(0, 128, 128, 255);
    pub const AQUA: Color = Color::new(0, 255, 255, 255);
    pub const CYAN: Color = Color::new(0, 255, 255, 255);
    pub const LIGHTCYAN: Color = Color::new(224, 255, 255, 255);
    pub const PALETURQUOISE: Color = Color::new(175, 238, 238, 255);
    pub const AQUAMARINE: Color = Color::new(127, 255, 212, 255);
    pub const TURQUOISE: Color = Color::new(64, 224, 208, 255);
    pub const MEDIUMTURQUOISE: Color = Color::new(72, 209, 204, 255);
    pub const DARKTURQUOISE: Color = Color::new(0, 206, 209, 255);
    pub const CADETBLUE: Color = Color::new(95, 158, 160, 255);
    pub const STEELBLUE: Color = Color::new(70, 130, 180, 255);
    pub const LIGHTSTEELBLUE: Color = Color::new(176, 196, 222, 255);
    pub const POWDERBLUE: Color = Color::new(176, 224, 230, 255);
    pub const LIGHTBLUE: Color = Color::new(173, 216, 230, 255);
    pub const SKYBLUE: Color = Color::new(135, 206, 235, 255);
    pub const LIGHTSKYBLUE: Color = Color::new(135, 206, 250, 255);
    pub const DEEPSKYBLUE: Color = Color::new(0, 191, 255, 255);
    pub const DODGERBLUE: Color = Color::new(30, 144, 255, 255);
    pub const CORNFLOWERBLUE: Color = Color::new(100, 149, 237, 255);
    pub const ROYALBLUE: Color = Color::new(65, 105, 225, 255);
    pub const BLUE: Color = Color::new(0, 0, 255, 255);
    pub const MEDIUMBLUE: Color = Color::new(0, 0, 205, 255);
    pub const DARKBLUE: Color = Color::new(0, 0, 139, 255);
    pub const NAVY: Color = Color::new(0, 0, 128, 255);
    pub const MIDNIGHTBLUE: Color = Color::new(25, 25, 112, 255);
    pub const CORNSILK: Color = Color::new(255, 248, 220, 255);
    pub const BLANCHEDALMOND: Color = Color::new(255, 235, 205, 255);
    pub const BISQUE: Color = Color::new(255, 228, 196, 255);
    pub const NAVAJOWHITE: Color = Color::new(255, 222, 173, 255);
    pub const WHEAT: Color = Color::new(245, 222, 179, 255);
    pub const BURLYWOOD: Color = Color::new(222, 184, 135, 255);
    pub const TAN: Color = Color::new(210, 180, 140, 255);
    pub const ROSYBROWN: Color = Color::new(188, 143, 143, 255);
    pub const SANDYBROWN: Color = Color::new(244, 164, 96, 255);
    pub const GOLDENROD: Color = Color::new(218, 165, 32, 255);
    pub const DARKGOLDENROD: Color = Color::new(184, 134, 11, 255);
    pub const PERU: Color = Color::new(205, 133, 63, 255);
    pub const CHOCOLATE: Color = Color::new(210, 105, 30, 255);
    pub const SADDLEBROWN: Color = Color::new(139, 69, 19, 255);
    pub const SIENNA: Color = Color::new(160, 82, 45, 255);
    pub const BROWN: Color = Color::new(165, 42, 42, 255);
    pub const DARKBROWN: Color = Color::new(76, 63, 47, 255);
    pub const MAROON: Color = Color::new(128, 0, 0, 255);
    pub const WHITE: Color = Color::new(255, 255, 255, 255);
    pub const SNOW: Color = Color::new(255, 250, 250, 255);
    pub const HONEYDEW: Color = Color::new(240, 255, 240, 255);
    pub const MINTCREAM: Color = Color::new(245, 255, 250, 255);
    pub const AZURE: Color = Color::new(240, 255, 255, 255);
    pub const ALICEBLUE: Color = Color::new(240, 248, 255, 255);
    pub const GHOSTWHITE: Color = Color::new(248, 248, 255, 255);
    pub const WHITESMOKE: Color = Color::new(245, 245, 245, 255);
    pub const SEASHELL: Color = Color::new(255, 245, 238, 255);
    pub const BEIGE: Color = Color::new(245, 245, 220, 255);
    pub const OLDLACE: Color = Color::new(253, 245, 230, 255);
    pub const FLORALWHITE: Color = Color::new(255, 250, 240, 255);
    pub const IVORY: Color = Color::new(255, 255, 240, 255);
    pub const ANTIQUEWHITE: Color = Color::new(250, 235, 215, 255);
    pub const LINEN: Color = Color::new(250, 240, 230, 255);
    pub const LAVENDERBLUSH: Color = Color::new(255, 240, 245, 255);
    pub const MISTYROSE: Color = Color::new(255, 228, 225, 255);
    pub const GAINSBORO: Color = Color::new(220, 220, 220, 255);
    pub const LIGHTGRAY: Color = Color::new(211, 211, 211, 255);
    pub const SILVER: Color = Color::new(192, 192, 192, 255);
    pub const DARKGRAY: Color = Color::new(169, 169, 169, 255);
    pub const GRAY: Color = Color::new(128, 128, 128, 255);
    pub const DIMGRAY: Color = Color::new(105, 105, 105, 255);
    pub const LIGHTSLATEGRAY: Color = Color::new(119, 136, 153, 255);
    pub const SLATEGRAY: Color = Color::new(112, 128, 144, 255);
    pub const DARKSLATEGRAY: Color = Color::new(47, 79, 79, 255);
    pub const BLACK: Color = Color::new(0, 0, 0, 255);
    pub const BLANK: Color = Color::new(0, 0, 0, 0);
    pub const RAYWHITE: Color = Color::new(245, 245, 245, 255);
}
