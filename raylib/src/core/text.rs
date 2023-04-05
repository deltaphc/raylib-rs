//! Text and Font related functions
//! Text manipulation functions are super unsafe so use rust String functions
use std::boxed::Box;
use std::convert::{AsMut, AsRef};
use std::ffi::CString;
use std::mem::ManuallyDrop;
use std::ptr;

use super::{
    texture::{Image, Texture2D},
    RaylibHandle, RaylibThread,
};
use crate::{
    buffer::RaylibBuffer,
    ffi::{self, Color, Rectangle, Vector2},
    make_bound_thin_wrapper, make_thin_wrapper,
};

fn no_drop<T>(_thing: T) {}
make_bound_thin_wrapper!(Font, ffi::Font, ffi::UnloadFont, RaylibHandle<'bind>);
make_thin_wrapper!(GlyphInfo, ffi::GlyphInfo, no_drop);

#[repr(transparent)]
#[derive(Debug)]
pub struct RSliceGlyphInfo(pub(crate) ManuallyDrop<Box<[GlyphInfo]>>);

impl Drop for RSliceGlyphInfo {
    #[allow(unused_unsafe)]
    fn drop(&mut self) {
        unsafe {
            let inner = ManuallyDrop::take(&mut self.0);
            let len = inner.len();
            ffi::UnloadFontData(Box::leak(inner).as_mut_ptr() as *mut _, len as i32);
        }
    }
}

impl AsRef<Box<[GlyphInfo]>> for RSliceGlyphInfo {
    fn as_ref(&self) -> &Box<[GlyphInfo]> {
        &self.0
    }
}

impl std::convert::AsMut<Box<[GlyphInfo]>> for RSliceGlyphInfo {
    fn as_mut(&mut self) -> &mut Box<[GlyphInfo]> {
        &mut self.0
    }
}

impl std::ops::Deref for RSliceGlyphInfo {
    type Target = Box<[GlyphInfo]>;
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for RSliceGlyphInfo {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

// #[cfg(feature = "nightly")]
// impl !Send for Font {}
// #[cfg(feature = "nightly")]
// unsafe impl Sync for Font {}
// #[cfg(feature = "nightly")]
// impl !Send for WeakFont {}
// #[cfg(feature = "nightly")]
// unsafe impl Sync for WeakFont {}

impl<'bind, 'a> AsRef<ffi::Texture2D> for Font<'bind, 'a> {
    fn as_ref(&self) -> &ffi::Texture2D {
        &self.0.texture
    }
}

/// Parameters for Font::load_font_ex
pub enum FontLoadEx<'a> {
    /// Count from default font
    Default(i32),
    Chars(&'a [i32]),
}

impl<'bind> RaylibHandle<'_> {
    /// Loads font from file into GPU memory (VRAM).
    #[inline]
    pub fn load_font(
        &'bind self,
        _: &RaylibThread,
        filename: &str,
    ) -> Result<Font<'bind, '_>, String> {
        let c_filename = CString::new(filename).unwrap();
        let f = unsafe { ffi::LoadFont(c_filename.as_ptr()) };
        if f.glyphs.is_null() || f.texture.id == 0 {
            return Err(format!(
                "Error loading font {}. Does it exist? Is it the right type?",
                filename
            ));
        }
        Ok(unsafe { Font::from_raw(f) })
    }

    /// Loads font from file with extended parameters.
    #[inline]
    pub fn load_font_ex(
        &'bind self,
        _: &RaylibThread,
        filename: &str,
        font_size: i32,
        chars: FontLoadEx,
    ) -> Result<Font<'bind, '_>, String> {
        let c_filename = CString::new(filename).unwrap();
        let f = unsafe {
            match chars {
                FontLoadEx::Chars(c) => ffi::LoadFontEx(
                    c_filename.as_ptr(),
                    font_size,
                    c.as_ptr() as *mut i32,
                    c.len() as i32,
                ),
                FontLoadEx::Default(count) => {
                    ffi::LoadFontEx(c_filename.as_ptr(), font_size, ptr::null_mut(), count)
                }
            }
        };
        if f.glyphs.is_null() || f.texture.id == 0 {
            return Err(format!(
                "Error loading font {}. Does it exist? Is it the right type?",
                filename
            ));
        }
        Ok(unsafe { Font::from_raw(f) })
    }

    /// Load font from Image (XNA style)
    #[inline]
    pub fn load_font_from_image(
        &'bind self,
        _: &RaylibThread,
        image: &Image,
        key: Color,
        first_char: i32,
    ) -> Result<Font<'bind, '_>, String> {
        let f = unsafe { ffi::LoadFontFromImage(image.0, key, first_char) };
        if f.glyphs.is_null() {
            return Err("Error loading font from image.".to_string());
        }
        Ok(unsafe { Font::from_raw(f) })
    }

    /// Loads font data for further use (see also `Font::from_data`).
    /// Now supports .tiff
    #[inline]
    pub fn load_font_data(
        &self,
        data: &[u8],
        font_size: i32,
        chars: Option<&[i32]>,
        sdf: i32,
    ) -> Option<RSliceGlyphInfo> {
        unsafe {
            let ci_arr_ptr = match chars {
                Some(c) => ffi::LoadFontData(
                    data.as_ptr(),
                    data.len() as i32,
                    font_size,
                    c.as_ptr() as *mut i32,
                    c.len() as i32,
                    sdf,
                ),
                None => ffi::LoadFontData(
                    data.as_ptr(),
                    data.len() as i32,
                    font_size,
                    ptr::null_mut(),
                    0,
                    sdf,
                ),
            };
            let ci_size = if let Some(c) = chars { c.len() } else { 95 }; // raylib assumes 95 if none given
            if ci_arr_ptr.is_null() {
                None
            } else {
                Some(RSliceGlyphInfo(ManuallyDrop::new(Box::from_raw(
                    std::slice::from_raw_parts_mut(ci_arr_ptr as *mut _, ci_size),
                ))))
            }
        }
    }
}

impl RaylibFont for Font<'_, '_> {}

pub trait RaylibFont: AsRef<ffi::Font> + AsMut<ffi::Font> {
    fn base_size(&self) -> i32 {
        self.as_ref().baseSize
    }
    fn texture(&self) -> &Texture2D {
        unsafe { std::mem::transmute(&self.as_ref().texture) }
    }
    fn chars(&self) -> &[GlyphInfo] {
        unsafe {
            std::slice::from_raw_parts(
                self.as_ref().glyphs as *const GlyphInfo,
                self.as_ref().glyphCount as usize,
            )
        }
    }
    fn chars_mut(&mut self) -> &mut [GlyphInfo] {
        unsafe {
            std::slice::from_raw_parts_mut(
                self.as_mut().glyphs as *mut GlyphInfo,
                self.as_ref().glyphCount as usize,
            )
        }
    }
}

impl<'a, 'bind> Font<'bind, 'a> {
    /// Returns a new `Font` using provided `GlyphInfo` data and parameters.
    pub fn from_data(
        &'bind mut self,
        chars: &[ffi::GlyphInfo],
        base_size: i32,
        padding: i32,
        pack_method: i32,
    ) -> Result<Font<'bind, 'a>, String> {
        let f = unsafe {
            let mut f = std::mem::zeroed::<Font>();
            f.baseSize = base_size;
            f.set_chars(chars);

            let atlas = ffi::GenImageFontAtlas(
                f.glyphs,
                &mut f.0.recs,
                f.baseSize,
                f.glyphCount,
                padding,
                pack_method,
            );
            f.texture = ffi::LoadTextureFromImage(atlas);
            ffi::UnloadImage(atlas);
            f
        };
        if f.0.glyphs.is_null() || f.0.texture.id == 0 {
            return Err("Error loading font from image.".to_string());
        }
        Ok(f)
    }

    /// Sets the character data on the current Font.
    pub fn set_chars(&mut self, chars: &[ffi::GlyphInfo]) {
        unsafe {
            self.glyphCount = chars.len() as i32;
            let data_size = self.glyphCount as usize * std::mem::size_of::<ffi::GlyphInfo>();
            let ci_arr_ptr = libc::malloc(data_size); // raylib frees this data in UnloadFont
            ptr::copy(
                chars.as_ptr(),
                ci_arr_ptr as *mut ffi::GlyphInfo,
                chars.len(),
            );
            self.glyphs = ci_arr_ptr as *mut ffi::GlyphInfo;
        }
    }

    /// Sets the texture on the current Font, and takes ownership of `tex`.
    pub fn set_texture(&mut self, tex: Texture2D) {
        self.texture = tex.0;
        std::mem::forget(tex); // UnloadFont will also unload the texture
    }
}

/// Generates image font atlas using `chars` info.
#[inline]
pub fn gen_image_font_atlas(
    _: &RaylibThread,
    chars: &mut [ffi::GlyphInfo],
    font_size: i32,
    padding: i32,
    pack_method: i32,
) -> Option<(Image, RaylibBuffer<'static, Rectangle>)> {
    unsafe {
        let mut ptr = ptr::null_mut();

        let img = Image(ffi::GenImageFontAtlas(
            chars.as_mut_ptr(),
            &mut ptr,
            font_size,
            chars.len() as i32,
            padding,
            pack_method,
        ));

        let buffer = RaylibBuffer::<Rectangle>::new(ptr, chars.len());

        buffer.map(|b| (img, b))
    }
}

impl<'bind, 'a> RaylibHandle<'_> {
    /// Gets the default font.
    #[inline]
    pub fn get_font_default(&'bind self) -> ManuallyDrop<Font<'bind, 'a>> {
        ManuallyDrop::new(unsafe { Font::from_raw(ffi::GetFontDefault()) })
    }
}

/// Measures string width in pixels for default font.
#[inline]
pub fn measure_text(text: &str, font_size: i32) -> i32 {
    let c_text = CString::new(text).unwrap();
    unsafe { ffi::MeasureText(c_text.as_ptr(), font_size) }
}

/// Measures string width in pixels for `font`.
#[inline]
pub fn measure_text_ex(
    font: impl AsRef<ffi::Font>,
    text: &str,
    font_size: f32,
    spacing: f32,
) -> Vector2 {
    let c_text = CString::new(text).unwrap();
    unsafe { ffi::MeasureTextEx(*font.as_ref(), c_text.as_ptr(), font_size, spacing) }
}

/// Gets index position for a unicode character on `font`.
#[inline]
pub fn get_glyph_index(font: impl AsRef<ffi::Font>, character: i32) -> i32 {
    unsafe { ffi::GetGlyphIndex(*font.as_ref(), character) }
}
