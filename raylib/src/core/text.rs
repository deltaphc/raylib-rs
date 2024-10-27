//! Text and Font related functions
//! Text manipulation functions are super unsafe so use rust String functions
use raylib_sys::LoadUTF8;

use crate::core::math::Vector2;
use crate::core::texture::{Image, Texture2D};
use crate::core::{RaylibHandle, RaylibThread};
use crate::error::{error, Error};
use crate::ffi;
use crate::math::Rectangle;

use std::convert::{AsMut, AsRef, TryInto};
use std::ffi::{CString, OsString};
use std::mem::ManuallyDrop;
use std::ops::Deref;

fn no_drop<T>(_thing: T) {}
make_thin_wrapper!(Font, ffi::Font, ffi::UnloadFont);
make_thin_wrapper!(WeakFont, ffi::Font, no_drop);
make_thin_wrapper!(GlyphInfo, ffi::GlyphInfo, no_drop);

#[repr(transparent)]
#[derive(Debug)]
pub struct RSliceGlyphInfo(pub(crate) std::mem::ManuallyDrop<std::boxed::Box<[GlyphInfo]>>);

impl Drop for RSliceGlyphInfo {
    #[allow(unused_unsafe)]
    fn drop(&mut self) {
        unsafe {
            let inner = std::mem::ManuallyDrop::take(&mut self.0);
            let len = inner.len();
            ffi::UnloadFontData(
                std::boxed::Box::leak(inner).as_mut_ptr() as *mut _,
                len as i32,
            );
        }
    }
}

impl std::convert::AsRef<Box<[GlyphInfo]>> for RSliceGlyphInfo {
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

impl AsRef<ffi::Texture2D> for Font {
    fn as_ref(&self) -> &ffi::Texture2D {
        return &self.0.texture;
    }
}

impl AsRef<ffi::Texture2D> for WeakFont {
    fn as_ref(&self) -> &ffi::Texture2D {
        return &self.0.texture;
    }
}

pub(crate) struct Codepoints(pub(crate) ManuallyDrop<Box<[i32]>>);

impl Drop for Codepoints {
    fn drop(&mut self) {
        unsafe {
            ffi::UnloadCodepoints(self.0.as_mut_ptr());
        }
    }
}

impl RaylibHandle {
    pub(crate) fn load_codepoints(&mut self, text: &str) -> Codepoints {
        let ptr = CString::new(text).unwrap();
        let mut len = 0;
        let u = unsafe { ffi::LoadCodepoints(ptr.as_ptr(), &mut len) };

        unsafe {
            Codepoints(std::mem::ManuallyDrop::new(Box::from_raw(
                std::slice::from_raw_parts_mut(u, text.len()),
            )))
        }
    }

    pub fn get_codepoint_count(text: &str) -> i32 {
        let ptr = CString::new(text).unwrap();
        unsafe { ffi::GetCodepointCount(ptr.as_ptr()) }
    }

    pub fn unload_font(&mut self, font: WeakFont) {
        unsafe { ffi::UnloadFont(font.0) };
    }

    /// Loads font from file into GPU memory (VRAM).
    #[inline]
    pub fn load_font(&mut self, _: &RaylibThread, filename: &str) -> Result<Font, Error> {
        let c_filename = CString::new(filename).unwrap();
        let f = unsafe { ffi::LoadFont(c_filename.as_ptr()) };
        if f.glyphs.is_null() || f.texture.id == 0 {
            return Err(error!(
                "Error loading font. Check if the file exists and if it's the right type",
                filename,
            ));
        }
        Ok(Font(f))
    }

    /// Loads font from file with extended parameters.
    /// Supplying None for chars loads the entire character set.
    #[inline]
    pub fn load_font_ex(
        &mut self,
        _: &RaylibThread,
        filename: &str,
        font_size: i32,
        chars: Option<&str>,
    ) -> Result<Font, Error> {
        let c_filename = CString::new(filename).unwrap();
        let f = unsafe {
            match chars {
                Some(c) => {
                    let mut co = self.load_codepoints(c);
                    ffi::LoadFontEx(
                        c_filename.as_ptr(),
                        font_size,
                        co.0.as_mut_ptr(),
                        c.len() as i32,
                    )
                }
                None => ffi::LoadFontEx(c_filename.as_ptr(), font_size, std::ptr::null_mut(), 0),
            }
        };
        if f.glyphs.is_null() || f.texture.id == 0 {
            return Err(error!(
                "Error loading font. Check if the file exists and if it's the right type",
                filename,
            ));
        }
        Ok(Font(f))
    }

