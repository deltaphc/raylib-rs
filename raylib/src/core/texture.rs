//! Image and texture related functions
use crate::core::color::Color;
use crate::core::math::{Rectangle, Vector4};
use crate::core::{RaylibHandle, RaylibThread};
use crate::ffi;
use std::ffi::CString;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct NPatchInfo {
    pub source_rec: Rectangle,
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
    pub type_: crate::consts::NPatchType,
}

impl From<ffi::NPatchInfo> for NPatchInfo {
    fn from(v: ffi::NPatchInfo) -> NPatchInfo {
        unsafe { std::mem::transmute(v) }
    }
}

impl Into<ffi::NPatchInfo> for NPatchInfo {
    fn into(self) -> ffi::NPatchInfo {
        unsafe { std::mem::transmute(self) }
    }
}

impl Into<ffi::NPatchInfo> for &NPatchInfo {
    fn into(self) -> ffi::NPatchInfo {
        ffi::NPatchInfo {
            sourceRec: self.source_rec.into(),
            left: self.left,
            top: self.top,
            right: self.right,
            bottom: self.bottom,
            type_: (self.type_ as u32) as i32,
        }
    }
}

fn no_drop<T>(_thing: T) {}
make_thin_wrapper!(Image, ffi::Image, ffi::UnloadImage);
make_thin_wrapper!(Texture2D, ffi::Texture2D, ffi::UnloadTexture);
make_thin_wrapper!(WeakTexture2D, ffi::Texture2D, no_drop);
make_thin_wrapper!(
    RenderTexture2D,
    ffi::RenderTexture2D,
    ffi::UnloadRenderTexture
);
make_thin_wrapper!(WeakRenderTexture2D, ffi::RenderTexture2D, no_drop);

// Weak things can be clone
impl Clone for WeakTexture2D {
    fn clone(&self) -> WeakTexture2D {
        WeakTexture2D(self.0)
    }
}

// Weak things can be clone
impl Clone for WeakRenderTexture2D {
    fn clone(&self) -> WeakRenderTexture2D {
        WeakRenderTexture2D(self.0)
    }
}

impl RaylibRenderTexture2D for WeakRenderTexture2D {}
impl RaylibRenderTexture2D for RenderTexture2D {}

impl AsRef<ffi::Texture2D> for RenderTexture2D {
    fn as_ref(&self) -> &ffi::Texture2D {
        self.texture()
    }
}

impl AsMut<ffi::Texture2D> for RenderTexture2D {
    fn as_mut(&mut self) -> &mut ffi::Texture2D {
        self.texture_mut()
    }
}

impl AsRef<ffi::Texture2D> for WeakRenderTexture2D {
    fn as_ref(&self) -> &ffi::Texture2D {
        self.texture()
    }
}

impl AsMut<ffi::Texture2D> for WeakRenderTexture2D {
    fn as_mut(&mut self) -> &mut ffi::Texture2D {
        self.texture_mut()
    }
}

impl RenderTexture2D {
    pub unsafe fn make_weak(self) -> WeakRenderTexture2D {
        let m = WeakRenderTexture2D(self.0);
        std::mem::forget(self);
        m
    }
}

pub trait RaylibRenderTexture2D: AsRef<ffi::RenderTexture2D> + AsMut<ffi::RenderTexture2D> {
    fn id(&self) -> u32 {
        self.as_ref().id
    }

    fn texture(&self) -> &WeakTexture2D {
        unsafe { std::mem::transmute(&self.as_ref().texture) }
    }

    fn texture_mut(&mut self) -> &mut WeakTexture2D {
        unsafe { std::mem::transmute(&mut self.as_mut().texture) }
    }

    fn depth(&self) -> Option<&WeakTexture2D> {
        if self.as_ref().depthTexture {
            return unsafe { Some(std::mem::transmute(&self.as_ref().depth)) };
        }
        None
    }

