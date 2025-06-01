//! [`Color`] manipulation helpers

use crate::math::{Vector3, Vector4};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Color, 4 components, R8G8B8A8 (32bit)
#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

/// A convenience function for making a new `Color`.
#[inline]
pub const fn rcolor(r: u8, g: u8, b: u8, a: u8) -> Color {
    Color::new(r, g, b, a)
}

impl Into<Vector4> for Color {
    fn into(self) -> Vector4 {
        Vector4 {
            x: self.r as f32 / 255.0,
            y: self.g as f32 / 255.0,
            z: self.b as f32 / 255.0,
            w: self.a as f32 / 255.0,
        }
    }
}

impl Into<Color> for &Color {
    fn into(self) -> Color {
        Color {
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
    /// produces Color from a hex string(6 characters long)
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
        unsafe { super::ColorToInt(self.into()) }
    }

    /// Returns color normalized as float [0..1]
    #[inline]
    pub fn color_normalize(&self) -> Vector4 {
        unsafe { super::ColorNormalize(self.into()).into() }
    }

    /// Returns HSV values for a Color
    #[inline]
    pub fn color_to_hsv(&self) -> Vector3 {
        unsafe { super::ColorToHSV(self.into()).into() }
    }

    /// Returns a Color from HSV values
    #[inline]
    pub fn color_from_hsv(hue: f32, saturation: f32, value: f32) -> Color {
        unsafe { super::ColorFromHSV(hue, saturation, value).into() }
    }

    /// Returns color from normalized values [0..1]
    #[inline]
    pub fn color_from_normalized(normalized: Vector4) -> Color {
        unsafe { super::ColorFromNormalized(normalized.into()).into() }
    }

    /// Returns a Color struct from hexadecimal value
    #[inline]
    pub fn get_color(hex_value: u32) -> Color {
        unsafe { super::GetColor(hex_value).into() }
    }

    /// Get color multiplied with another color
    #[inline]
    pub fn tint(&self, color: Self) -> Self {
        unsafe { super::ColorTint(self.into(), color.into()).into() }
    }
    /// Get color with brightness correction, brightness factor goes from -1.0f to 1.0f
    #[inline]
    pub fn brightness(&self, factor: f32) -> Self {
        unsafe { super::ColorBrightness(self.into(), factor).into() }
    }
    /// Get color with contrast correction, contrast values between -1.0f and 1.0f
    #[inline]
    pub fn contrast(&self, factor: f32) -> Self {
        unsafe { super::ColorContrast(self.into(), factor).into() }
    }
    /// Get color with alpha applied, alpha goes from 0.0f to 1.0f
    #[inline]
    pub fn alpha(&self, alpha: f32) -> Self {
        unsafe { super::ColorAlpha(self.into(), alpha).into() }
    }

    /// Get color with alpha applied, alpha goes from 0.0f to 1.0f
    #[deprecated = "Use Color::alpha instead"]
    pub fn fade(&self, alpha: f32) -> Self {
        unsafe { super::Fade(self.into(), alpha).into() }
    }

    /// Color fade-in or fade-out, alpha goes from 0.0f to 1.0f
    #[inline]
    pub fn color_alpha_blend(dst: &Color, src: &Color, tint: &Color) -> Color {
        unsafe { super::ColorAlphaBlend(dst.into(), src.into(), tint.into()).into() }
    }
    /// Check if color is equal to another.
    #[inline]
    pub fn is_equal(&self, rhs: impl Into<super::Color>) -> bool {
        unsafe { super::ColorIsEqual(self.into(), rhs.into()) }
    }

    /// Get color lerp interpolation between two colors, factor [0.0f..1.0f]
    #[inline]
    pub fn lerp(&self, rhs: Color, factor: f32) -> Color {
        unsafe { super::ColorLerp(self.into(), rhs.into(), factor).into() }
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

#[rustfmt::skip]
/// Some Basic Colors
/// NOTE: Custom raylib color palette for amazing visuals on WHITE background
pub trait RaylibPalette {
    /** Light Gray                 */ const LIGHTGRAY:  Color = Color::new(200, 200, 200, 255);
    /** Gray                       */ const GRAY:       Color = Color::new(130, 130, 130, 255);
    /** Dark Gray                  */ const DARKGRAY:   Color = Color::new( 80,  80,  80, 255);
    /** Yellow                     */ const YELLOW:     Color = Color::new(253, 249,   0, 255);
    /** Gold                       */ const GOLD:       Color = Color::new(255, 203,   0, 255);
    /** Orange                     */ const ORANGE:     Color = Color::new(255, 161,   0, 255);
    /** Pink                       */ const PINK:       Color = Color::new(255, 109, 194, 255);
    /** Red                        */ const RED:        Color = Color::new(230,  41,  55, 255);
    /** Maroon                     */ const MAROON:     Color = Color::new(190,  33,  55, 255);
    /** Green                      */ const GREEN:      Color = Color::new(  0, 228,  48, 255);
    /** Lime                       */ const LIME:       Color = Color::new(  0, 158,  47, 255);
    /** Dark Green                 */ const DARKGREEN:  Color = Color::new(  0, 117,  44, 255);
    /** Sky Blue                   */ const SKYBLUE:    Color = Color::new(102, 191, 255, 255);
    /** Blue                       */ const BLUE:       Color = Color::new(  0, 121, 241, 255);
    /** Dark Blue                  */ const DARKBLUE:   Color = Color::new(  0,  82, 172, 255);
    /** Purple                     */ const PURPLE:     Color = Color::new(200, 122, 255, 255);
    /** Violet                     */ const VIOLET:     Color = Color::new(135,  60, 190, 255);
    /** Dark Purple                */ const DARKPURPLE: Color = Color::new(112,  31, 126, 255);
    /** Beige                      */ const BEIGE:      Color = Color::new(211, 176, 131, 255);
    /** Brown                      */ const BROWN:      Color = Color::new(127, 106,  79, 255);
    /** Dark Brown                 */ const DARKBROWN:  Color = Color::new( 76,  63,  47, 255);

    /** White                      */ const WHITE:      Color = Color::new(255, 255, 255, 255);
    /** Black                      */ const BLACK:      Color = Color::new(  0,   0,   0, 255);
    /** Blank (Transparent)        */ const BLANK:      Color = Color::new(  0,   0,   0,   0);
    /** Magenta                    */ const MAGENTA:    Color = Color::new(255,   0, 255, 255);
    /** My own White (raylib logo) */ const RAYWHITE:   Color = Color::new(245, 245, 245, 255);
}
impl RaylibPalette for Color {}

#[rustfmt::skip]
/// CSS Color constants
pub trait CSSPalette {
    /** #f0f8ffff */ const ALICEBLUE:            Color = Color::new(0xf0, 0xf8, 0xff, 0xff);
    /** #faebd7ff */ const ANTIQUEWHITE:         Color = Color::new(0xfa, 0xeb, 0xd7, 0xff);
    /** #00ffffff */ const AQUA:                 Color = Color::new(0x00, 0xff, 0xff, 0xff);
    /** #7fffd4ff */ const AQUAMARINE:           Color = Color::new(0x7f, 0xff, 0xd4, 0xff);
    /** #f0ffffff */ const AZURE:                Color = Color::new(0xf0, 0xff, 0xff, 0xff);
    /** #f5f5dcff */ const BEIGE:                Color = Color::new(0xf5, 0xf5, 0xdc, 0xff);
    /** #ffe4c4ff */ const BISQUE:               Color = Color::new(0xff, 0xe4, 0xc4, 0xff);
    /** #000000ff */ const BLACK:                Color = Color::new(0x00, 0x00, 0x00, 0xff);
    /** #ffebcdff */ const BLANCHEDALMOND:       Color = Color::new(0xff, 0xeb, 0xcd, 0xff);
    /** #0000ffff */ const BLUE:                 Color = Color::new(0x00, 0x00, 0xff, 0xff);
    /** #8a2be2ff */ const BLUEVIOLET:           Color = Color::new(0x8a, 0x2b, 0xe2, 0xff);
    /** #a52a2aff */ const BROWN:                Color = Color::new(0xa5, 0x2a, 0x2a, 0xff);
    /** #deb887ff */ const BURLYWOOD:            Color = Color::new(0xde, 0xb8, 0x87, 0xff);
    /** #5f9ea0ff */ const CADETBLUE:            Color = Color::new(0x5f, 0x9e, 0xa0, 0xff);
    /** #7fff00ff */ const CHARTREUSE:           Color = Color::new(0x7f, 0xff, 0x00, 0xff);
    /** #d2691eff */ const CHOCOLATE:            Color = Color::new(0xd2, 0x69, 0x1e, 0xff);
    /** #ff7f50ff */ const CORAL:                Color = Color::new(0xff, 0x7f, 0x50, 0xff);
    /** #6495edff */ const CORNFLOWERBLUE:       Color = Color::new(0x64, 0x95, 0xed, 0xff);
    /** #fff8dcff */ const CORNSILK:             Color = Color::new(0xff, 0xf8, 0xdc, 0xff);
    /** #dc143cff */ const CRIMSON:              Color = Color::new(0xdc, 0x14, 0x3c, 0xff);
    /** #00ffffff */ const CYAN:                 Color = Self::AQUA;
    /** #00008bff */ const DARKBLUE:             Color = Color::new(0x00, 0x00, 0x8b, 0xff);
    /** #008b8bff */ const DARKCYAN:             Color = Color::new(0x00, 0x8b, 0x8b, 0xff);
    /** #b8860bff */ const DARKGOLDENROD:        Color = Color::new(0xb8, 0x86, 0x0b, 0xff);
    /** #a9a9a9ff */ const DARKGRAY:             Color = Color::new(0xa9, 0xa9, 0xa9, 0xff);
    /** #006400ff */ const DARKGREEN:            Color = Color::new(0x00, 0x64, 0x00, 0xff);
    /** #a9a9a9ff */ const DARKGREY:             Color = Color::new(0xa9, 0xa9, 0xa9, 0xff);
    /** #bdb76bff */ const DARKKHAKI:            Color = Color::new(0xbd, 0xb7, 0x6b, 0xff);
    /** #8b008bff */ const DARKMAGENTA:          Color = Color::new(0x8b, 0x00, 0x8b, 0xff);
    /** #556b2fff */ const DARKOLIVEGREEN:       Color = Color::new(0x55, 0x6b, 0x2f, 0xff);
    /** #ff8c00ff */ const DARKORANGE:           Color = Color::new(0xff, 0x8c, 0x00, 0xff);
    /** #9932ccff */ const DARKORCHID:           Color = Color::new(0x99, 0x32, 0xcc, 0xff);
    /** #8b0000ff */ const DARKRED:              Color = Color::new(0x8b, 0x00, 0x00, 0xff);
    /** #e9967aff */ const DARKSALMON:           Color = Color::new(0xe9, 0x96, 0x7a, 0xff);
    /** #8fbc8fff */ const DARKSEAGREEN:         Color = Color::new(0x8f, 0xbc, 0x8f, 0xff);
    /** #483d8bff */ const DARKSLATEBLUE:        Color = Color::new(0x48, 0x3d, 0x8b, 0xff);
    /** #2f4f4fff */ const DARKSLATEGRAY:        Color = Color::new(0x2f, 0x4f, 0x4f, 0xff);
    /** #2f4f4fff */ const DARKSLATEGREY:        Color = Color::new(0x2f, 0x4f, 0x4f, 0xff);
    /** #00ced1ff */ const DARKTURQUOISE:        Color = Color::new(0x00, 0xce, 0xd1, 0xff);
    /** #9400d3ff */ const DARKVIOLET:           Color = Color::new(0x94, 0x00, 0xd3, 0xff);
    /** #ff1493ff */ const DEEPPINK:             Color = Color::new(0xff, 0x14, 0x93, 0xff);
    /** #00bfffff */ const DEEPSKYBLUE:          Color = Color::new(0x00, 0xbf, 0xff, 0xff);
    /** #696969ff */ const DIMGRAY:              Color = Color::new(0x69, 0x69, 0x69, 0xff);
    /** #696969ff */ const DIMGREY:              Color = Color::new(0x69, 0x69, 0x69, 0xff);
    /** #1e90ffff */ const DODGERBLUE:           Color = Color::new(0x1e, 0x90, 0xff, 0xff);
    /** #b22222ff */ const FIREBRICK:            Color = Color::new(0xb2, 0x22, 0x22, 0xff);
    /** #fffaf0ff */ const FLORALWHITE:          Color = Color::new(0xff, 0xfa, 0xf0, 0xff);
    /** #228b22ff */ const FORESTGREEN:          Color = Color::new(0x22, 0x8b, 0x22, 0xff);
    /** #ff00ffff */ const FUCHSIA:              Color = Color::new(0xff, 0x00, 0xff, 0xff);
    /** #dcdcdcff */ const GAINSBORO:            Color = Color::new(0xdc, 0xdc, 0xdc, 0xff);
    /** #f8f8ffff */ const GHOSTWHITE:           Color = Color::new(0xf8, 0xf8, 0xff, 0xff);
    /** #ffd700ff */ const GOLD:                 Color = Color::new(0xff, 0xd7, 0x00, 0xff);
    /** #daa520ff */ const GOLDENROD:            Color = Color::new(0xda, 0xa5, 0x20, 0xff);
    /** #808080ff */ const GRAY:                 Color = Color::new(0x80, 0x80, 0x80, 0xff);
    /** #008000ff */ const GREEN:                Color = Color::new(0x00, 0x80, 0x00, 0xff);
    /** #adff2fff */ const GREENYELLOW:          Color = Color::new(0xad, 0xff, 0x2f, 0xff);
    /** #808080ff */ const GREY:                 Color = Self::GRAY;
    /** #f0fff0ff */ const HONEYDEW:             Color = Color::new(0xf0, 0xff, 0xf0, 0xff);
    /** #ff69b4ff */ const HOTPINK:              Color = Color::new(0xff, 0x69, 0xb4, 0xff);
    /** #cd5c5cff */ const INDIANRED:            Color = Color::new(0xcd, 0x5c, 0x5c, 0xff);
    /** #4b0082ff */ const INDIGO:               Color = Color::new(0x4b, 0x00, 0x82, 0xff);
    /** #fffff0ff */ const IVORY:                Color = Color::new(0xff, 0xff, 0xf0, 0xff);
    /** #f0e68cff */ const KHAKI:                Color = Color::new(0xf0, 0xe6, 0x8c, 0xff);
    /** #e6e6faff */ const LAVENDER:             Color = Color::new(0xe6, 0xe6, 0xfa, 0xff);
    /** #fff0f5ff */ const LAVENDERBLUSH:        Color = Color::new(0xff, 0xf0, 0xf5, 0xff);
    /** #7cfc00ff */ const LAWNGREEN:            Color = Color::new(0x7c, 0xfc, 0x00, 0xff);
    /** #fffacdff */ const LEMONCHIFFON:         Color = Color::new(0xff, 0xfa, 0xcd, 0xff);
    /** #add8e6ff */ const LIGHTBLUE:            Color = Color::new(0xad, 0xd8, 0xe6, 0xff);
    /** #f08080ff */ const LIGHTCORAL:           Color = Color::new(0xf0, 0x80, 0x80, 0xff);
    /** #e0ffffff */ const LIGHTCYAN:            Color = Color::new(0xe0, 0xff, 0xff, 0xff);
    /** #fafad2ff */ const LIGHTGOLDENRODYELLOW: Color = Color::new(0xfa, 0xfa, 0xd2, 0xff);
    /** #d3d3d3ff */ const LIGHTGRAY:            Color = Color::new(0xd3, 0xd3, 0xd3, 0xff);
    /** #90ee90ff */ const LIGHTGREEN:           Color = Color::new(0x90, 0xee, 0x90, 0xff);
    /** #d3d3d3ff */ const LIGHTGREY:            Color = Color::new(0xd3, 0xd3, 0xd3, 0xff);
    /** #ffb6c1ff */ const LIGHTPINK:            Color = Color::new(0xff, 0xb6, 0xc1, 0xff);
    /** #ffa07aff */ const LIGHTSALMON:          Color = Color::new(0xff, 0xa0, 0x7a, 0xff);
    /** #20b2aaff */ const LIGHTSEAGREEN:        Color = Color::new(0x20, 0xb2, 0xaa, 0xff);
    /** #87cefaff */ const LIGHTSKYBLUE:         Color = Color::new(0x87, 0xce, 0xfa, 0xff);
    /** #778899ff */ const LIGHTSLATEGRAY:       Color = Color::new(0x77, 0x88, 0x99, 0xff);
    /** #778899ff */ const LIGHTSLATEGREY:       Color = Color::new(0x77, 0x88, 0x99, 0xff);
    /** #b0c4deff */ const LIGHTSTEELBLUE:       Color = Color::new(0xb0, 0xc4, 0xde, 0xff);
    /** #ffffe0ff */ const LIGHTYELLOW:          Color = Color::new(0xff, 0xff, 0xe0, 0xff);
    /** #00ff00ff */ const LIME:                 Color = Color::new(0x00, 0xff, 0x00, 0xff);
    /** #32cd32ff */ const LIMEGREEN:            Color = Color::new(0x32, 0xcd, 0x32, 0xff);
    /** #faf0e6ff */ const LINEN:                Color = Color::new(0xfa, 0xf0, 0xe6, 0xff);
    /** #ff00ffff */ const MAGENTA:              Color = Self::FUCHSIA;
    /** #800000ff */ const MAROON:               Color = Color::new(0x80, 0x00, 0x00, 0xff);
    /** #66cdaaff */ const MEDIUMAQUAMARINE:     Color = Color::new(0x66, 0xcd, 0xaa, 0xff);
    /** #0000cdff */ const MEDIUMBLUE:           Color = Color::new(0x00, 0x00, 0xcd, 0xff);
    /** #ba55d3ff */ const MEDIUMORCHID:         Color = Color::new(0xba, 0x55, 0xd3, 0xff);
    /** #9370dbff */ const MEDIUMPURPLE:         Color = Color::new(0x93, 0x70, 0xdb, 0xff);
    /** #3cb371ff */ const MEDIUMSEAGREEN:       Color = Color::new(0x3c, 0xb3, 0x71, 0xff);
    /** #7b68eeff */ const MEDIUMSLATEBLUE:      Color = Color::new(0x7b, 0x68, 0xee, 0xff);
    /** #00fa9aff */ const MEDIUMSPRINGGREEN:    Color = Color::new(0x00, 0xfa, 0x9a, 0xff);
    /** #48d1ccff */ const MEDIUMTURQUOISE:      Color = Color::new(0x48, 0xd1, 0xcc, 0xff);
    /** #c71585ff */ const MEDIUMVIOLETRED:      Color = Color::new(0xc7, 0x15, 0x85, 0xff);
    /** #191970ff */ const MIDNIGHTBLUE:         Color = Color::new(0x19, 0x19, 0x70, 0xff);
    /** #f5fffaff */ const MINTCREAM:            Color = Color::new(0xf5, 0xff, 0xfa, 0xff);
    /** #ffe4e1ff */ const MISTYROSE:            Color = Color::new(0xff, 0xe4, 0xe1, 0xff);
    /** #ffe4b5ff */ const MOCCASIN:             Color = Color::new(0xff, 0xe4, 0xb5, 0xff);
    /** #ffdeadff */ const NAVAJOWHITE:          Color = Color::new(0xff, 0xde, 0xad, 0xff);
    /** #000080ff */ const NAVY:                 Color = Color::new(0x00, 0x00, 0x80, 0xff);
    /** #fdf5e6ff */ const OLDLACE:              Color = Color::new(0xfd, 0xf5, 0xe6, 0xff);
    /** #808000ff */ const OLIVE:                Color = Color::new(0x80, 0x80, 0x00, 0xff);
    /** #6b8e23ff */ const OLIVEDRAB:            Color = Color::new(0x6b, 0x8e, 0x23, 0xff);
    /** #ffa500ff */ const ORANGE:               Color = Color::new(0xff, 0xa5, 0x00, 0xff);
    /** #ff4500ff */ const ORANGERED:            Color = Color::new(0xff, 0x45, 0x00, 0xff);
    /** #da70d6ff */ const ORCHID:               Color = Color::new(0xda, 0x70, 0xd6, 0xff);
    /** #eee8aaff */ const PALEGOLDENROD:        Color = Color::new(0xee, 0xe8, 0xaa, 0xff);
    /** #98fb98ff */ const PALEGREEN:            Color = Color::new(0x98, 0xfb, 0x98, 0xff);
    /** #afeeeeff */ const PALETURQUOISE:        Color = Color::new(0xaf, 0xee, 0xee, 0xff);
    /** #db7093ff */ const PALEVIOLETRED:        Color = Color::new(0xdb, 0x70, 0x93, 0xff);
    /** #ffefd5ff */ const PAPAYAWHIP:           Color = Color::new(0xff, 0xef, 0xd5, 0xff);
    /** #ffdab9ff */ const PEACHPUFF:            Color = Color::new(0xff, 0xda, 0xb9, 0xff);
    /** #cd853fff */ const PERU:                 Color = Color::new(0xcd, 0x85, 0x3f, 0xff);
    /** #ffc0cbff */ const PINK:                 Color = Color::new(0xff, 0xc0, 0xcb, 0xff);
    /** #dda0ddff */ const PLUM:                 Color = Color::new(0xdd, 0xa0, 0xdd, 0xff);
    /** #b0e0e6ff */ const POWDERBLUE:           Color = Color::new(0xb0, 0xe0, 0xe6, 0xff);
    /** #800080ff */ const PURPLE:               Color = Color::new(0x80, 0x00, 0x80, 0xff);
    /** #663399ff */ const REBECCAPURPLE:        Color = Color::new(0x66, 0x33, 0x99, 0xff);
    /** #ff0000ff */ const RED:                  Color = Color::new(0xff, 0x00, 0x00, 0xff);
    /** #bc8f8fff */ const ROSYBROWN:            Color = Color::new(0xbc, 0x8f, 0x8f, 0xff);
    /** #4169e1ff */ const ROYALBLUE:            Color = Color::new(0x41, 0x69, 0xe1, 0xff);
    /** #8b4513ff */ const SADDLEBROWN:          Color = Color::new(0x8b, 0x45, 0x13, 0xff);
    /** #fa8072ff */ const SALMON:               Color = Color::new(0xfa, 0x80, 0x72, 0xff);
    /** #f4a460ff */ const SANDYBROWN:           Color = Color::new(0xf4, 0xa4, 0x60, 0xff);
    /** #2e8b57ff */ const SEAGREEN:             Color = Color::new(0x2e, 0x8b, 0x57, 0xff);
    /** #fff5eeff */ const SEASHELL:             Color = Color::new(0xff, 0xf5, 0xee, 0xff);
    /** #a0522dff */ const SIENNA:               Color = Color::new(0xa0, 0x52, 0x2d, 0xff);
    /** #c0c0c0ff */ const SILVER:               Color = Color::new(0xc0, 0xc0, 0xc0, 0xff);
    /** #87ceebff */ const SKYBLUE:              Color = Color::new(0x87, 0xce, 0xeb, 0xff);
    /** #6a5acdff */ const SLATEBLUE:            Color = Color::new(0x6a, 0x5a, 0xcd, 0xff);
    /** #708090ff */ const SLATEGRAY:            Color = Color::new(0x70, 0x80, 0x90, 0xff);
    /** #708090ff */ const SLATEGREY:            Color = Color::new(0x70, 0x80, 0x90, 0xff);
    /** #fffafaff */ const SNOW:                 Color = Color::new(0xff, 0xfa, 0xfa, 0xff);
    /** #00ff7fff */ const SPRINGGREEN:          Color = Color::new(0x00, 0xff, 0x7f, 0xff);
    /** #4682b4ff */ const STEELBLUE:            Color = Color::new(0x46, 0x82, 0xb4, 0xff);
    /** #d2b48cff */ const TAN:                  Color = Color::new(0xd2, 0xb4, 0x8c, 0xff);
    /** #008080ff */ const TEAL:                 Color = Color::new(0x00, 0x80, 0x80, 0xff);
    /** #d8bfd8ff */ const THISTLE:              Color = Color::new(0xd8, 0xbf, 0xd8, 0xff);
    /** #00000000 */ const TRANSPARENT:          Color = Color::new(0x00, 0x00, 0x00, 0x00);
    /** #ff6347ff */ const TOMATO:               Color = Color::new(0xff, 0x63, 0x47, 0xff);
    /** #40e0d0ff */ const TURQUOISE:            Color = Color::new(0x40, 0xe0, 0xd0, 0xff);
    /** #ee82eeff */ const VIOLET:               Color = Color::new(0xee, 0x82, 0xee, 0xff);
    /** #f5deb3ff */ const WHEAT:                Color = Color::new(0xf5, 0xde, 0xb3, 0xff);
    /** #ffffffff */ const WHITE:                Color = Color::new(0xff, 0xff, 0xff, 0xff);
    /** #f5f5f5ff */ const WHITESMOKE:           Color = Color::new(0xf5, 0xf5, 0xf5, 0xff);
    /** #ffff00ff */ const YELLOW:               Color = Color::new(0xff, 0xff, 0x00, 0xff);
    /** #9acd32ff */ const YELLOWGREEN:          Color = Color::new(0x9a, 0xcd, 0x32, 0xff);
}
impl CSSPalette for Color {}

#[rustfmt::skip]
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
