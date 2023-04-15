//! Image and texture related functions
use std::ffi::CString;

use super::{RaylibHandle, RaylibThread};
use crate::{
    buffer::RaylibBuffer,
    ffi::{
        self, Color, CubemapLayout, NPatchLayout, PixelFormat, Rectangle, TextureFilter,
        TextureWrap, Vector2,
    },
    impl_wrapper, make_bound_thin_wrapper,
};

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct NPatchInfo {
    pub source: Rectangle,
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
    pub layout: NPatchLayout,
}

impl From<ffi::NPatchInfo> for NPatchInfo {
    fn from(v: ffi::NPatchInfo) -> NPatchInfo {
        unsafe { std::mem::transmute(v) }
    }
}

impl From<NPatchInfo> for ffi::NPatchInfo {
    fn from(val: NPatchInfo) -> Self {
        unsafe { std::mem::transmute(val) }
    }
}

#[repr(transparent)]
#[derive(Debug)]
pub struct Image(pub(crate) ffi::Image);

impl_wrapper!(Image, ffi::Image, (ffi::UnloadImage), 0);

make_bound_thin_wrapper!(
    Texture2D,
    ffi::Texture2D,
    ffi::UnloadTexture,
    RaylibHandle<'bind>
);
make_bound_thin_wrapper!(
    RenderTexture2D,
    ffi::RenderTexture2D,
    ffi::UnloadRenderTexture,
    RaylibHandle<'bind>
);

impl<'bind, 'a> AsRef<ffi::Texture2D> for RenderTexture2D<'bind, 'a> {
    fn as_ref(&self) -> &ffi::Texture2D {
        &self.0.texture
    }
}