    /// Load font from Image (XNA style)
    #[inline]
    pub fn load_font_from_image(
        &mut self,
        _: &RaylibThread,
        image: &Image,
        key: impl Into<ffi::Color>,
        first_char: i32,
    ) -> Result<Font, Error> {
        let f = unsafe { ffi::LoadFontFromImage(image.0, key.into(), first_char) };
        if f.glyphs.is_null() {
            return Err(error!("Error loading font from image."));
        }
        Ok(Font(f))
    }
    /// Load font data from a given memory buffer.
    /// `file_type` refers to the extension, e.g. ".ttf".
    /// You can pass Some(...) to chars to get the desired charaters, or None to get the whole set.
    #[inline]
    pub fn load_font_from_memory(
        &mut self,
        _: &RaylibThread,
        file_type: &str,
        file_data: &[u8],
        font_size: i32,
        chars: Option<&str>,
    ) -> Result<Font, Error> {
        let c_file_type = CString::new(file_type).unwrap();
        let f = unsafe {
            match chars {
                Some(c) => {
                    let mut co = self.load_codepoints(c);
                    ffi::LoadFontFromMemory(
                        c_file_type.as_ptr(),
                        file_data.as_ptr(),
                        file_data.len() as i32,
                        font_size,
                        co.0.as_mut_ptr(),
                        c.len() as i32,
                    )
                }
                None => ffi::LoadFontFromMemory(
                    c_file_type.as_ptr(),
                    file_data.as_ptr(),
                    file_data.len() as i32,
                    font_size,
                    std::ptr::null_mut(),
                    0,
                ),
            }
        };
        if f.glyphs.is_null() || f.texture.id == 0 {
            return Err(error!(
                "Error loading font from memory. Check if the file's type is correct"
            ));
        }
        Ok(Font(f))
    }
    /// Loads font data for further use (see also `Font::from_data`).
    /// Now supports .tiff
    #[inline]
    pub fn load_font_data(
        &mut self,
        data: &[u8],
        font_size: i32,
        chars: Option<&str>,
        sdf: i32,
    ) -> Option<RSliceGlyphInfo> {
        unsafe {
            let ci_arr_ptr = match chars {
                Some(c) => {
                    let mut co = self.load_codepoints(c);
                    ffi::LoadFontData(
                        data.as_ptr(),
                        data.len() as i32,
                        font_size,
                        co.0.as_mut_ptr(),
                        c.len() as i32,
                        sdf,
                    )
                }
                None => ffi::LoadFontData(
                    data.as_ptr(),
                    data.len() as i32,
                    font_size,
                    std::ptr::null_mut(),
                    0,
                    sdf,
                ),
            };
            let ci_size = if let Some(c) = chars { c.len() } else { 95 }; // raylib assumes 95 if none given
            if ci_arr_ptr.is_null() {
                None
            } else {
                Some(RSliceGlyphInfo(std::mem::ManuallyDrop::new(Box::from_raw(
                    std::slice::from_raw_parts_mut(ci_arr_ptr as *mut _, ci_size),
                ))))
            }
        }
    }
}

impl RaylibFont for WeakFont {}
impl RaylibFont for Font {}

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

    /// Check if a font is valid
    fn is_font_valid(&self) -> bool {
        unsafe { ffi::IsFontValid(*self.as_ref()) }
    }

    /// Export font as code file, returns true on success
    fn export_font_as_code<A>(&self, filename: A) -> bool
    where
        A: Into<OsString>,
    {
        let c_str = CString::new(filename.into().to_string_lossy().as_bytes()).unwrap();
        unsafe { ffi::ExportFontAsCode(*self.as_ref(), c_str.as_ptr()) }
    }