    fn depth_mut(&mut self) -> Option<&mut WeakTexture2D> {
        if self.as_mut().depthTexture {
            return unsafe { Some(std::mem::transmute(&mut self.as_mut().depth)) };
        }
        None
    }
}

impl Clone for Image {
    fn clone(&self) -> Image {
        unsafe { Image(ffi::ImageCopy(self.0)) }
    }
}

impl Image {
    pub fn width(&self) -> i32 {
        self.0.width
    }
    pub fn height(&self) -> i32 {
        self.0.height
    }
    pub fn mipmaps(&self) -> i32 {
        self.0.mipmaps
    }
    pub unsafe fn data(&self) -> *mut ::std::os::raw::c_void {
        self.0.data
    }

    #[inline]
    pub fn format(&self) -> crate::consts::PixelFormat {
        let i: u32 = self.format as u32;
        unsafe { std::mem::transmute(i) }
    }

    #[inline]
    pub fn from_image(&self, rec: impl Into<ffi::Rectangle>) -> Image {
        unsafe { Image(ffi::ImageFromImage(self.0, rec.into())) }
    }

    /// Exports image as a PNG file.
    #[inline]
    pub fn export_image(&self, filename: &str) {
        let c_filename = CString::new(filename).unwrap();
        unsafe {
            ffi::ExportImage(self.0, c_filename.as_ptr());
        }
    }

    /// Exports image as a PNG file.
    #[inline]
    pub fn export_image_as_code(&self, filename: &str) {
        let c_filename = CString::new(filename).unwrap();
        unsafe {
            ffi::ExportImageAsCode(self.0, c_filename.as_ptr());
        }
    }

    /// Gets pixel data from `image` as a Vec of Color structs.
    #[inline]
    pub fn get_image_data(&self) -> Vec<Color> {
        unsafe {
            let image_data = ffi::GetImageData(self.0);
            let image_data_len = (self.width * self.height) as usize;
            let mut safe_image_data: Vec<Color> = Vec::with_capacity(image_data_len);
            safe_image_data.set_len(image_data_len);
            std::ptr::copy(
                image_data,
                safe_image_data.as_mut_ptr() as *mut ffi::Color,
                image_data_len,
            );
            libc::free(image_data as *mut libc::c_void);
            safe_image_data
        }
    }

    /// Gets normalized (`0.0` to `1.0`) pixel data from `image` as a Vec of Vector4 structs.
    #[inline]
    pub fn get_image_data_normalized(&self) -> Vec<Vector4> {
        unsafe {
            let image_data = ffi::GetImageDataNormalized(self.0);
            let image_data_len = (self.width * self.height) as usize;
            let mut safe_image_data: Vec<Vector4> = Vec::with_capacity(image_data_len);
            safe_image_data.set_len(image_data_len);
            std::ptr::copy(
                image_data,
                safe_image_data.as_mut_ptr() as *mut ffi::Vector4,
                image_data_len,
            );
            libc::free(image_data as *mut libc::c_void);
            safe_image_data
        }
    }

    /// Extract color palette from image to maximum size
    #[inline]
    pub fn extract_palette(&self, max_palette_size: u32) -> Vec<Color> {
        unsafe {
            let mut palette_len = 0;
            let image_data =
                ffi::ImageExtractPalette(self.0, max_palette_size as i32, &mut palette_len);
            let mut safe_image_data: Vec<Color> = Vec::with_capacity(palette_len as usize);
            safe_image_data.set_len(palette_len as usize);
            std::ptr::copy(
                image_data,
                safe_image_data.as_mut_ptr() as *mut ffi::Color,
                palette_len as usize,
            );
            // TODO replace this with raylib free
            libc::free(image_data as *mut libc::c_void);
            safe_image_data
        }
    }

    /// Converts `image` to POT (power-of-two).
    #[inline]
    pub fn to_pot(&mut self, fill_color: impl Into<ffi::Color>) {
        unsafe {
            ffi::ImageToPOT(&mut self.0, fill_color.into());
        }
    }

