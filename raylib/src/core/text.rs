use crate::core::*;
use crate::ffi;

use std::convert::AsRef;
use std::ffi::CString;

fn no_drop<T>(_thing: T) {}
make_thin_wrapper!(Font, ffi::Font, ffi::UnloadFont);
/// WeakFont is a font that will leak memory when dropped.
/// Must called unload on the font
make_thin_wrapper!(WeakFont, ffi::Font, no_drop);

impl !Send for Font {}
unsafe impl Sync for Font {}
impl !Send for WeakFont {}
unsafe impl Sync for WeakFont {}

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

    /// Loads font data for further use (see also `Font::from_data`).
    #[inline]
    pub fn load_font_data(
        &mut self,
        filename: &str,
        font_size: i32,
        chars: Option<&[i32]>,
        sdf: i32,
    ) -> Vec<ffi::CharInfo> {
        let c_filename = CString::new(filename).unwrap();
        unsafe {
            let ci_arr_ptr = match chars {
                Some(c) => ffi::LoadFontData(
                    c_filename.as_ptr(),
                    font_size,
                    c.as_ptr() as *mut i32,
                    c.len() as i32,
                    sdf,
                ),
                None => {
                    ffi::LoadFontData(c_filename.as_ptr(), font_size, std::ptr::null_mut(), 0, sdf)
                }
            };
            let ci_size = if let Some(c) = chars { c.len() } else { 95 }; // raylib assumes 95 if none given
            let mut ci_vec = Vec::with_capacity(ci_size);
            for i in 0..ci_size {
                ci_vec.push(*ci_arr_ptr.offset(i as isize));
            }
            libc::free(ci_arr_ptr as *mut libc::c_void);
            ci_vec
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

            let atlas =
                ffi::GenImageFontAtlas(f.chars, f.baseSize, f.charsCount, padding, pack_method);
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
#[inline]
pub fn gen_image_font_atlas(
    _: &RaylibThread,
    chars: &mut [ffi::CharInfo],
    font_size: i32,
    padding: i32,
    pack_method: i32,
) -> Image {
    unsafe {
        Image(ffi::GenImageFontAtlas(
            chars.as_mut_ptr(),
            font_size,
            chars.len() as i32,
            padding,
            pack_method,
        ))
    }
}

impl RaylibHandle {
    /// Measures string width in pixels for default font.
    #[inline]
    pub fn measure_text(&self, text: &str, font_size: i32) -> i32 {
        let c_text = CString::new(text).unwrap();
        unsafe { ffi::MeasureText(c_text.as_ptr(), font_size) }
    }

    /// Gets the default font.
    #[inline]
    pub fn get_font_default(&self) -> WeakFont {
        WeakFont(unsafe { ffi::GetFontDefault() })
    }
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

#[cfg(test)]
mod text_test {
    use crate::core::*;
    use crate::tests::*;
    ray_test!(test_font_load);
    fn test_font_load(thread: &RaylibThread) {
        let mut handle = TEST_HANDLE.write().unwrap();
        let rl = handle.as_mut().unwrap();
        let f = rl
            .load_font(thread, "resources/alagard.png")
            .expect("couldn't load font");
    }

    ray_draw_test!(test_default_font);
    fn test_default_font(d: &mut RaylibDrawHandle, _: &TestAssets) {
        d.clear_background(Color::WHITE);
        d.draw_fps(0, 0);
        d.draw_text("Hello World", 100, 100, 32, Color::RED);
    }

    ray_draw_test!(test_custom_font);
    fn test_custom_font(d: &mut RaylibDrawHandle, assets: &TestAssets) {
        d.clear_background(Color::WHITE);
        d.draw_fps(0, 0);
        d.draw_text_ex(
            &assets.font,
            "Hello World",
            Vector2::new(100.0, 100.0),
            32.0,
            5.0,
            Color::RED,
        );
    }
}
