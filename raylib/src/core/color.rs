//! [`Color`] manipulation helpers
use crate::core::math::{Vector3, Vector4};
use crate::ffi;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
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

    pub const LIGHTGRAY: Color = Color {
        r: 200,
        g: 200,
        b: 200,
        a: 255,
    };
    pub const GRAY: Color = Color {
        r: 130,
        g: 130,
        b: 130,
        a: 255,
    };
    pub const DARKGRAY: Color = Color {
        r: 80,
        g: 80,
        b: 80,
        a: 255,
    };
    pub const YELLOW: Color = Color {
        r: 253,
        g: 249,
        b: 0,
        a: 255,
    };
    pub const GOLD: Color = Color {
        r: 255,
        g: 203,
        b: 0,
        a: 255,
    };
    pub const ORANGE: Color = Color {
        r: 255,
        g: 161,
        b: 0,
        a: 255,
    };
    pub const PINK: Color = Color {
        r: 255,
        g: 109,
        b: 194,
        a: 255,
    };
    pub const RED: Color = Color {
        r: 230,
        g: 41,
        b: 55,
        a: 255,
    };
    pub const MAROON: Color = Color {
        r: 190,
        g: 33,
        b: 55,
        a: 255,
    };
    pub const GREEN: Color = Color {
        r: 0,
        g: 228,
        b: 48,
        a: 255,
    };
    pub const LIME: Color = Color {
        r: 0,
        g: 158,
        b: 47,
        a: 255,
    };
    pub const DARKGREEN: Color = Color {
        r: 0,
        g: 117,
        b: 44,
        a: 255,
    };
    pub const SKYBLUE: Color = Color {
        r: 102,
        g: 191,
        b: 255,
        a: 255,
    };
    pub const BLUE: Color = Color {
        r: 0,
        g: 121,
        b: 241,
        a: 255,
    };
    pub const DARKBLUE: Color = Color {
        r: 0,
        g: 82,
        b: 172,
        a: 255,
    };
    pub const PURPLE: Color = Color {
        r: 200,
        g: 122,
        b: 255,
        a: 255,
    };
    pub const VIOLET: Color = Color {
        r: 135,
        g: 60,
        b: 190,
        a: 255,
    };
    pub const DARKPURPLE: Color = Color {
        r: 112,
        g: 31,
        b: 126,
        a: 255,
    };
    pub const BEIGE: Color = Color {
        r: 211,
        g: 176,
        b: 131,
        a: 255,
    };
    pub const BROWN: Color = Color {
        r: 127,
        g: 106,
        b: 79,
        a: 255,
    };
    pub const DARKBROWN: Color = Color {
        r: 76,
        g: 63,
        b: 47,
        a: 255,
    };
    pub const WHITE: Color = Color {
        r: 255,
        g: 255,
        b: 255,
        a: 255,
    };
    pub const BLACK: Color = Color {
        r: 0,
        g: 0,
        b: 0,
        a: 255,
    };
    pub const BLANK: Color = Color {
        r: 0,
        g: 0,
        b: 0,
        a: 0,
    };
    pub const MAGENTA: Color = Color {
        r: 255,
        g: 0,
        b: 255,
        a: 255,
    };
    pub const RAYWHITE: Color = Color {
        r: 245,
        g: 245,
        b: 245,
        a: 255,
    };

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
    pub fn get_color(hex_value: i32) -> Color {
        unsafe { ffi::GetColor(hex_value).into() }
    }

    /// Color fade-in or fade-out, alpha goes from 0.0f to 1.0f
    #[inline]
    pub fn fade(&self, alpha: f32) -> Color {
        unsafe { ffi::Fade(self.into(), alpha).into() }
    }

    /// Color fade-in or fade-out, alpha goes from 0.0f to 1.0f
    #[inline]
    pub fn color_alpha_blend(dst: &Color, src: &Color, tint: &Color) -> Color {
        unsafe { ffi::ColorAlphaBlend(dst.into(), src.into(), tint.into()).into() }
    }
}