    /// Converts `image` data to desired pixel format.
    #[inline]
    pub fn set_format(&mut self, new_format: crate::consts::PixelFormat) {
        unsafe {
            ffi::ImageFormat(&mut self.0, (new_format as u32) as i32);
        }
    }

    /// Applies alpha mask to `image`.
    /// Alpha mask must be same size as the image. If alpha mask is not greyscale
    /// Ensure the colors are white (255, 255, 255, 255) or black (0, 0, 0, 0)
    #[inline]
    pub fn alpha_mask(&mut self, alpha_mask: &Image) {
        unsafe {
            ffi::ImageAlphaMask(&mut self.0, alpha_mask.0);
        }
    }

    /// Clears alpha channel on `image` to desired color.
    #[inline]
    pub fn alpha_clear(&mut self, color: impl Into<ffi::Color>, threshold: f32) {
        unsafe {
            ffi::ImageAlphaClear(&mut self.0, color.into(), threshold);
        }
    }

    /// Crops `image` depending on alpha value.
    #[inline]
    pub fn alpha_crop(&mut self, threshold: f32) {
        unsafe {
            ffi::ImageAlphaCrop(&mut self.0, threshold);
        }
    }

    /// Premultiplies alpha channel on `image`.
    #[inline]
    pub fn alpha_premultiply(&mut self) {
        unsafe {
            ffi::ImageAlphaPremultiply(&mut self.0);
        }
    }

    /// Crops `image` to a defined rectangle.
    #[inline]
    pub fn crop(&mut self, crop: impl Into<ffi::Rectangle>) {
        unsafe {
            ffi::ImageCrop(&mut self.0, crop.into());
        }
    }

    /// Resizes `image` (bilinear filtering).
    #[inline]
    pub fn resize(&mut self, new_width: i32, new_height: i32) {
        unsafe {
            ffi::ImageResize(&mut self.0, new_width, new_height);
        }
    }

    /// Resizes `image` (nearest-neighbor scaling).
    #[inline]
    pub fn resize_nn(&mut self, new_width: i32, new_height: i32) {
        unsafe {
            ffi::ImageResizeNN(&mut self.0, new_width, new_height);
        }
    }

    /// Resizes `image` canvas and fills with `color`.
    #[inline]
    pub fn resize_canvas(
        &mut self,
        new_width: i32,
        new_height: i32,
        offset_x: i32,
        offset_y: i32,
        color: impl Into<ffi::Color>,
    ) {
        unsafe {
            ffi::ImageResizeCanvas(
                &mut self.0,
                new_width,
                new_height,
                offset_x,
                offset_y,
                color.into(),
            );
        }
    }

    /// Generates all mipmap levels for a provided `image`.
    #[inline]
    pub fn gen_mipmaps(&mut self) {
        unsafe {
            ffi::ImageMipmaps(&mut self.0);
        }
    }

    /// Dithers `image` data to 16bpp or lower (Floyd-Steinberg dithering).
    #[inline]
    pub fn dither(&mut self, r_bpp: i32, g_bpp: i32, b_bpp: i32, a_bpp: i32) {
        unsafe {
            ffi::ImageDither(&mut self.0, r_bpp, g_bpp, b_bpp, a_bpp);
        }
    }

    /// Get image alpha border rectangle
    #[inline]
    pub fn get_image_alpha_border(&self, threshold: f32) -> Rectangle {
        unsafe { ffi::GetImageAlphaBorder(self.0, threshold).into() }
    }

    /// Clear image background with given color
    #[inline]
    pub fn clear_background(&mut self, color: impl Into<ffi::Color>) {
        unsafe { ffi::ImageClearBackground(&mut self.0, color.into()) }
    }

    /// Draws a source image within a destination image.
    #[inline]
    pub fn draw(
        &mut self,
        src: &Image,
        src_rec: Rectangle,
        dst_rec: Rectangle,
        tint: impl Into<ffi::Color>,
    ) {
        unsafe {
            ffi::ImageDraw(
                &mut self.0,
                src.0,
                src_rec.into(),
                dst_rec.into(),
                tint.into(),
            );
        }
    }

