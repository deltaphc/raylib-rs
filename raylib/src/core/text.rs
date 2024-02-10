//! Text and Font related functions
//! Text manipulation functions are super unsafe so use rust String functions
use crate::core::math::Vector2;
use crate::core::texture::{Image, Texture2D};
use crate::core::{RaylibHandle, RaylibThread};
use crate::ffi;

use std::convert::{AsMut, AsRef};
use std::ffi::CString;

fn no_drop<T>(_thing: T) {}
make_thin_wrapper!(Font, ffi::Font, ffi::UnloadFont);
make_thin_wrapper!(WeakFont, ffi::Font, no_drop);
make_thin_wrapper!(CharInfo, ffi::CharInfo, no_drop);

#[repr(transparent)]
#[derive(Debug)]
pub struct RSliceCharInfo(pub(crate) std::mem::ManuallyDrop<std::boxed::Box<[CharInfo]>>);

impl Drop for RSliceCharInfo {
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

impl std::convert::AsRef<Box<[CharInfo]>> for RSliceCharInfo {
    fn as_ref(&self) -> &Box<[CharInfo]> {
        &self.0
    }
}

impl std::convert::AsMut<Box<[CharInfo]>> for RSliceCharInfo {
    fn as_mut(&mut self) -> &mut Box<[CharInfo]> {
        &mut self.0
    }
}

impl std::ops::Deref for RSliceCharInfo {
    type Target = Box<[CharInfo]>;
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for RSliceCharInfo {
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

/// Parameters for Font::load_font_ex
pub enum FontLoadEx<'a> {
    /// Count from default font
    Default(i32),
    Chars(&'a [i32]),
}

impl RaylibHandle {
    pub fn unload_font(&mut self, font: WeakFont) {
        unsafe { ffi::UnloadFont(font.0) };
    }

    /// Loads font from file into GPU memory (VRAM).
    #[inline]
    pub fn load_font(&mut self, _: &RaylibThread, filename: &str) -> Result<Font, String> {
        let c_filename = CString::new(filename).unwrap();
        let f = unsafe { ffi::LoadFont(c_filename.as_ptr()) };
        if f.chars.is_null() || f.texture.id == 0 {
            return Err(format!(
                "Error loading font {}. Does it exist? Is it the right type?",
                filename
            ));
        }
        Ok(Font(f))
    }

    /// Loads font from file with extended parameters.
    #[inline]
    pub fn load_font_ex(
        &mut self,
        _: &RaylibThread,
        filename: &str,
        font_size: i32,
        chars: FontLoadEx,
    ) -> Result<Font, String> {
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
                    ffi::LoadFontEx(c_filename.as_ptr(), font_size, std::ptr::null_mut(), count)
                }
            }
        };
        if f.chars.is_null() || f.texture.id == 0 {
            return Err(format!(
                "Error loading font {}. Does it exist? Is it the right type?",
                filename
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
    ) -> Result<Font, String> {
        let f = unsafe { ffi::LoadFontFromImage(image.0, key.into(), first_char) };
        if f.chars.is_null() {
            return Err(format!("Error loading font from image."));
        }
        Ok(Font(f))
    }
    /// Load font data from a given memory buffer.
    /// `file_type` refers to the extension, e.g. ".ttf".
    #[inline]
    pub fn load_font_from_memory(
        &mut self,
        _: &RaylibThread,
        file_type: &str,
        file_data: &[u8],
        font_size: i32,
        chars: FontLoadEx,
    ) -> Result<Font, String> {
        let c_file_type = CString::new(file_type).unwrap();
        let f = unsafe {
            match chars {
                FontLoadEx::Chars(c) => ffi::LoadFontFromMemory(
                    c_file_type.as_ptr(),
                    file_data.as_ptr(),
                    file_data.len() as i32,
                    font_size,
                    c.as_ptr() as *mut i32,
                    c.len() as i32,
                ),
                FontLoadEx::Default(count) => ffi::LoadFontFromMemory(
                    c_file_type.as_ptr(),
                    file_data.as_ptr(),
                    file_data.len() as i32,
                    font_size,
                    std::ptr::null_mut(),
                    count,
                ),
            }
        };
        if f.chars.is_null() || f.texture.id == 0 {
            return Err(format!(
                "Error loading font from memory. Is it the right type?"
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
        chars: Option<&[i32]>,
        sdf: i32,
    ) -> Option<RSliceCharInfo> {
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
                    std::ptr::null_mut(),
                    0,
                    sdf,
                ),
            };
            let ci_size = if let Some(c) = chars { c.len() } else { 95 }; // raylib assumes 95 if none given
            if ci_arr_ptr.is_null() {
                None
            } else {
                Some(RSliceCharInfo(std::mem::ManuallyDrop::new(Box::from_raw(
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
    fn chars(&self) -> &[CharInfo] {
        unsafe {
            std::slice::from_raw_parts(
                self.as_ref().chars as *const CharInfo,
                self.as_ref().charsCount as usize,
            )
        }
    }
    fn chars_mut(&mut self) -> &mut [CharInfo] {
        unsafe {
            std::slice::from_raw_parts_mut(
                self.as_mut().chars as *mut CharInfo,
                self.as_ref().charsCount as usize,
            )
        }
    }
}

impl Font {
    pub fn make_weak(self) -> WeakFont {
        let w = WeakFont(self.0);
        std::mem::forget(self);
        return w;
    }
    /// Returns a new `Font` using provided `CharInfo` data and parameters.
    fn from_data(
        chars: &[ffi::CharInfo],
        base_size: i32,
        padding: i32,
        pack_method: i32,
    ) -> Result<Font, String> {
        let f = unsafe {
            let mut f = std::mem::zeroed::<Font>();
            f.baseSize = base_size;
            f.set_chars(chars);

            let atlas = ffi::GenImageFontAtlas(
                f.chars,
                &mut f.0.recs,
                f.baseSize,
                f.charsCount,
                padding,
                pack_method,
            );
            f.texture = ffi::LoadTextureFromImage(atlas);
            ffi::UnloadImage(atlas);
            f
        };
        if f.0.chars.is_null() || f.0.texture.id == 0 {
            return Err(format!("Error loading font from image."));
        }
        Ok(f)
    }

    /// Sets the character data on the current Font.
    fn set_chars(&mut self, chars: &[ffi::CharInfo]) {
        unsafe {
            self.charsCount = chars.len() as i32;
            let data_size = self.charsCount as usize * std::mem::size_of::<ffi::CharInfo>();
            let ci_arr_ptr = libc::malloc(data_size); // raylib frees this data in UnloadFont
            std::ptr::copy(
                chars.as_ptr(),
                ci_arr_ptr as *mut ffi::CharInfo,
                chars.len(),
            );
            self.chars = ci_arr_ptr as *mut ffi::CharInfo;
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
    chars: &mut [ffi::CharInfo],
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
        recs.set_len(chars.len());
        std::ptr::copy(ptr, recs.as_mut_ptr(), chars.len());
        ffi::MemFree(ptr as *mut libc::c_void);
        return (img, recs);
    }
}

impl RaylibHandle {
    /// Gets the default font.
    #[inline]
    pub fn get_font_default(&self) -> WeakFont {
        WeakFont(unsafe { ffi::GetFontDefault() })
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
    font: impl std::convert::AsRef<ffi::Font>,
    text: &str,
    font_size: f32,
    spacing: f32,
) -> Vector2 {
    let c_text = CString::new(text).unwrap();
    unsafe { ffi::MeasureTextEx(*font.as_ref(), c_text.as_ptr(), font_size, spacing).into() }
}

/// Gets index position for a unicode character on `font`.
#[inline]
pub fn get_glyph_index(font: impl std::convert::AsRef<ffi::Font>, character: i32) -> i32 {
    unsafe { ffi::GetGlyphIndex(*font.as_ref(), character) }
}