    /// Get glyph font info data for a codepoint (unicode character), fallback to '?' if not found
    fn get_glyph_info(&self, codepoint: char) -> GlyphInfo {
        unsafe { GlyphInfo(ffi::GetGlyphInfo(*self.as_ref(), codepoint as i32)) }
    }

    /// Gets index position for a unicode character on `font`.
    fn get_glyph_index(&self, codepoint: char) -> i32 {
        unsafe { ffi::GetGlyphIndex(*self.as_ref(), codepoint as i32) }
    }

    /// Get glyph rectangle in font atlas for a codepoint (unicode character), fallback to '?' if not found
    fn get_glyph_atlas_rec(&self, codepoint: char) -> Rectangle {
        unsafe { ffi::GetGlyphAtlasRec(*self.as_ref(), codepoint as i32).into() }
    }

    /// Measures string width in pixels for `font`.
    fn measure_text(&self, text: &str, font_size: f32, spacing: f32) -> Vector2 {
        let c_text = CString::new(text).unwrap();
        unsafe { ffi::MeasureTextEx(*self.as_ref(), c_text.as_ptr(), font_size, spacing).into() }
    }
}

impl Font {
    pub fn make_weak(self) -> WeakFont {
        let w = WeakFont(self.0);
        std::mem::forget(self);
        return w;
    }
    /// Returns a new `Font` using provided `GlyphInfo` data and parameters.
    fn from_data(
        chars: &[ffi::GlyphInfo],
        base_size: i32,
        padding: i32,
        pack_method: i32,
    ) -> Result<Font, Error> {
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
            return Err(error!("Error loading font from image."));
        }
        Ok(f)
    }

    /// Sets the character data on the current Font.
    fn set_chars(&mut self, chars: &[ffi::GlyphInfo]) {
        unsafe {
            self.glyphCount = chars.len() as i32;
            let data_size = self.glyphCount as usize * std::mem::size_of::<ffi::GlyphInfo>();
            let ci_arr_ptr = ffi::MemAlloc(data_size.try_into().unwrap()); // raylib frees this data in UnloadFont
            std::ptr::copy(
                chars.as_ptr(),
                ci_arr_ptr as *mut ffi::GlyphInfo,
                chars.len(),
            );
            self.glyphs = ci_arr_ptr as *mut ffi::GlyphInfo;
        }
    }

    /// Sets the texture on the current Font, and takes ownership of `tex`.
    fn set_texture(&mut self, tex: Texture2D) {
        self.texture = tex.0;
        std::mem::forget(tex); // UnloadFont will also unload the texture
    }
}

/// Generates image font atlas using `chars` info.
/// Sets a pointer to an array of rectangles raylib allocated that MUST manually be freed.
/// Good luck freeing it safely though ;)
#[inline]
pub fn gen_image_font_atlas(
    _: &RaylibThread,
    chars: &mut [ffi::GlyphInfo],
    font_size: i32,
    padding: i32,
    pack_method: i32,
) -> (Image, Vec<ffi::Rectangle>) {
    unsafe {
        let mut ptr = std::ptr::null_mut();

        let img = Image(ffi::GenImageFontAtlas(
            chars.as_mut_ptr(),
            &mut ptr,
            font_size,
            chars.len() as i32,
            padding,
            pack_method,
        ));

        let mut recs = Vec::with_capacity(chars.len());
        #[allow(clippy::uninit_vec)]
        recs.set_len(chars.len());
        std::ptr::copy(ptr, recs.as_mut_ptr(), chars.len());
        ffi::MemFree(ptr as *mut ::std::os::raw::c_void);
        return (img, recs);
    }
}

impl RaylibHandle {
    /// Gets the default font.
    #[inline]
    pub fn get_font_default(&self) -> WeakFont {
        WeakFont(unsafe { ffi::GetFontDefault() })
    }
    /// Measures string width in pixels for default font.
    #[inline]
    pub fn measure_text(&self, text: &str, font_size: i32) -> i32 {
        let c_text = CString::new(text).unwrap();
        unsafe { ffi::MeasureText(c_text.as_ptr(), font_size) }
    }

    pub fn set_text_line_spacing(&self, spacing: i32) {
        unsafe { ffi::SetTextLineSpacing(spacing) }
    }
}