    /// Draw pixel within an image
    #[inline]
    pub fn draw_pixel(&mut self, pos_x: i32, pos_y: i32, color: impl Into<ffi::Color>) {
        unsafe { ffi::ImageDrawPixel(&mut self.0, pos_x, pos_y, color.into()) }
    }

    /// Draw pixel within an image (Vector version)
    #[inline]
    pub fn draw_pixel_v(
        &mut self,
        position: impl Into<ffi::Vector2>,
        color: impl Into<ffi::Color>,
    ) {
        unsafe { ffi::ImageDrawPixelV(&mut self.0, position.into(), color.into()) }
    }

    /// Draw line within an image
    #[inline]
    pub fn draw_line(
        &mut self,
        start_pos_x: i32,
        start_pos_y: i32,
        end_pos_x: i32,
        end_pos_y: i32,
        color: impl Into<ffi::Color>,
    ) {
        unsafe {
            ffi::ImageDrawLine(
                &mut self.0,
                start_pos_x,
                start_pos_y,
                end_pos_x,
                end_pos_y,
                color.into(),
            )
        }
    }

    /// Draw line within an image (Vector version)
    #[inline]
    pub fn draw_line_v(
        &mut self,
        start: impl Into<ffi::Vector2>,
        end: impl Into<ffi::Vector2>,
        color: impl Into<ffi::Color>,
    ) {
        unsafe { ffi::ImageDrawLineV(&mut self.0, start.into(), end.into(), color.into()) }
    }

    /// Draw circle within an image
    #[inline]
    pub fn draw_circle(
        &mut self,
        center_x: i32,
        center_y: i32,
        radius: i32,
        color: impl Into<ffi::Color>,
    ) {
        unsafe { ffi::ImageDrawCircle(&mut self.0, center_x, center_y, radius, color.into()) }
    }

    /// Draw circle within an image (Vector version)
    #[inline]
    pub fn draw_circle_v(
        &mut self,
        center: impl Into<ffi::Vector2>,
        radius: i32,
        color: impl Into<ffi::Color>,
    ) {
        unsafe { ffi::ImageDrawCircleV(&mut self.0, center.into(), radius, color.into()) }
    }

    /// Draws a rectangle within an image.
    #[inline]
    pub fn draw_rectangle(
        &mut self,
        pos_x: i32,
        pos_y: i32,
        width: i32,
        height: i32,
        color: impl Into<ffi::Color>,
    ) {
        unsafe {
            ffi::ImageDrawRectangle(&mut self.0, pos_x, pos_y, width, height, color.into());
        }
    }

    /// Draws a rectangle within an image.
    #[inline]
    pub fn draw_rectangle_lines(
        &mut self,
        rec: Rectangle,
        thickness: i32,
        color: impl Into<ffi::Color>,
    ) {
        unsafe {
            ffi::ImageDrawRectangleLines(&mut self.0, rec.into(), thickness, color.into());
        }
    }

    /// Draws text (default font) within an image (destination).
    #[inline]
    pub fn draw_text(
        &mut self,
        position: impl Into<ffi::Vector2>,
        text: &str,
        font_size: i32,
        color: impl Into<ffi::Color>,
    ) {
        let c_text = CString::new(text).unwrap();
        unsafe {
            ffi::ImageDrawText(
                &mut self.0,
                position.into(),
                c_text.as_ptr(),
                font_size,
                color.into(),
            );
        }
    }

    /// Draws text (default font) within an image (destination).
    #[inline]
    pub fn draw_text_ex(
        &mut self,
        position: impl Into<ffi::Vector2>,
        font: impl AsRef<ffi::Font>,
        text: &str,
        font_size: f32,
        spacing: f32,
        color: impl Into<ffi::Color>,
    ) {
        let c_text = CString::new(text).unwrap();
        unsafe {
            ffi::ImageDrawTextEx(
                &mut self.0,
                position.into(),
                *font.as_ref(),
                c_text.as_ptr(),
                font_size,
                spacing,
                color.into(),
            );
        }
    }