impl<'bind, 'a> AsMut<ffi::Texture2D> for RenderTexture2D<'bind, 'a> {
    fn as_mut(&mut self) -> &mut ffi::Texture2D {
        &mut self.0.texture
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
    pub unsafe fn data(&self) -> *mut core::ffi::c_void {
        self.0.data
    }

    #[inline]
    pub fn format(&self) -> PixelFormat {
        let i: u32 = self.0.format as u32;
        unsafe { std::mem::transmute(i) }
    }

    #[inline]
    pub fn from_image(&self, rec: Rectangle) -> Image {
        unsafe { Image(ffi::ImageFromImage(self.0, rec)) }
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

    /// Get pixel data size in bytes (image or texture)
    pub fn get_pixel_data_size(&self) -> usize {
        unsafe { ffi::GetPixelDataSize(self.width(), self.height(), self.format() as i32) as usize }
    }

    /// Gets pixel data from `image` as a Vec of Color structs.
    pub fn get_image_data(&self) -> RaylibBuffer<Color> {
        unsafe {
            let image_data = ffi::LoadImageColors(self.0);
            let image_data_len = (self.0.width * self.0.height) as usize;

            RaylibBuffer::new(image_data as _, image_data_len).unwrap()
        }
    }

    /// Extract color palette from image to maximum size
    #[inline]
    pub fn extract_palette(&self, max_palette_size: u32) -> RaylibBuffer<Color> {
        unsafe {
            let mut palette_len = 0;
            let image_data =
                ffi::LoadImagePalette(self.0, max_palette_size as i32, &mut palette_len);

            RaylibBuffer::new(image_data as _, palette_len as usize).unwrap()
        }
    }

    /// Converts `image` to POT (power-of-two).
    #[inline]
    pub fn to_pot(&mut self, fill_color: Color) {
        unsafe {
            ffi::ImageToPOT(&mut self.0, fill_color);
        }
    }

    /// Converts `image` data to desired pixel format.
    #[inline]
    pub fn set_format(&mut self, new_format: PixelFormat) {
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
    pub fn alpha_clear(&mut self, color: Color, threshold: f32) {
        unsafe { ffi::ImageAlphaClear(&mut self.0, color, threshold) }
    }

    /// Crops `image` depending on alpha value.
    #[inline]
    pub fn alpha_crop(&mut self, threshold: f32) {
        unsafe { ffi::ImageAlphaCrop(&mut self.0, threshold) }
    }

    /// Premultiplies alpha channel on `image`.
    #[inline]
    pub fn alpha_premultiply(&mut self) {
        unsafe { ffi::ImageAlphaPremultiply(&mut self.0) }
    }

    /// Apply Gaussian blur using a box blur approximation
    #[inline]
    pub fn blur_gaussian(&mut self, blur_size: i32) {
        unsafe { ffi::ImageBlurGaussian(&mut self.0, blur_size) }
    }

    /// Crops `image` to a defined rectangle.
    #[inline]
    pub fn crop(&mut self, crop: Rectangle) {
        unsafe { ffi::ImageCrop(&mut self.0, crop) }
    }

    /// Resizes `image` (bilinear filtering).
    #[inline]
    pub fn resize(&mut self, new_width: i32, new_height: i32) {
        unsafe { ffi::ImageResize(&mut self.0, new_width, new_height) }
    }

    /// Resizes `image` (nearest-neighbor scaling).
    #[inline]
    pub fn resize_nn(&mut self, new_width: i32, new_height: i32) {
        unsafe { ffi::ImageResizeNN(&mut self.0, new_width, new_height) }
    }

    /// Resizes `image` canvas and fills with `color`.
    #[inline]
    pub fn resize_canvas(
        &mut self,
        new_width: i32,
        new_height: i32,
        offset_x: i32,
        offset_y: i32,
        color: Color,
    ) {
        unsafe {
            ffi::ImageResizeCanvas(
                &mut self.0,
                new_width,
                new_height,
                offset_x,
                offset_y,
                color,
            )
        }
    }

    /// Generates all mipmap levels for a provided `image`.
    #[inline]
    pub fn gen_mipmaps(&mut self) {
        unsafe { ffi::ImageMipmaps(&mut self.0) }
    }

    /// Dithers `image` data to 16bpp or lower (Floyd-Steinberg dithering).
    #[inline]
    pub fn dither(&mut self, r_bpp: i32, g_bpp: i32, b_bpp: i32, a_bpp: i32) {
        unsafe { ffi::ImageDither(&mut self.0, r_bpp, g_bpp, b_bpp, a_bpp) }
    }

    /// Get image alpha border rectangle
    #[inline]
    pub fn get_image_alpha_border(&self, threshold: f32) -> Rectangle {
        unsafe { ffi::GetImageAlphaBorder(self.0, threshold) }
    }

    /// Clear image background with given color
    #[inline]
    pub fn clear_background(&mut self, color: Color) {
        unsafe { ffi::ImageClearBackground(&mut self.0, color) }
    }

    /// Draws a source image within a destination image.
    #[inline]
    pub fn draw(&mut self, src: &Image, src_rec: Rectangle, dst_rec: Rectangle, tint: Color) {
        unsafe { ffi::ImageDraw(&mut self.0, src.0, src_rec, dst_rec, tint) }
    }

    /// Draw pixel within an image
    #[inline]
    pub fn draw_pixel(&mut self, pos_x: i32, pos_y: i32, color: Color) {
        unsafe { ffi::ImageDrawPixel(&mut self.0, pos_x, pos_y, color) }
    }

    /// Draw pixel within an image (Vector version)
    #[inline]
    pub fn draw_pixel_v(&mut self, position: Vector2, color: Color) {
        unsafe { ffi::ImageDrawPixelV(&mut self.0, position, color) }
    }

    /// Draw line within an image
    #[inline]
    pub fn draw_line(
        &mut self,
        start_pos_x: i32,
        start_pos_y: i32,
        end_pos_x: i32,
        end_pos_y: i32,
        color: Color,
    ) {
        unsafe {
            ffi::ImageDrawLine(
                &mut self.0,
                start_pos_x,
                start_pos_y,
                end_pos_x,
                end_pos_y,
                color,
            )
        }
    }

    /// Draw line within an image (Vector version)
    #[inline]
    pub fn draw_line_v(&mut self, start: Vector2, end: Vector2, color: Color) {
        unsafe { ffi::ImageDrawLineV(&mut self.0, start, end, color) }
    }

    /// Draw circle within an image
    #[inline]
    pub fn draw_circle(&mut self, center_x: i32, center_y: i32, radius: i32, color: Color) {
        unsafe { ffi::ImageDrawCircle(&mut self.0, center_x, center_y, radius, color) }
    }

    /// Draw circle within an image (Vector version)
    #[inline]
    pub fn draw_circle_v(&mut self, center: Vector2, radius: i32, color: Color) {
        unsafe { ffi::ImageDrawCircleV(&mut self.0, center, radius, color) }
    }

    /// Draws a rectangle within an image.
    #[inline]
    pub fn draw_rectangle(
        &mut self,
        pos_x: i32,
        pos_y: i32,
        width: i32,
        height: i32,
        color: Color,
    ) {
        unsafe {
            ffi::ImageDrawRectangle(&mut self.0, pos_x, pos_y, width, height, color);
        }
    }

    /// Draws a rectangle within an image.
    #[inline]
    pub fn draw_rectangle_lines(&mut self, rec: Rectangle, thickness: i32, color: Color) {
        unsafe {
            ffi::ImageDrawRectangleLines(&mut self.0, rec, thickness, color);
        }
    }

    /// Draws text (default font) within an image (destination).
    #[inline]
    pub fn draw_text(&mut self, text: &str, pos_x: i32, pos_y: i32, font_size: i32, color: Color) {
        let c_text = CString::new(text).unwrap();
        unsafe {
            ffi::ImageDrawText(&mut self.0, c_text.as_ptr(), pos_x, pos_y, font_size, color);
        }
    }

    /// Draws text (default font) within an image (destination).
    #[inline]
    pub fn draw_text_ex(
        &mut self,
        font: impl AsRef<ffi::Font>,
        text: &str,
        position: Vector2,
        font_size: f32,
        spacing: f32,
        color: Color,
    ) {
        let c_text = CString::new(text).unwrap();
        unsafe {
            ffi::ImageDrawTextEx(
                &mut self.0,
                *font.as_ref(),
                c_text.as_ptr(),
                position,
                font_size,
                spacing,
                color,
            );
        }
    }

    /// Flips `image` vertically.
    #[inline]
    pub fn flip_vertical(&mut self) {
        unsafe { ffi::ImageFlipVertical(&mut self.0) }
    }

    /// Flips `image` horizontally.
    #[inline]
    pub fn flip_horizontal(&mut self) {
        unsafe { ffi::ImageFlipHorizontal(&mut self.0) }
    }

    /// Rotates `image` clockwise by 90 degrees (PI/2 radians).
    #[inline]
    pub fn rotate_cw(&mut self) {
        unsafe { ffi::ImageRotateCW(&mut self.0) }
    }

    /// Rotates `image` counterclockwise by 90 degrees (PI/2 radians).
    #[inline]
    pub fn rotate_ccw(&mut self) {
        unsafe { ffi::ImageRotateCCW(&mut self.0) }
    }

    /// Tints colors in `image` using specified `color`.
    #[inline]
    pub fn color_tint(&mut self, color: Color) {
        unsafe { ffi::ImageColorTint(&mut self.0, color) }
    }

    /// Inverts the colors in `image`.
    #[inline]
    pub fn color_invert(&mut self) {
        unsafe { ffi::ImageColorInvert(&mut self.0) }
    }

    /// Converts `image color to grayscale.
    #[inline]
    pub fn color_grayscale(&mut self) {
        unsafe { ffi::ImageColorGrayscale(&mut self.0) }
    }

    /// Adjusts the contrast of `image`.
    #[inline]
    pub fn color_contrast(&mut self, contrast: f32) {
        unsafe { ffi::ImageColorContrast(&mut self.0, contrast) }
    }

    /// Adjusts the brightness of `image`.
    #[inline]
    pub fn color_brightness(&mut self, brightness: i32) {
        unsafe { ffi::ImageColorBrightness(&mut self.0, brightness) }
    }

    /// Searches `image` for all occurences of `color` and replaces them with `replace` color.
    #[inline]
    pub fn color_replace(&mut self, color: Color, replace: Color) {
        unsafe { ffi::ImageColorReplace(&mut self.0, color, replace) }
    }

    /// Generates a plain `color` Image.
    #[inline]
    pub fn gen_image_color(width: i32, height: i32, color: Color) -> Image {
        unsafe { Image(ffi::GenImageColor(width, height, color)) }
    }

    /// Generates an Image containing a vertical gradient.
    #[inline]
    pub fn gen_image_gradient_v(width: i32, height: i32, top: Color, bottom: Color) -> Image {
        unsafe { Image(ffi::GenImageGradientV(width, height, top, bottom)) }
    }

    /// Generates an Image containing a horizonal gradient.
    #[inline]
    pub fn gen_image_gradient_h(width: i32, height: i32, left: Color, right: Color) -> Image {
        unsafe { Image(ffi::GenImageGradientH(width, height, left, right)) }
    }

    /// Generates an Image containing a radial gradient.
    #[inline]
    pub fn gen_image_gradient_radial(
        width: i32,
        height: i32,
        density: f32,
        inner: Color,
        outer: Color,
    ) -> Image {
        unsafe {
            Image(ffi::GenImageGradientRadial(
                width, height, density, inner, outer,
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
        col1: Color,
        col2: Color,
    ) -> Image {
        unsafe {
            Image(ffi::GenImageChecked(
                width, height, checks_x, checks_y, col1, col2,
            ))
        }
    }

    /// Generates an Image containing white noise.
    #[inline]
    pub fn gen_image_white_noise(width: i32, height: i32, factor: f32) -> Image {
        unsafe { Image(ffi::GenImageWhiteNoise(width, height, factor)) }
    }

    /// Generates an Image containing white noise.
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
            return Err(
            "Image data is null. Either the file doesnt exist or the image type is unsupported.".to_string()
        );
        }
        Ok(Image(i))
    }

    /// Loads image from a given memory buffer as a vector of arrays
    pub fn load_image_from_mem(
        filetype: &str,
        bytes: &Vec<u8>,
        size: i32,
    ) -> Result<Image, String> {
        let c_filetype = CString::new(filetype).unwrap();
        let c_bytes = bytes.as_ptr();
        let i = unsafe { ffi::LoadImageFromMemory(c_filetype.as_ptr(), c_bytes, size) };
        if i.data.is_null() {
            return Err("Image data is null. Check provided buffer data".to_string());
        };
        Ok(Image(i))
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
            return Err("Image data is null. Either the file doesnt exist or the image type is unsupported.".to_string()
        );
        }
        Ok(Image(i))
    }

    /// Load image sequence from file (frames appended to image.data)
    pub fn load_image_anim(filename: &str) -> (Image, u32) {
        let c_filename = CString::new(filename).unwrap();
        let mut count = 0i32;
        let image = unsafe { ffi::LoadImageAnim(c_filename.as_ptr(), &mut count) };

        (Image(image), count as u32)
    }

    /// Creates an image from `text` (custom font).
    #[inline]
    pub fn image_text(text: &str, font_size: i32, color: Color) -> Image {
        let c_text = CString::new(text).unwrap();
        unsafe { Image(ffi::ImageText(c_text.as_ptr(), font_size, color)) }
    }

    /// Creates an image from `text` (custom font).
    #[inline]
    pub fn image_text_ex(
        font: impl std::convert::AsRef<ffi::Font>,
        text: &str,
        font_size: f32,
        spacing: f32,
        tint: Color,
    ) -> Image {
        let c_text = CString::new(text).unwrap();
        unsafe {
            Image(ffi::ImageTextEx(
                *font.as_ref(),
                c_text.as_ptr(),
                font_size,
                spacing,
                tint,
            ))
        }
    }
}

impl<'bind, 'a> RaylibTexture2D for Texture2D<'bind, 'a> {}
impl<'bind, 'a> RaylibTexture2D for RenderTexture2D<'bind, 'a> {}

pub trait RaylibTexture2D: AsRef<ffi::Texture2D> + AsMut<ffi::Texture2D> {
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
            ffi::UpdateTexture(*self.as_mut(), pixels.as_ptr() as *const core::ffi::c_void);
        }
    }

    /// Gets pixel data from GPU texture and returns an `Image`.
    /// Fairly sure this would never fail. If it does wrap in result.
    #[inline]
    fn load_image(&self) -> Result<Image, String> {
        let i = unsafe { ffi::LoadImageFromTexture(*self.as_ref()) };
        if i.data.is_null() {
            return Err("Texture cannot be rendered to an image".to_string());
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
    fn set_texture_filter(&self, _: &RaylibThread, filter_mode: TextureFilter) {
        unsafe {
            ffi::SetTextureFilter(*self.as_ref(), filter_mode as i32);
        }
    }

    /// Sets global texture wrapping mode.
    #[inline]
    fn set_texture_wrap(&self, _: &RaylibThread, wrap_mode: TextureWrap) {
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

impl<'bind, 'a> RaylibHandle<'_> {
    /// Loads texture from file into GPU memory (VRAM).
    pub fn load_texture(
        &'bind self,
        _: &RaylibThread,
        filename: &str,
    ) -> Result<Texture2D<'bind, 'a>, String> {
        let c_filename = CString::new(filename).unwrap();
        let t = unsafe { ffi::LoadTexture(c_filename.as_ptr()) };
        if t.id == 0 {
            return Err(format!("failed to load {} as a texture.", filename));
        }
        Ok(unsafe { Texture2D::from_raw(t) })
    }

    /// Load cubemap from image, multiple image cubemap layouts supported
    pub fn load_texture_cubemap(
        &'bind self,
        _: &RaylibThread,
        image: &Image,
        layout: CubemapLayout,
    ) -> Result<Texture2D<'bind, 'a>, String> {
        let t = unsafe { ffi::LoadTextureCubemap(image.0, layout as i32) };
        if t.id == 0 {
            return Err("failed to load image as a texture cubemap.".to_string());
        }
        Ok(unsafe { Texture2D::from_raw(t) })
    }

    /// Loads texture from image data.
    #[inline]
    pub fn load_texture_from_image(
        &'bind self,
        _: &RaylibThread,
        image: &Image,
    ) -> Result<Texture2D<'bind, 'a>, String> {
        let t = unsafe { ffi::LoadTextureFromImage(image.0) };
        if t.id == 0 {
            return Err("failed to load image as a texture.".to_string());
        }
        Ok(unsafe { Texture2D::from_raw(t) })
    }

    /// Loads texture for rendering (framebuffer).
    pub fn load_render_texture(
        &'bind self,
        _: &RaylibThread,
        width: u32,
        height: u32,
    ) -> Result<RenderTexture2D<'bind, 'a>, String> {
        let t = unsafe { ffi::LoadRenderTexture(width as i32, height as i32) };
        if t.id == 0 {
            return Err("failed to create render texture.".to_string());
        }
        Ok(unsafe { RenderTexture2D::from_raw(t) })
    }
}
