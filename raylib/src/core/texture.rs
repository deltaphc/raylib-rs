//! Image and texture related functions

use crate::core::color::Color;
use crate::core::math::Rectangle;
use crate::core::{RaylibHandle, RaylibThread};
use crate::error::{error, Error};
use crate::ffi;
use std::convert::TryInto;
use std::ffi::CString;
use std::mem::ManuallyDrop;
use std::os::raw::c_void;
use std::ptr::{null, null_mut};

use super::math::Vector2;

make_rslice!(ImagePalette, Color, ffi::UnloadImagePalette);
make_rslice!(ImageColors, Color, ffi::UnloadImageColors);

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct NPatchInfo {
    pub source: Rectangle,
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
    pub layout: crate::consts::NPatchLayout,
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
            source: self.source.into(),
            left: self.left,
            top: self.top,
            right: self.right,
            bottom: self.bottom,
            layout: (self.layout as u32) as i32,
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

    pub fn is_render_texture_valid(&self) -> bool {
        unsafe { ffi::IsRenderTextureValid(self.0) }
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

    /// Apply Gaussian blur using a box blur approximation
    pub fn blur_gaussian(&mut self, blur_size: i32) {
        unsafe { ffi::ImageBlurGaussian(&mut self.0, blur_size) }
    }
    /// Rotate image by input angle in degrees (-359 to 359)
    pub fn rotate(&mut self, degrees: i32) {
        unsafe { ffi::ImageRotate(&mut self.0, degrees) }
    }
    /// Get image pixel color at (x, y) position
    pub fn get_color(&mut self, x: i32, y: i32) -> Color {
        Color::from(unsafe { ffi::GetImageColor(self.0, x, y) })
    }
    /// Draw circle outline within an image
    pub fn draw_circle_lines(
        &mut self,
        center_x: i32,
        center_y: i32,
        radius: i32,
        color: crate::prelude::Color,
    ) {
        unsafe { ffi::ImageDrawCircleLines(&mut self.0, center_x, center_y, radius, color.into()) }
    }
    /// Draw circle outline within an image (Vector version)
    pub fn draw_circle_lines_v(
        &mut self,
        center: crate::prelude::Vector2,
        center_y: i32,
        color: Color,
    ) {
        unsafe { ffi::ImageDrawCircleLinesV(&mut self.0, center.into(), center_y, color.into()) }
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

    pub fn from_channel(&self, selected_channel: i32) -> Image {
        unsafe { Image(ffi::ImageFromChannel(self.0, selected_channel)) }
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
    pub fn get_image_data(&self) -> ImageColors {
        unsafe {
            let image_data = ffi::LoadImageColors(self.0);
            let image_data_len = (self.width * self.height) as usize;
            ImageColors(ManuallyDrop::new(Box::from_raw(
                std::slice::from_raw_parts_mut(image_data as *mut _, image_data_len),
            )))
        }
    }

    /// Extract color palette from image to maximum size
    #[inline]
    pub fn extract_palette(&self, max_palette_size: u32) -> ImagePalette {
        unsafe {
            let mut palette_len = 0;
            let image_data =
                ffi::LoadImagePalette(self.0, max_palette_size as i32, &mut palette_len);
            ImagePalette(ManuallyDrop::new(Box::from_raw(
                std::slice::from_raw_parts_mut(image_data as *mut _, palette_len as usize),
            )))
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

    /// Draw a line (using triangles/quads)
    pub fn draw_line_ex(
        &mut self,
        start_pos: impl Into<ffi::Vector2>,
        end_pos: impl Into<ffi::Vector2>,
        thick: i32,
        color: impl Into<ffi::Color>,
    ) {
        unsafe {
            ffi::ImageDrawLineEx(
                &mut self.0,
                start_pos.into(),
                end_pos.into(),
                thick,
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

    /// Draw triangle within an image
    pub fn draw_triangle(
        &mut self,
        v1: impl Into<ffi::Vector2>,
        v2: impl Into<ffi::Vector2>,
        v3: impl Into<ffi::Vector2>,
        color: impl Into<ffi::Color>,
    ) {
        unsafe {
            ffi::ImageDrawTriangle(&mut self.0, v1.into(), v2.into(), v3.into(), color.into())
        }
    }

    /// Draw triangle with interpolated colors within an image
    pub fn draw_triangle_ex(
        &mut self,
        v1: impl Into<ffi::Vector2>,
        v2: impl Into<ffi::Vector2>,
        v3: impl Into<ffi::Vector2>,
        c1: impl Into<ffi::Color>,
        c2: impl Into<ffi::Color>,
        c3: impl Into<ffi::Color>,
    ) {
        unsafe {
            ffi::ImageDrawTriangleEx(
                &mut self.0,
                v1.into(),
                v2.into(),
                v3.into(),
                c1.into(),
                c2.into(),
                c3.into(),
            )
        }
    }

    /// Draw triangle outline within an image
    pub fn draw_triangle_lines(
        &mut self,
        v1: impl Into<ffi::Vector2>,
        v2: impl Into<ffi::Vector2>,
        v3: impl Into<ffi::Vector2>,
        color: impl Into<ffi::Color>,
    ) {
        unsafe {
            ffi::ImageDrawTriangleLines(&mut self.0, v1.into(), v2.into(), v3.into(), color.into())
        }
    }

    /// Draw a triangle fan defined by points within an image (first vertex is the center)
    pub fn draw_triangle_fan(&mut self, points: Vec<Vector2>, color: impl Into<ffi::Color>) {
        unsafe {
            ffi::ImageDrawTriangleFan(
                &mut self.0,
                points
                    .iter()
                    .map(|f| f.into())
                    .collect::<Vec<ffi::Vector2>>()
                    .as_ptr() as *mut ffi::Vector2,
                points.len() as i32,
                color.into(),
            )
        }
    }

    /// Draw a triangle strip defined by points within an image
    pub fn draw_triangle_strip(&mut self, points: Vec<Vector2>, color: impl Into<ffi::Color>) {
        unsafe {
            ffi::ImageDrawTriangleStrip(
                &mut self.0,
                points
                    .iter()
                    .map(|f| f.into())
                    .collect::<Vec<ffi::Vector2>>()
                    .as_ptr() as *mut ffi::Vector2,
                points.len() as i32,
                color.into(),
            )
        }
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

    /// Draw rectangle within an image (Vector version)
    pub fn draw_rectangle_v(
        &mut self,
        position: impl Into<ffi::Vector2>,
        size: impl Into<ffi::Vector2>,
        color: impl Into<ffi::Color>,
    ) {
        unsafe {
            ffi::ImageDrawRectangleV(&mut self.0, position.into(), size.into(), color.into());
        }
    }

    /// Draw rectangle within an image (Rectangle version)
    pub fn draw_rectangle_rec(
        &mut self,
        rectangle: impl Into<ffi::Rectangle>,
        color: impl Into<ffi::Color>,
    ) {
        unsafe {
            ffi::ImageDrawRectangleRec(&mut self.0, rectangle.into(), color.into());
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
        text: &str,
        pos_x: i32,
        pos_y: i32,
        font_size: i32,
        color: impl Into<ffi::Color>,
    ) {
        let c_text = CString::new(text).unwrap();
        unsafe {
            ffi::ImageDrawText(
                &mut self.0,
                c_text.as_ptr(),
                pos_x,
                pos_y,
                font_size,
                color.into(),
            );
        }
    }

    /// Draws text (default font) within an image (destination).
    #[inline]
    pub fn draw_text_ex(
        &mut self,
        font: impl AsRef<ffi::Font>,
        text: &str,
        position: impl Into<ffi::Vector2>,
        font_size: f32,
        spacing: f32,
        color: impl Into<ffi::Color>,
    ) {
        let c_text = CString::new(text).unwrap();
        unsafe {
            ffi::ImageDrawTextEx(
                &mut self.0,
                *font.as_ref(),
                c_text.as_ptr(),
                position.into(),
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

    /// Export image to memory buffer.
    pub fn export_image_to_memory(&self, file_type: &str) -> Result<&[u8], Error> {
        if self.width == 0 {
            return Err(error!("Invalid image; width == 0"));
        }
        if self.height == 0 {
            return Err(error!("Invalid image; height == 0"));
        }
        if self.data == null_mut() {
            return Err(error!("Invalid image; data == null"));
        }

        let c_filetype = CString::new(file_type).unwrap();
        let data_size: &mut i32 = &mut 0;
        let data = unsafe { ffi::ExportImageToMemory(self.0, c_filetype.as_ptr(), data_size) };

        // The actual function returns null if the code for converting to a file type never goes off.
        if data == null_mut() {
            return Err(error!("Unsupported format."));
        }

        return Ok(unsafe { std::slice::from_raw_parts(data as *const u8, *data_size as usize) });
    }

    /// Apply custom square convolution kernel to image
    /// NOTE: The convolution kernel matrix is expected to be square
    pub fn kernel_convolution(&mut self, kernel: &[f32]) -> Result<(), Error> {
        if self.width == 0 {
            return Err(error!("Invalid image; width == 0"));
        }
        if self.height == 0 {
            return Err(error!("Invalid image; height == 0"));
        }
        if self.data == null_mut() {
            return Err(error!("Invalid image; data == null"));
        }

        let kernel_width = (kernel.len() as f32).sqrt() as i32;

        if (kernel_width * kernel_width) as usize != kernel.len() {
            return Err(error!("Convolution kernel must be square to be applied"));
        }

        unsafe { ffi::ImageKernelConvolution(&mut self.0, kernel.as_ptr(), kernel.len() as i32) }

        Ok(())
    }

    /// Generates a plain `color` Image.
    #[inline]
    pub fn gen_image_color(width: i32, height: i32, color: impl Into<ffi::Color>) -> Image {
        unsafe { Image(ffi::GenImageColor(width, height, color.into())) }
    }
    /// Generate image: perlin noise
    pub fn gen_image_perlin_noise(
        &self,
        width: i32,
        height: i32,
        offset_x: i32,
        offset_y: i32,
        scale: f32,
    ) -> Image {
        Image(unsafe { ffi::GenImagePerlinNoise(width, height, offset_x, offset_y, scale) })
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

    /// Generate images an image linear gradient.
    /// `direction` in expected to be degrees [0..360]. 0 results in a vertical gradient
    pub fn gen_image_gradient_linear(
        width: i32,
        height: i32,
        direction: i32,
        start: Color,
        end: Color,
    ) -> Image {
        unsafe {
            Image(ffi::GenImageGradientLinear(
                width,
                height,
                direction,
                start.into(),
                end.into(),
            ))
        }
    }
    /// Generate images an image with a square gradient
    /// For best results, `density` should be `0.0..1.0``
    pub fn gen_image_gradient_square(
        width: i32,
        height: i32,
        density: f32,
        start: Color,
        end: Color,
    ) -> Image {
        unsafe {
            Image(ffi::GenImageGradientSquare(
                width,
                height,
                density,
                start.into(),
                end.into(),
            ))
        }
    }

    // Generates an image with text
    pub fn gen_image_text(width: i32, height: i32, text: &str) -> Image {
        let c_str = CString::new(text).unwrap();
        unsafe { Image(ffi::GenImageText(width, height, c_str.as_ptr())) }
    }

    /// Generates an Image containing white noise.
    #[inline]
    pub fn gen_image_white_noise(width: i32, height: i32, factor: f32) -> Image {
        unsafe { Image(ffi::GenImageWhiteNoise(width, height, factor)) }
    }

    /// Generates an Image using a cellular algorithm. Bigger `tile_size` means bigger cells.
    #[inline]
    pub fn gen_image_cellular(width: i32, height: i32, tile_size: i32) -> Image {
        unsafe { Image(ffi::GenImageCellular(width, height, tile_size)) }
    }

    /// Get clipboard image.
    ///
    /// NOTE: Only avaliable on Windows. Do not use if you plan to compile to other platforms.
    #[cfg(target_os = "windows")]
    pub fn get_clipboard_image(&mut self) -> Result<Image, Error> {
        let i = unsafe { ffi::GetClipboardImage() };
        if i.data.is_null() {
            return Err(error!("Image data is null."));
        }
        Ok(Image(i))
    }

    /// Loads image from file into CPU memory (RAM).
    pub fn load_image(filename: &str) -> Result<Image, Error> {
        let c_filename = CString::new(filename).unwrap();
        let i = unsafe { ffi::LoadImage(c_filename.as_ptr()) };
        if i.data.is_null() {
            return Err(error!(
                "Image data is null. Either the file doesnt exist or the image type is unsupported."
            ));
        }
        Ok(Image(i))
    }

    /// Loads image from a given memory buffer
    /// The input data is expected to be in a supported file format such as png. Which formats are
    /// supported depend on the build flags used for the raylib (C) library.
    pub fn load_image_from_mem(filetype: &str, bytes: &[u8]) -> Result<Image, Error> {
        let c_filetype = CString::new(filetype).unwrap();
        let i = unsafe {
            ffi::LoadImageFromMemory(
                c_filetype.as_ptr(),
                bytes.as_ptr(),
                bytes.len().try_into().unwrap(),
            )
        };
        if i.data.is_null() {
            return Err(error!("Image data is null. Check provided buffer data"));
        };
        Ok(Image(i))
    }

    /// Load image sequence from file, with the number of frames loaded saved to frame_num.
    /// Image.data buffer includes all frames.
    /// All frames returned are in RGBA format.
    /// Frames delay data is discarded
    pub fn load_image_anim(filename: &str, frame_num: &mut i32) -> Self {
        let c_filename = CString::new(filename).unwrap();

        unsafe { Image(ffi::LoadImageAnim(c_filename.as_ptr(), frame_num)) }
    }

    /// Load image from memory buffer, with the number of frames loaded saved to frame_num.
    /// fileType refers to extension: i.e. ".png". File extension must be provided in lower-case
    pub fn load_image_anim_from_memory(filetype: &str, data: &[u8], frame_num: &mut i32) -> Self {
        let c_filetype = CString::new(filetype).unwrap();

        unsafe {
            Image(ffi::LoadImageAnimFromMemory(
                c_filetype.as_ptr(),
                data.as_ptr(),
                data.len() as i32,
                frame_num,
            ))
        }
    }

    /// Loads image from RAW file data.
    pub fn load_image_raw(
        filename: &str,
        width: i32,
        height: i32,
        format: i32,
        header_size: i32,
    ) -> Result<Image, Error> {
        let c_filename = CString::new(filename).unwrap();
        let i =
            unsafe { ffi::LoadImageRaw(c_filename.as_ptr(), width, height, format, header_size) };
        if i.data.is_null() {
            return Err(error!(
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

    pub fn is_image_valid(&self) -> bool {
        unsafe { ffi::IsImageValid(self.0) }
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
    fn update_texture(&mut self, pixels: &[u8]) -> Result<(), Error> {
        let expected_len = unsafe {
            get_pixel_data_size(
                self.as_ref().width,
                self.as_ref().height,
                std::mem::transmute::<i32, ffi::PixelFormat>(self.as_ref().format),
            ) as usize
        };
        if pixels.len() != expected_len {
            return Err(error!(std::borrow::Cow::Owned(format!(
                "update_texture: Data is wrong size. Expected {}, got {}",
                expected_len,
                pixels.len()
            ))));
        }
        unsafe {
            ffi::UpdateTexture(
                *self.as_mut(),
                pixels.as_ptr() as *const std::os::raw::c_void,
            );
        }

        Ok(())
    }

    /// Update GPU texture rectangle with new data
    fn update_texture_rec(
        &mut self,
        rec: impl Into<ffi::Rectangle>,
        pixels: &[u8],
    ) -> Result<(), Error> {
        let expected_len = unsafe {
            get_pixel_data_size(
                self.as_ref().width,
                self.as_ref().height,
                std::mem::transmute::<i32, ffi::PixelFormat>(self.as_ref().format),
            ) as usize
        };
        if pixels.len() != expected_len {
            return Err(error!(std::borrow::Cow::Owned(format!(
                "update_texture: Data is wrong size. Expected {}, got {}",
                expected_len,
                pixels.len()
            ))));
        }
        unsafe {
            ffi::UpdateTextureRec(
                *self.as_ref(),
                rec.into(),
                pixels.as_ptr() as *const std::os::raw::c_void,
            )
        }

        Ok(())
    }

    /// Gets pixel data from GPU texture and returns an `Image`.
    /// Fairly sure this would never fail. If it does wrap in result.
    #[inline]
    fn load_image(&self) -> Result<Image, Error> {
        let i = unsafe { ffi::LoadImageFromTexture(*self.as_ref()) };
        if i.data.is_null() {
            return Err(error!("Texture cannot be rendered to an image"));
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
    fn set_texture_filter(&self, _: &RaylibThread, filter_mode: crate::consts::TextureFilter) {
        unsafe {
            ffi::SetTextureFilter(*self.as_ref(), filter_mode as i32);
        }
    }

    /// Sets global texture wrapping mode.
    #[inline]
    fn set_texture_wrap(&self, _: &RaylibThread, wrap_mode: crate::consts::TextureWrap) {
        unsafe {
            ffi::SetTextureWrap(*self.as_ref(), wrap_mode as i32);
        }
    }

    fn is_texture_valid(&self) -> bool {
        unsafe { ffi::IsTextureValid(*self.as_ref()) }
    }
}

/// Gets pixel data size in bytes (image or texture).
#[inline]
pub fn get_pixel_data_size(width: i32, height: i32, format: ffi::PixelFormat) -> i32 {
    unsafe { ffi::GetPixelDataSize(width, height, format as i32) }
}

impl RaylibHandle {
    /// Loads texture from file into GPU memory (VRAM).
    pub fn load_texture(&mut self, _: &RaylibThread, filename: &str) -> Result<Texture2D, Error> {
        let c_filename = CString::new(filename).unwrap();
        let t = unsafe { ffi::LoadTexture(c_filename.as_ptr()) };
        if t.id == 0 {
            return Err(error!("failed to load the texture.", filename));
        }
        Ok(Texture2D(t))
    }

    /// Load cubemap from image, multiple image cubemap layouts supported
    pub fn load_texture_cubemap(
        &mut self,
        _: &RaylibThread,
        image: &Image,
        layout: crate::consts::CubemapLayout,
    ) -> Result<Texture2D, Error> {
        let t = unsafe { ffi::LoadTextureCubemap(image.0, layout as i32) };
        if t.id == 0 {
            return Err(error!("failed to load image as a texture cubemap."));
        }
        Ok(Texture2D(t))
    }

    /// Loads texture from image data.
    #[inline]
    pub fn load_texture_from_image(
        &mut self,
        _: &RaylibThread,
        image: &Image,
    ) -> Result<Texture2D, Error> {
        let t = unsafe { ffi::LoadTextureFromImage(image.0) };
        if t.id == 0 {
            return Err(error!("failed to load image as a texture."));
        }
        Ok(Texture2D(t))
    }

    /// Loads texture for rendering (framebuffer).
    pub fn load_render_texture(
        &mut self,
        _: &RaylibThread,
        width: u32,
        height: u32,
    ) -> Result<RenderTexture2D, Error> {
        let t = unsafe { ffi::LoadRenderTexture(width as i32, height as i32) };
        if t.id == 0 {
            return Err(error!("failed to create render texture."));
        }
        Ok(RenderTexture2D(t))
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