    /// Flips `image` vertically.
    #[inline]
    pub fn flip_vertical(&mut self) {
        unsafe {
            ffi::ImageFlipVertical(&mut self.0);
        }
    }

    /// Flips `image` horizontally.
    #[inline]
    pub fn flip_horizontal(&mut self) {
        unsafe {
            ffi::ImageFlipHorizontal(&mut self.0);
        }
    }

    /// Rotates `image` clockwise by 90 degrees (PI/2 radians).
    #[inline]
    pub fn rotate_cw(&mut self) {
        unsafe {
            ffi::ImageRotateCW(&mut self.0);
        }
    }

    /// Rotates `image` counterclockwise by 90 degrees (PI/2 radians).
    #[inline]
    pub fn rotate_ccw(&mut self) {
        unsafe {
            ffi::ImageRotateCCW(&mut self.0);
        }
    }

    /// Tints colors in `image` using specified `color`.
    #[inline]
    pub fn color_tint(&mut self, color: impl Into<ffi::Color>) {
        unsafe {
            ffi::ImageColorTint(&mut self.0, color.into());
        }
    }

    /// Inverts the colors in `image`.
    #[inline]
    pub fn color_invert(&mut self) {
        unsafe {
            ffi::ImageColorInvert(&mut self.0);
        }
    }

    /// Converts `image color to grayscale.
    #[inline]
    pub fn color_grayscale(&mut self) {
        unsafe {
            ffi::ImageColorGrayscale(&mut self.0);
        }
    }

    /// Adjusts the contrast of `image`.
    #[inline]
    pub fn color_contrast(&mut self, contrast: f32) {
        unsafe {
            ffi::ImageColorContrast(&mut self.0, contrast);
        }
    }

    /// Adjusts the brightness of `image`.
    #[inline]
    pub fn color_brightness(&mut self, brightness: i32) {
        unsafe {
            ffi::ImageColorBrightness(&mut self.0, brightness);
        }
    }

    /// Searches `image` for all occurences of `color` and replaces them with `replace` color.
    #[inline]
    pub fn color_replace(&mut self, color: impl Into<ffi::Color>, replace: impl Into<ffi::Color>) {
        unsafe {
            ffi::ImageColorReplace(&mut self.0, color.into(), replace.into());
        }
    }

    /// Generates a plain `color` Image.
    #[inline]
    pub fn gen_image_color(width: i32, height: i32, color: impl Into<ffi::Color>) -> Image {
        unsafe { Image(ffi::GenImageColor(width, height, color.into())) }
    }

    /// Generates an Image containing a vertical gradient.
    #[inline]
    pub fn gen_image_gradient_v(
        width: i32,
        height: i32,
        top: impl Into<ffi::Color>,
        bottom: impl Into<ffi::Color>,
    ) -> Image {
        unsafe {
            Image(ffi::GenImageGradientV(
                width,
                height,
                top.into(),
                bottom.into(),
            ))
        }
    }

    /// Generates an Image containing a horizonal gradient.
    #[inline]
    pub fn gen_image_gradient_h(
        width: i32,
        height: i32,
        left: impl Into<ffi::Color>,
        right: impl Into<ffi::Color>,
    ) -> Image {
        unsafe {
            Image(ffi::GenImageGradientH(
                width,
                height,
                left.into(),
                right.into(),
            ))
        }
    }

    /// Generates an Image containing a radial gradient.
    #[inline]
    pub fn gen_image_gradient_radial(
        width: i32,
        height: i32,
        density: f32,
        inner: impl Into<ffi::Color>,
        outer: impl Into<ffi::Color>,
    ) -> Image {
        unsafe {
            Image(ffi::GenImageGradientRadial(
                width,
                height,
                density,
                inner.into(),
                outer.into(),
            ))
        }
    }

    /// Generates an Image containing a checkerboard pattern.
    #[inline]
    pub fn gen_image_checked(
        width: i32,
        height: i32,
        checks_x: i32,
        checks_y: i32,
        col1: impl Into<ffi::Color>,
        col2: impl Into<ffi::Color>,
    ) -> Image {
        unsafe {
            Image(ffi::GenImageChecked(
                width,
                height,
                checks_x,
                checks_y,
                col1.into(),
                col2.into(),
            ))
        }
    }

    /// Generates an Image containing white noise.
    #[inline]
    pub fn gen_image_white_noise(width: i32, height: i32, factor: f32) -> Image {
        unsafe { Image(ffi::GenImageWhiteNoise(width, height, factor)) }
    }

    /// Generates an Image containing perlin noise.
    #[inline]
    pub fn gen_image_perlin_noise(
        width: i32,
        height: i32,
        offset_x: i32,
        offset_y: i32,
        scale: f32,
    ) -> Image {
        unsafe {
            Image(ffi::GenImagePerlinNoise(
                width, height, offset_x, offset_y, scale,
            ))
        }
    }

    /// Generates an Image using a cellular algorithm. Bigger `tile_size` means bigger cells.
    #[inline]
    pub fn gen_image_cellular(width: i32, height: i32, tile_size: i32) -> Image {
        unsafe { Image(ffi::GenImageCellular(width, height, tile_size)) }
    }

    /// Loads image from file into CPU memory (RAM).
    pub fn load_image(filename: &str) -> Result<Image, String> {
        let c_filename = CString::new(filename).unwrap();
        let i = unsafe { ffi::LoadImage(c_filename.as_ptr()) };
        if i.data.is_null() {
            return Err(format!(
            "Image data is null. Either the file doesnt exist or the image type is unsupported."
        ));
        }
        Ok(Image(i))
    }

    /// Loads image from Color array data (RGBA - 32bit).
    pub fn load_image_ex(pixels: &[Color], width: i32, height: i32) -> Result<Image, String> {
        let expected_len = (width * height) as usize;
        if pixels.len() != expected_len {
            return Err(format!(
                "load_image_ex: Data is wrong size. Expected {}, got {}",
                expected_len,
                pixels.len()
            ));
        }
        unsafe {
            // An examination of Raylib source (textures.c) shows that it does not mutate the given pixels
            // this never fails no need for null check
            Ok(Image(ffi::LoadImageEx(
                pixels.as_ptr() as *mut ffi::Color,
                width,
                height,
            )))
        }
    }

    /// Loads image from raw data with parameters.
    pub fn load_image_pro(
        data: &[u8],
        width: i32,
        height: i32,
        format: crate::consts::PixelFormat,
    ) -> Result<Image, String> {
        let expected_len = get_pixel_data_size(width, height, format) as usize;
        if data.len() != expected_len {
            return Err(format!(
                "load_image_pro: Data is wrong size. Expected {}, got {}",
                expected_len,
                data.len()
            ));
        }
        unsafe {
            Ok(Image(ffi::LoadImagePro(
                data.as_ptr() as *mut std::os::raw::c_void,
                width,
                height,
                format as i32,
            )))
        }
    }

    /// Loads image from RAW file data.
    pub fn load_image_raw(
        filename: &str,
        width: i32,
        height: i32,
        format: i32,
        header_size: i32,
    ) -> Result<Image, String> {
        let c_filename = CString::new(filename).unwrap();
        let i =
            unsafe { ffi::LoadImageRaw(c_filename.as_ptr(), width, height, format, header_size) };
        if i.data.is_null() {
            return Err(format!(
            "Image data is null. Either the file doesnt exist or the image type is unsupported."
        ));
        }
        Ok(Image(i))
    }

    /// Creates an image from `text` (custom font).
    #[inline]
    pub fn image_text(text: &str, font_size: i32, color: impl Into<ffi::Color>) -> Image {
        let c_text = CString::new(text).unwrap();
        unsafe { Image(ffi::ImageText(c_text.as_ptr(), font_size, color.into())) }
    }

    /// Creates an image from `text` (custom font).
    #[inline]
    pub fn image_text_ex(
        font: impl std::convert::AsRef<ffi::Font>,
        text: &str,
        font_size: f32,
        spacing: f32,
        tint: impl Into<ffi::Color>,
    ) -> Image {
        let c_text = CString::new(text).unwrap();
        unsafe {
            Image(ffi::ImageTextEx(
                *font.as_ref(),
                c_text.as_ptr(),
                font_size,
                spacing,
                tint.into(),
            ))
        }
    }
}

impl RaylibTexture2D for WeakTexture2D {}
impl RaylibTexture2D for Texture2D {}
impl RaylibTexture2D for WeakRenderTexture2D {}
impl RaylibTexture2D for RenderTexture2D {}

impl Texture2D {
    pub unsafe fn make_weak(self) -> WeakTexture2D {
        let m = WeakTexture2D(self.0);
        std::mem::forget(self);
        m
    }
}

pub trait RaylibTexture2D: AsRef<ffi::Texture2D> + AsMut<ffi::Texture2D> {
    fn width(&self) -> i32 {
        self.as_ref().width
    }

    fn height(&self) -> i32 {
        self.as_ref().height
    }

    fn mipmaps(&self) -> i32 {
        self.as_ref().width
    }

    fn format(&self) -> i32 {
        self.as_ref().format
    }

    /// Updates GPU texture with new data.
    #[inline]
    fn update_texture(&mut self, pixels: &[u8]) {
        let expected_len = unsafe {
            get_pixel_data_size(
                self.as_ref().width,
                self.as_ref().height,
                std::mem::transmute::<i32, ffi::PixelFormat>(self.as_ref().format),
            ) as usize
        };
        if pixels.len() != expected_len {
            panic!(
                "update_texture: Data is wrong size. Expected {}, got {}",
                expected_len,
                pixels.len()
            );
        }
        unsafe {
            ffi::UpdateTexture(
                *self.as_mut(),
                pixels.as_ptr() as *const std::os::raw::c_void,
            );
        }
    }

    /// Gets pixel data from GPU texture and returns an `Image`.
    /// Fairly sure this would never fail. If it does wrap in result.
    #[inline]
    fn get_texture_data(&self) -> Result<Image, String> {
        let i = unsafe { ffi::GetTextureData(*self.as_ref()) };
        if i.data.is_null() {
            return Err(format!("Texture cannot be rendered to an image"));
        }
        Ok(Image(i))
    }

    /// Generates GPU mipmaps for a `texture`.
    #[inline]
    fn gen_texture_mipmaps(&mut self) {
        unsafe {
            ffi::GenTextureMipmaps(self.as_mut());
        }
    }

    /// Sets global `texture` scaling filter mode.
    #[inline]
    fn set_texture_filter(&self, _: &RaylibThread, filter_mode: crate::consts::TextureFilterMode) {
        unsafe {
            ffi::SetTextureFilter(*self.as_ref(), filter_mode as i32);
        }
    }

    /// Sets global texture wrapping mode.
    #[inline]
    fn set_texture_wrap(&self, _: &RaylibThread, wrap_mode: crate::consts::TextureWrapMode) {
        unsafe {
            ffi::SetTextureWrap(*self.as_ref(), wrap_mode as i32);
        }
    }
}

/// Gets pixel data size in bytes (image or texture).
#[inline]
pub fn get_pixel_data_size(width: i32, height: i32, format: ffi::PixelFormat) -> i32 {
    unsafe { ffi::GetPixelDataSize(width, height, format as i32) }
}

impl RaylibHandle {
    /// Loads texture from file into GPU memory (VRAM).
    pub fn load_texture(&mut self, _: &RaylibThread, filename: &str) -> Result<Texture2D, String> {
        let c_filename = CString::new(filename).unwrap();
        let t = unsafe { ffi::LoadTexture(c_filename.as_ptr()) };
        if t.id == 0 {
            return Err(format!("failed to load {} as a texture.", filename));
        }
        Ok(Texture2D(t))
    }

    /// Load cubemap from image, multiple image cubemap layouts supported
    pub fn load_texture_cubemap(
        &mut self,
        _: &RaylibThread,
        image: &Image,
        layout: crate::consts::CubemapLayoutType,
    ) -> Result<Texture2D, String> {
        let t = unsafe { ffi::LoadTextureCubemap(image.0, std::mem::transmute(layout)) };
        if t.id == 0 {
            return Err(format!("failed to load image as a texture cubemap."));
        }
        Ok(Texture2D(t))
    }

    /// Loads texture from image data.
    #[inline]
    pub fn load_texture_from_image(
        &mut self,
        _: &RaylibThread,
        image: &Image,
    ) -> Result<Texture2D, String> {
        let t = unsafe { ffi::LoadTextureFromImage(image.0) };
        if t.id == 0 {
            return Err(format!("failed to load image as a texture."));
        }
        Ok(Texture2D(t))
    }

    /// Loads texture for rendering (framebuffer).
    pub fn load_render_texture(
        &mut self,
        _: &RaylibThread,
        width: u32,
        height: u32,
    ) -> Result<RenderTexture2D, String> {
        let t = unsafe { ffi::LoadRenderTexture(width as i32, height as i32) };
        if t.id == 0 {
            return Err(format!("failed to create render texture."));
        }
        Ok(RenderTexture2D(t))
    }
}

impl RaylibHandle {
    /// Generate cubemap texture from 2D texture
    pub fn gen_texture_cubemap(
        &mut self,
        _: &RaylibThread,
        shader: impl AsRef<ffi::Shader>,
        map: impl AsRef<ffi::Texture2D>,
        size: i32,
    ) -> Texture2D {
        unsafe {
            Texture2D(ffi::GenTextureCubemap(
                *shader.as_ref(),
                *map.as_ref(),
                size,
            ))
        }
    }

    /// Generate irradiance texture using cubemap data
    pub fn gen_texture_irradiance(
        &mut self,
        _: &RaylibThread,
        shader: impl AsRef<ffi::Shader>,
        cubemap: impl AsRef<ffi::Texture2D>,
        size: i32,
    ) -> Texture2D {
        unsafe {
            Texture2D(ffi::GenTextureIrradiance(
                *shader.as_ref(),
                *cubemap.as_ref(),
                size,
            ))
        }
    }

    /// Generate prefilter texture using cubemap data
    pub fn gen_texture_prefilter(
        &mut self,
        _: &RaylibThread,
        shader: impl AsRef<ffi::Shader>,
        cubemap: impl AsRef<ffi::Texture2D>,
        size: i32,
    ) -> Texture2D {
        unsafe {
            Texture2D(ffi::GenTexturePrefilter(
                *shader.as_ref(),
                *cubemap.as_ref(),
                size,
            ))
        }
    }

    /// Generate BRDF texture
    pub fn gen_texture_brdf(
        &mut self,
        _: &RaylibThread,
        shader: impl AsRef<ffi::Shader>,
        size: i32,
    ) -> Texture2D {
        unsafe { Texture2D(ffi::GenTextureBRDF(*shader.as_ref(), size)) }
    }
}

impl RaylibHandle {
    /// Weak Textures will leak memeory if they are not unlaoded
    /// Unload textures from GPU memory (VRAM)
    pub unsafe fn unload_texture(&mut self, _: &RaylibThread, texture: WeakTexture2D) {
        {
            ffi::UnloadTexture(*texture.as_ref())
        }
    }
    /// Weak RenderTextures will leak memeory if they are not unlaoded
    /// Unload RenderTextures from GPU memory (VRAM)
    pub unsafe fn unload_render_texture(&mut self, _: &RaylibThread, texture: WeakRenderTexture2D) {
        {
            ffi::UnloadRenderTexture(*texture.as_ref())
        }
    }
}
