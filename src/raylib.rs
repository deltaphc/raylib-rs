/* raylib-rs
   raylib.rs - Structs and raw FFI bindings
   Generated in part by bindgen

Copyright (c) 2018 Paul Clement (@deltaphc)

This software is provided "as-is", without any express or implied warranty. In no event will the authors be held liable for any damages arising from the use of this software.

Permission is granted to anyone to use this software for any purpose, including commercial applications, and to alter it and redistribute it freely, subject to the following restrictions:

  1. The origin of this software must not be misrepresented; you must not claim that you wrote the original software. If you use this software in a product, an acknowledgment in the product documentation would be appreciated but is not required.

  2. Altered source versions must be plainly marked as such, and must not be misrepresented as being the original software.

  3. This notice may not be removed or altered from any source distribution.
*/

extern crate libc;

pub use raymath::*;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Rectangle {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Image {
    pub data: *mut libc::c_void,
    pub width: i32,
    pub height: i32,
    pub mipmaps: i32,
    pub format: i32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Texture2D {
    pub id: u32,
    pub width: i32,
    pub height: i32,
    pub mipmaps: i32,
    pub format: i32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct RenderTexture2D {
    pub id: u32,
    pub texture: Texture2D,
    pub depth: Texture2D,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct CharInfo {
    pub value: i32,
    pub rec: Rectangle,
    pub offset_x: i32,
    pub offset_y: i32,
    pub advance_x: i32,
    pub data: *mut u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Font {
    pub texture: Texture2D,
    pub base_size: i32,
    pub chars_count: i32,
    pub chars: *mut CharInfo,
}

impl Font {
    /// Returns a new `Font` using provided `CharInfo` data and parameters.
    pub fn from_data(chars: &Vec<CharInfo>, base_size: i32, padding: i32, pack_method: i32) -> Font {
        unsafe {
            let mut f = ::std::mem::zeroed::<Font>();
            f.base_size = base_size;
            f.chars_count = chars.len() as i32;
            
            let data_size = f.chars_count as usize * ::std::mem::size_of::<CharInfo>();
            let ci_arr_ptr = libc::malloc(data_size); // raylib frees this data in UnloadFont
            ::std::ptr::copy(chars.as_ptr(), ci_arr_ptr as *mut CharInfo, chars.len());
            f.chars = ci_arr_ptr as *mut CharInfo;

            let atlas = GenImageFontAtlas(f.chars, f.base_size, f.chars_count, padding, pack_method);
            f.texture = LoadTextureFromImage(atlas);
            UnloadImage(atlas);
            f
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Camera3D {
    pub position: Vector3,
    pub target: Vector3,
    pub up: Vector3,
    pub fovy: f32,
    pub camtype: i32,
}

pub type Camera = Camera3D;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Camera2D {
    pub offset: Vector2,
    pub target: Vector2,
    pub rotation: f32,
    pub zoom: f32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct BoundingBox {
    pub min: Vector3,
    pub max: Vector3,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Mesh {
    pub vertex_count: i32,
    pub triangle_count: i32,
    pub vertices: *mut f32,
    pub texcoords: *mut f32,
    pub texcoords2: *mut f32,
    pub normals: *mut f32,
    pub tangents: *mut f32,
    pub colors: *mut u8,
    pub indices: *mut u16,
    pub vao_id: u32,
    pub vbo_id: [u32; 7],
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Shader {
    pub id: u32,
    pub locs: [i32; 32],
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct MaterialMap {
    pub texture: Texture2D,
    pub color: Color,
    pub value: f32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Material {
    pub shader: Shader,
    pub maps: [MaterialMap; 12],
    pub params: *mut f32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Model {
    pub mesh: Mesh,
    pub transform: Matrix,
    pub material: Material,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Ray {
    pub position: Vector3,
    pub direction: Vector3,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct RayHitInfo {
    pub hit: bool,
    pub distance: f32,
    pub position: Vector3,
    pub normal: Vector3,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Wave {
    pub sample_count: u32,
    pub sample_rate: u32,
    pub sample_size: u32,
    pub channels: u32,
    pub data: *mut libc::c_void,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Sound {
    pub audio_buffer: *mut libc::c_void,
    pub source: u32,
    pub buffer: u32,
    pub format: i32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct MusicData {
    _unused: [u8; 0],
}
pub type Music = *mut MusicData;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct AudioStream {
    pub sample_rate: u32,
    pub sample_size: u32,
    pub channels: u32,
    pub audio_buffer: *mut libc::c_void,
    pub format: i32,
    pub source: u32,
    pub buffers: [u32; 2],
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VrDeviceInfo {
    pub h_resolution: i32,
    pub v_resolution: i32,
    pub h_screen_size: f32,
    pub v_screen_size: f32,
    pub v_screen_center: f32,
    pub eye_to_screen_distance: f32,
    pub lens_separation_distance: f32,
    pub interpupillary_distance: f32,
    pub lens_distortion_values: [f32; 4],
    pub chroma_ab_correction: [f32; 4],
}

extern "C" {
    pub fn InitWindow(
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        title: *const ::std::os::raw::c_char,
    );

    pub fn CloseWindow();

    pub fn IsWindowReady() -> bool;

    pub fn WindowShouldClose() -> bool;

    pub fn IsWindowMinimized() -> bool;

    pub fn ToggleFullscreen();

    pub fn SetWindowIcon(image: Image);

    pub fn SetWindowTitle(title: *const ::std::os::raw::c_char);

    pub fn SetWindowPosition(x: ::std::os::raw::c_int, y: ::std::os::raw::c_int);

    pub fn SetWindowMonitor(monitor: ::std::os::raw::c_int);

    pub fn SetWindowMinSize(width: ::std::os::raw::c_int, height: ::std::os::raw::c_int);

    pub fn SetWindowSize(width: ::std::os::raw::c_int, height: ::std::os::raw::c_int);

    pub fn GetScreenWidth() -> ::std::os::raw::c_int;

    pub fn GetScreenHeight() -> ::std::os::raw::c_int;

    pub fn ShowCursor();

    pub fn HideCursor();

    pub fn IsCursorHidden() -> bool;

    pub fn EnableCursor();

    pub fn DisableCursor();

    pub fn ClearBackground(color: Color);

    pub fn BeginDrawing();

    pub fn EndDrawing();

    pub fn BeginMode2D(camera: Camera2D);

    pub fn EndMode2D();

    pub fn BeginMode3D(camera: Camera3D);

    pub fn EndMode3D();

    pub fn BeginTextureMode(target: RenderTexture2D);

    pub fn EndTextureMode();

    pub fn GetMouseRay(mousePosition: Vector2, camera: Camera3D) -> Ray;

    pub fn GetWorldToScreen(position: Vector3, camera: Camera3D) -> Vector2;

    pub fn GetCameraMatrix(camera: Camera3D) -> Matrix;

    pub fn SetTargetFPS(fps: ::std::os::raw::c_int);

    pub fn GetFPS() -> ::std::os::raw::c_int;

    pub fn GetFrameTime() -> f32;

    pub fn GetTime() -> f64;

    pub fn ColorToInt(color: Color) -> ::std::os::raw::c_int;

    pub fn ColorNormalize(color: Color) -> Vector4;

    pub fn ColorToHSV(color: Color) -> Vector3;

    pub fn GetColor(hexValue: ::std::os::raw::c_int) -> Color;

    pub fn Fade(color: Color, alpha: f32) -> Color;

    pub fn ShowLogo();

    pub fn SetConfigFlags(flags: ::std::os::raw::c_uchar);

    pub fn SetTraceLog(types: ::std::os::raw::c_uchar);

    // pub fn TraceLog(logType: ::std::os::raw::c_int, text: *const ::std::os::raw::c_char, ...);

    pub fn TakeScreenshot(fileName: *const ::std::os::raw::c_char);

    pub fn GetRandomValue(
        min: ::std::os::raw::c_int,
        max: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;

    pub fn IsFileExtension(
        fileName: *const ::std::os::raw::c_char,
        ext: *const ::std::os::raw::c_char,
    ) -> bool;

    pub fn GetExtension(fileName: *const ::std::os::raw::c_char) -> *const ::std::os::raw::c_char;

    pub fn GetFileName(filePath: *const ::std::os::raw::c_char) -> *const ::std::os::raw::c_char;

    pub fn GetDirectoryPath(
        fileName: *const ::std::os::raw::c_char,
    ) -> *const ::std::os::raw::c_char;

    pub fn GetWorkingDirectory() -> *const ::std::os::raw::c_char;

    pub fn ChangeDirectory(dir: *const ::std::os::raw::c_char) -> bool;

    pub fn IsFileDropped() -> bool;

    pub fn GetDroppedFiles(count: *mut ::std::os::raw::c_int) -> *mut *mut ::std::os::raw::c_char;

    pub fn ClearDroppedFiles();

    pub fn StorageSaveValue(position: ::std::os::raw::c_int, value: ::std::os::raw::c_int);

    pub fn StorageLoadValue(position: ::std::os::raw::c_int) -> ::std::os::raw::c_int;

    pub fn IsKeyPressed(key: ::std::os::raw::c_int) -> bool;

    pub fn IsKeyDown(key: ::std::os::raw::c_int) -> bool;

    pub fn IsKeyReleased(key: ::std::os::raw::c_int) -> bool;

    pub fn IsKeyUp(key: ::std::os::raw::c_int) -> bool;

    pub fn GetKeyPressed() -> ::std::os::raw::c_int;

    pub fn SetExitKey(key: ::std::os::raw::c_int);

    pub fn IsGamepadAvailable(gamepad: ::std::os::raw::c_int) -> bool;

    pub fn IsGamepadName(
        gamepad: ::std::os::raw::c_int,
        name: *const ::std::os::raw::c_char,
    ) -> bool;

    pub fn GetGamepadName(gamepad: ::std::os::raw::c_int) -> *const ::std::os::raw::c_char;

    pub fn IsGamepadButtonPressed(
        gamepad: ::std::os::raw::c_int,
        button: ::std::os::raw::c_int,
    ) -> bool;

    pub fn IsGamepadButtonDown(
        gamepad: ::std::os::raw::c_int,
        button: ::std::os::raw::c_int,
    ) -> bool;

    pub fn IsGamepadButtonReleased(
        gamepad: ::std::os::raw::c_int,
        button: ::std::os::raw::c_int,
    ) -> bool;

    pub fn IsGamepadButtonUp(
        gamepad: ::std::os::raw::c_int,
        button: ::std::os::raw::c_int,
    ) -> bool;

    pub fn GetGamepadButtonPressed() -> ::std::os::raw::c_int;

    pub fn GetGamepadAxisCount(gamepad: ::std::os::raw::c_int) -> ::std::os::raw::c_int;

    pub fn GetGamepadAxisMovement(
        gamepad: ::std::os::raw::c_int,
        axis: ::std::os::raw::c_int,
    ) -> f32;

    pub fn IsMouseButtonPressed(button: ::std::os::raw::c_int) -> bool;

    pub fn IsMouseButtonDown(button: ::std::os::raw::c_int) -> bool;

    pub fn IsMouseButtonReleased(button: ::std::os::raw::c_int) -> bool;

    pub fn IsMouseButtonUp(button: ::std::os::raw::c_int) -> bool;

    pub fn GetMouseX() -> ::std::os::raw::c_int;

    pub fn GetMouseY() -> ::std::os::raw::c_int;

    pub fn GetMousePosition() -> Vector2;

    pub fn SetMousePosition(position: Vector2);

    pub fn SetMouseScale(scale: f32);

    pub fn GetMouseWheelMove() -> ::std::os::raw::c_int;

    pub fn GetTouchX() -> ::std::os::raw::c_int;

    pub fn GetTouchY() -> ::std::os::raw::c_int;

    pub fn GetTouchPosition(index: ::std::os::raw::c_int) -> Vector2;

    pub fn SetGesturesEnabled(gestureFlags: ::std::os::raw::c_uint);

    pub fn IsGestureDetected(gesture: ::std::os::raw::c_int) -> bool;

    pub fn GetGestureDetected() -> ::std::os::raw::c_int;

    pub fn GetTouchPointsCount() -> ::std::os::raw::c_int;

    pub fn GetGestureHoldDuration() -> f32;

    pub fn GetGestureDragVector() -> Vector2;

    pub fn GetGestureDragAngle() -> f32;

    pub fn GetGesturePinchVector() -> Vector2;

    pub fn GetGesturePinchAngle() -> f32;

    pub fn SetCameraMode(camera: Camera3D, mode: ::std::os::raw::c_int);

    pub fn UpdateCamera(camera: *mut Camera3D);

    pub fn SetCameraPanControl(panKey: ::std::os::raw::c_int);

    pub fn SetCameraAltControl(altKey: ::std::os::raw::c_int);

    pub fn SetCameraSmoothZoomControl(szKey: ::std::os::raw::c_int);

    pub fn SetCameraMoveControls(
        frontKey: ::std::os::raw::c_int,
        backKey: ::std::os::raw::c_int,
        rightKey: ::std::os::raw::c_int,
        leftKey: ::std::os::raw::c_int,
        upKey: ::std::os::raw::c_int,
        downKey: ::std::os::raw::c_int,
    );

    pub fn DrawPixel(posX: ::std::os::raw::c_int, posY: ::std::os::raw::c_int, color: Color);

    pub fn DrawPixelV(position: Vector2, color: Color);

    pub fn DrawLine(
        startPosX: ::std::os::raw::c_int,
        startPosY: ::std::os::raw::c_int,
        endPosX: ::std::os::raw::c_int,
        endPosY: ::std::os::raw::c_int,
        color: Color,
    );

    pub fn DrawLineV(startPos: Vector2, endPos: Vector2, color: Color);

    pub fn DrawLineEx(startPos: Vector2, endPos: Vector2, thick: f32, color: Color);

    pub fn DrawLineBezier(startPos: Vector2, endPos: Vector2, thick: f32, color: Color);

    pub fn DrawCircle(
        centerX: ::std::os::raw::c_int,
        centerY: ::std::os::raw::c_int,
        radius: f32,
        color: Color,
    );

    pub fn DrawCircleGradient(
        centerX: ::std::os::raw::c_int,
        centerY: ::std::os::raw::c_int,
        radius: f32,
        color1: Color,
        color2: Color,
    );

    pub fn DrawCircleV(center: Vector2, radius: f32, color: Color);

    pub fn DrawCircleLines(
        centerX: ::std::os::raw::c_int,
        centerY: ::std::os::raw::c_int,
        radius: f32,
        color: Color,
    );

    pub fn DrawRectangle(
        posX: ::std::os::raw::c_int,
        posY: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        color: Color,
    );

    pub fn DrawRectangleV(position: Vector2, size: Vector2, color: Color);

    pub fn DrawRectangleRec(rec: Rectangle, color: Color);

    pub fn DrawRectanglePro(rec: Rectangle, origin: Vector2, rotation: f32, color: Color);

    pub fn DrawRectangleGradientV(
        posX: ::std::os::raw::c_int,
        posY: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        color1: Color,
        color2: Color,
    );

    pub fn DrawRectangleGradientH(
        posX: ::std::os::raw::c_int,
        posY: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        color1: Color,
        color2: Color,
    );

    pub fn DrawRectangleGradientEx(
        rec: Rectangle,
        col1: Color,
        col2: Color,
        col3: Color,
        col4: Color,
    );

    pub fn DrawRectangleLines(
        posX: ::std::os::raw::c_int,
        posY: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        color: Color,
    );

    pub fn DrawRectangleLinesEx(rec: Rectangle, lineThick: ::std::os::raw::c_int, color: Color);

    pub fn DrawTriangle(v1: Vector2, v2: Vector2, v3: Vector2, color: Color);

    pub fn DrawTriangleLines(v1: Vector2, v2: Vector2, v3: Vector2, color: Color);

    pub fn DrawPoly(
        center: Vector2,
        sides: ::std::os::raw::c_int,
        radius: f32,
        rotation: f32,
        color: Color,
    );

    pub fn DrawPolyEx(points: *mut Vector2, numPoints: ::std::os::raw::c_int, color: Color);

    pub fn DrawPolyExLines(points: *mut Vector2, numPoints: ::std::os::raw::c_int, color: Color);

    pub fn CheckCollisionRecs(rec1: Rectangle, rec2: Rectangle) -> bool;

    pub fn CheckCollisionCircles(
        center1: Vector2,
        radius1: f32,
        center2: Vector2,
        radius2: f32,
    ) -> bool;

    pub fn CheckCollisionCircleRec(center: Vector2, radius: f32, rec: Rectangle) -> bool;

    pub fn GetCollisionRec(rec1: Rectangle, rec2: Rectangle) -> Rectangle;

    pub fn CheckCollisionPointRec(point: Vector2, rec: Rectangle) -> bool;

    pub fn CheckCollisionPointCircle(point: Vector2, center: Vector2, radius: f32) -> bool;

    pub fn CheckCollisionPointTriangle(
        point: Vector2,
        p1: Vector2,
        p2: Vector2,
        p3: Vector2,
    ) -> bool;

    pub fn LoadImage(fileName: *const ::std::os::raw::c_char) -> Image;

    pub fn LoadImageEx(
        pixels: *mut Color,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
    ) -> Image;

    pub fn LoadImagePro(
        data: *mut ::std::os::raw::c_void,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        format: ::std::os::raw::c_int,
    ) -> Image;

    pub fn LoadImageRaw(
        fileName: *const ::std::os::raw::c_char,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        format: ::std::os::raw::c_int,
        headerSize: ::std::os::raw::c_int,
    ) -> Image;

    pub fn ExportImage(fileName: *const ::std::os::raw::c_char, image: Image);

    pub fn LoadTexture(fileName: *const ::std::os::raw::c_char) -> Texture2D;

    pub fn LoadTextureFromImage(image: Image) -> Texture2D;

    pub fn LoadRenderTexture(
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
    ) -> RenderTexture2D;

    pub fn UnloadImage(image: Image);

    pub fn UnloadTexture(texture: Texture2D);

    pub fn UnloadRenderTexture(target: RenderTexture2D);

    pub fn GetImageData(image: Image) -> *mut Color;

    pub fn GetImageDataNormalized(image: Image) -> *mut Vector4;

    pub fn GetPixelDataSize(
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        format: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;

    pub fn GetTextureData(texture: Texture2D) -> Image;

    pub fn UpdateTexture(texture: Texture2D, pixels: *const ::std::os::raw::c_void);

    pub fn ImageCopy(image: Image) -> Image;

    pub fn ImageToPOT(image: *mut Image, fillColor: Color);

    pub fn ImageFormat(image: *mut Image, newFormat: ::std::os::raw::c_int);

    pub fn ImageAlphaMask(image: *mut Image, alphaMask: Image);

    pub fn ImageAlphaClear(image: *mut Image, color: Color, threshold: f32);

    pub fn ImageAlphaCrop(image: *mut Image, threshold: f32);

    pub fn ImageAlphaPremultiply(image: *mut Image);

    pub fn ImageCrop(image: *mut Image, crop: Rectangle);

    pub fn ImageResize(
        image: *mut Image,
        newWidth: ::std::os::raw::c_int,
        newHeight: ::std::os::raw::c_int,
    );

    pub fn ImageResizeNN(
        image: *mut Image,
        newWidth: ::std::os::raw::c_int,
        newHeight: ::std::os::raw::c_int,
    );

    pub fn ImageResizeCanvas(
        image: *mut Image,
        newWidth: ::std::os::raw::c_int,
        newHeight: ::std::os::raw::c_int,
        offsetX: ::std::os::raw::c_int,
        offsetY: ::std::os::raw::c_int,
        color: Color,
    );

    pub fn ImageMipmaps(image: *mut Image);

    pub fn ImageDither(
        image: *mut Image,
        rBpp: ::std::os::raw::c_int,
        gBpp: ::std::os::raw::c_int,
        bBpp: ::std::os::raw::c_int,
        aBpp: ::std::os::raw::c_int,
    );

    pub fn ImageText(
        text: *const ::std::os::raw::c_char,
        fontSize: ::std::os::raw::c_int,
        color: Color,
    ) -> Image;

    pub fn ImageTextEx(
        font: Font,
        text: *const ::std::os::raw::c_char,
        fontSize: f32,
        spacing: f32,
        tint: Color,
    ) -> Image;

    pub fn ImageDraw(dst: *mut Image, src: Image, srcRec: Rectangle, dstRec: Rectangle);

    pub fn ImageDrawRectangle(dst: *mut Image, position: Vector2, rec: Rectangle, color: Color);

    pub fn ImageDrawText(
        dst: *mut Image,
        position: Vector2,
        text: *const ::std::os::raw::c_char,
        fontSize: ::std::os::raw::c_int,
        color: Color,
    );

    pub fn ImageDrawTextEx(
        dst: *mut Image,
        position: Vector2,
        font: Font,
        text: *const ::std::os::raw::c_char,
        fontSize: f32,
        spacing: f32,
        color: Color,
    );

    pub fn ImageFlipVertical(image: *mut Image);

    pub fn ImageFlipHorizontal(image: *mut Image);

    pub fn ImageRotateCW(image: *mut Image);

    pub fn ImageRotateCCW(image: *mut Image);

    pub fn ImageColorTint(image: *mut Image, color: Color);

    pub fn ImageColorInvert(image: *mut Image);

    pub fn ImageColorGrayscale(image: *mut Image);

    pub fn ImageColorContrast(image: *mut Image, contrast: f32);

    pub fn ImageColorBrightness(image: *mut Image, brightness: ::std::os::raw::c_int);

    pub fn ImageColorReplace(image: *mut Image, color: Color, replace: Color);

    pub fn GenImageColor(
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        color: Color,
    ) -> Image;

    pub fn GenImageGradientV(
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        top: Color,
        bottom: Color,
    ) -> Image;

    pub fn GenImageGradientH(
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        left: Color,
        right: Color,
    ) -> Image;

    pub fn GenImageGradientRadial(
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        density: f32,
        inner: Color,
        outer: Color,
    ) -> Image;

    pub fn GenImageChecked(
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        checksX: ::std::os::raw::c_int,
        checksY: ::std::os::raw::c_int,
        col1: Color,
        col2: Color,
    ) -> Image;

    pub fn GenImageWhiteNoise(
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        factor: f32,
    ) -> Image;

    pub fn GenImagePerlinNoise(
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        offsetX: ::std::os::raw::c_int,
        offsetY: ::std::os::raw::c_int,
        scale: f32,
    ) -> Image;

    pub fn GenImageCellular(
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        tileSize: ::std::os::raw::c_int,
    ) -> Image;

    pub fn GenTextureMipmaps(texture: *mut Texture2D);

    pub fn SetTextureFilter(texture: Texture2D, filterMode: ::std::os::raw::c_int);

    pub fn SetTextureWrap(texture: Texture2D, wrapMode: ::std::os::raw::c_int);

    pub fn DrawTexture(
        texture: Texture2D,
        posX: ::std::os::raw::c_int,
        posY: ::std::os::raw::c_int,
        tint: Color,
    );

    pub fn DrawTextureV(texture: Texture2D, position: Vector2, tint: Color);

    pub fn DrawTextureEx(
        texture: Texture2D,
        position: Vector2,
        rotation: f32,
        scale: f32,
        tint: Color,
    );

    pub fn DrawTextureRec(texture: Texture2D, sourceRec: Rectangle, position: Vector2, tint: Color);

    pub fn DrawTexturePro(
        texture: Texture2D,
        sourceRec: Rectangle,
        destRec: Rectangle,
        origin: Vector2,
        rotation: f32,
        tint: Color,
    );

    pub fn GetFontDefault() -> Font;

    pub fn LoadFont(fileName: *const ::std::os::raw::c_char) -> Font;

    pub fn LoadFontEx(
        fileName: *const ::std::os::raw::c_char,
        fontSize: ::std::os::raw::c_int,
        charsCount: ::std::os::raw::c_int,
        fontChars: *mut ::std::os::raw::c_int,
    ) -> Font;

    pub fn LoadFontData(
        fileName: *const ::std::os::raw::c_char,
        fontSize: ::std::os::raw::c_int,
        fontChars: *mut ::std::os::raw::c_int,
        charsCount: ::std::os::raw::c_int,
        sdf: bool,
    ) -> *mut CharInfo;

    pub fn GenImageFontAtlas(
        chars: *mut CharInfo,
        fontSize: ::std::os::raw::c_int,
        charsCount: ::std::os::raw::c_int,
        padding: ::std::os::raw::c_int,
        packMethod: ::std::os::raw::c_int,
    ) -> Image;

    pub fn UnloadFont(font: Font);

    pub fn DrawFPS(posX: ::std::os::raw::c_int, posY: ::std::os::raw::c_int);

    pub fn DrawText(
        text: *const ::std::os::raw::c_char,
        posX: ::std::os::raw::c_int,
        posY: ::std::os::raw::c_int,
        fontSize: ::std::os::raw::c_int,
        color: Color,
    );

    pub fn DrawTextEx(
        font: Font,
        text: *const ::std::os::raw::c_char,
        position: Vector2,
        fontSize: f32,
        spacing: f32,
        tint: Color,
    );

    pub fn MeasureText(
        text: *const ::std::os::raw::c_char,
        fontSize: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;

    pub fn MeasureTextEx(
        font: Font,
        text: *const ::std::os::raw::c_char,
        fontSize: f32,
        spacing: f32,
    ) -> Vector2;

    // pub fn FormatText(text: *const ::std::os::raw::c_char, ...) -> *const ::std::os::raw::c_char;

    /* pub fn SubText(
        text: *const ::std::os::raw::c_char,
        position: ::std::os::raw::c_int,
        length: ::std::os::raw::c_int,
    ) -> *const ::std::os::raw::c_char; */

    pub fn GetGlyphIndex(font: Font, character: ::std::os::raw::c_int) -> ::std::os::raw::c_int;

    pub fn DrawLine3D(startPos: Vector3, endPos: Vector3, color: Color);

    pub fn DrawCircle3D(
        center: Vector3,
        radius: f32,
        rotationAxis: Vector3,
        rotationAngle: f32,
        color: Color,
    );

    pub fn DrawCube(position: Vector3, width: f32, height: f32, length: f32, color: Color);

    pub fn DrawCubeV(position: Vector3, size: Vector3, color: Color);

    pub fn DrawCubeWires(position: Vector3, width: f32, height: f32, length: f32, color: Color);

    pub fn DrawCubeTexture(
        texture: Texture2D,
        position: Vector3,
        width: f32,
        height: f32,
        length: f32,
        color: Color,
    );

    pub fn DrawSphere(centerPos: Vector3, radius: f32, color: Color);

    pub fn DrawSphereEx(
        centerPos: Vector3,
        radius: f32,
        rings: ::std::os::raw::c_int,
        slices: ::std::os::raw::c_int,
        color: Color,
    );

    pub fn DrawSphereWires(
        centerPos: Vector3,
        radius: f32,
        rings: ::std::os::raw::c_int,
        slices: ::std::os::raw::c_int,
        color: Color,
    );

    pub fn DrawCylinder(
        position: Vector3,
        radiusTop: f32,
        radiusBottom: f32,
        height: f32,
        slices: ::std::os::raw::c_int,
        color: Color,
    );

    pub fn DrawCylinderWires(
        position: Vector3,
        radiusTop: f32,
        radiusBottom: f32,
        height: f32,
        slices: ::std::os::raw::c_int,
        color: Color,
    );

    pub fn DrawPlane(centerPos: Vector3, size: Vector2, color: Color);

    pub fn DrawRay(ray: Ray, color: Color);

    pub fn DrawGrid(slices: ::std::os::raw::c_int, spacing: f32);

    pub fn DrawGizmo(position: Vector3);

    pub fn LoadModel(fileName: *const ::std::os::raw::c_char) -> Model;

    pub fn LoadModelFromMesh(mesh: Mesh) -> Model;

    pub fn UnloadModel(model: Model);

    pub fn LoadMesh(fileName: *const ::std::os::raw::c_char) -> Mesh;

    pub fn UnloadMesh(mesh: *mut Mesh);

    pub fn ExportMesh(fileName: *const ::std::os::raw::c_char, mesh: Mesh);

    pub fn MeshBoundingBox(mesh: Mesh) -> BoundingBox;

    pub fn MeshTangents(mesh: *mut Mesh);

    pub fn MeshBinormals(mesh: *mut Mesh);

    pub fn GenMeshPlane(
        width: f32,
        length: f32,
        resX: ::std::os::raw::c_int,
        resZ: ::std::os::raw::c_int,
    ) -> Mesh;

    pub fn GenMeshCube(width: f32, height: f32, length: f32) -> Mesh;

    pub fn GenMeshSphere(
        radius: f32,
        rings: ::std::os::raw::c_int,
        slices: ::std::os::raw::c_int,
    ) -> Mesh;

    pub fn GenMeshHemiSphere(
        radius: f32,
        rings: ::std::os::raw::c_int,
        slices: ::std::os::raw::c_int,
    ) -> Mesh;

    pub fn GenMeshCylinder(radius: f32, height: f32, slices: ::std::os::raw::c_int) -> Mesh;

    pub fn GenMeshTorus(
        radius: f32,
        size: f32,
        radSeg: ::std::os::raw::c_int,
        sides: ::std::os::raw::c_int,
    ) -> Mesh;

    pub fn GenMeshKnot(
        radius: f32,
        size: f32,
        radSeg: ::std::os::raw::c_int,
        sides: ::std::os::raw::c_int,
    ) -> Mesh;

    pub fn GenMeshHeightmap(heightmap: Image, size: Vector3) -> Mesh;

    pub fn GenMeshCubicmap(cubicmap: Image, cubeSize: Vector3) -> Mesh;

    pub fn LoadMaterial(fileName: *const ::std::os::raw::c_char) -> Material;

    pub fn LoadMaterialDefault() -> Material;

    pub fn UnloadMaterial(material: Material);

    pub fn DrawModel(model: Model, position: Vector3, scale: f32, tint: Color);

    pub fn DrawModelEx(
        model: Model,
        position: Vector3,
        rotationAxis: Vector3,
        rotationAngle: f32,
        scale: Vector3,
        tint: Color,
    );

    pub fn DrawModelWires(model: Model, position: Vector3, scale: f32, tint: Color);

    pub fn DrawModelWiresEx(
        model: Model,
        position: Vector3,
        rotationAxis: Vector3,
        rotationAngle: f32,
        scale: Vector3,
        tint: Color,
    );

    pub fn DrawBoundingBox(box_: BoundingBox, color: Color);

    pub fn DrawBillboard(
        camera: Camera3D,
        texture: Texture2D,
        center: Vector3,
        size: f32,
        tint: Color,
    );

    pub fn DrawBillboardRec(
        camera: Camera3D,
        texture: Texture2D,
        sourceRec: Rectangle,
        center: Vector3,
        size: f32,
        tint: Color,
    );

    pub fn CheckCollisionSpheres(
        centerA: Vector3,
        radiusA: f32,
        centerB: Vector3,
        radiusB: f32,
    ) -> bool;

    pub fn CheckCollisionBoxes(box1: BoundingBox, box2: BoundingBox) -> bool;

    pub fn CheckCollisionBoxSphere(
        box_: BoundingBox,
        centerSphere: Vector3,
        radiusSphere: f32,
    ) -> bool;

    pub fn CheckCollisionRaySphere(ray: Ray, spherePosition: Vector3, sphereRadius: f32) -> bool;

    pub fn CheckCollisionRaySphereEx(
        ray: Ray,
        spherePosition: Vector3,
        sphereRadius: f32,
        collisionPoint: *mut Vector3,
    ) -> bool;

    pub fn CheckCollisionRayBox(ray: Ray, box_: BoundingBox) -> bool;

    pub fn GetCollisionRayModel(ray: Ray, model: *mut Model) -> RayHitInfo;

    pub fn GetCollisionRayTriangle(ray: Ray, p1: Vector3, p2: Vector3, p3: Vector3) -> RayHitInfo;

    pub fn GetCollisionRayGround(ray: Ray, groundHeight: f32) -> RayHitInfo;

    pub fn LoadText(fileName: *const ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;

    pub fn LoadShader(
        vsFileName: *const ::std::os::raw::c_char,
        fsFileName: *const ::std::os::raw::c_char,
    ) -> Shader;

    pub fn LoadShaderCode(
        vsCode: *mut ::std::os::raw::c_char,
        fsCode: *mut ::std::os::raw::c_char,
    ) -> Shader;

    pub fn UnloadShader(shader: Shader);

    pub fn GetShaderDefault() -> Shader;

    pub fn GetTextureDefault() -> Texture2D;

    pub fn GetShaderLocation(
        shader: Shader,
        uniformName: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;

    pub fn SetShaderValue(
        shader: Shader,
        uniformLoc: ::std::os::raw::c_int,
        value: *const f32,
        size: ::std::os::raw::c_int,
    );

    pub fn SetShaderValuei(
        shader: Shader,
        uniformLoc: ::std::os::raw::c_int,
        value: *const ::std::os::raw::c_int,
        size: ::std::os::raw::c_int,
    );

    pub fn SetShaderValueMatrix(shader: Shader, uniformLoc: ::std::os::raw::c_int, mat: Matrix);

    pub fn SetMatrixProjection(proj: Matrix);

    pub fn SetMatrixModelview(view: Matrix);

    pub fn GetMatrixModelview() -> Matrix;

    pub fn GenTextureCubemap(
        shader: Shader,
        skyHDR: Texture2D,
        size: ::std::os::raw::c_int,
    ) -> Texture2D;

    pub fn GenTextureIrradiance(
        shader: Shader,
        cubemap: Texture2D,
        size: ::std::os::raw::c_int,
    ) -> Texture2D;

    pub fn GenTexturePrefilter(
        shader: Shader,
        cubemap: Texture2D,
        size: ::std::os::raw::c_int,
    ) -> Texture2D;

    pub fn GenTextureBRDF(
        shader: Shader,
        cubemap: Texture2D,
        size: ::std::os::raw::c_int,
    ) -> Texture2D;

    pub fn BeginShaderMode(shader: Shader);

    pub fn EndShaderMode();

    pub fn BeginBlendMode(mode: ::std::os::raw::c_int);

    pub fn EndBlendMode();

    pub fn GetVrDeviceInfo(vrDeviceType: ::std::os::raw::c_int) -> VrDeviceInfo;

    pub fn InitVrSimulator(info: VrDeviceInfo);

    pub fn CloseVrSimulator();

    pub fn IsVrSimulatorReady() -> bool;

    pub fn SetVrDistortionShader(shader: Shader);

    pub fn UpdateVrTracking(camera: *mut Camera3D);

    pub fn ToggleVrMode();

    pub fn BeginVrDrawing();

    pub fn EndVrDrawing();

    pub fn InitAudioDevice();

    pub fn CloseAudioDevice();

    pub fn IsAudioDeviceReady() -> bool;

    pub fn SetMasterVolume(volume: f32);

    pub fn LoadWave(fileName: *const ::std::os::raw::c_char) -> Wave;

    pub fn LoadWaveEx(
        data: *mut ::std::os::raw::c_void,
        sampleCount: ::std::os::raw::c_int,
        sampleRate: ::std::os::raw::c_int,
        sampleSize: ::std::os::raw::c_int,
        channels: ::std::os::raw::c_int,
    ) -> Wave;

    pub fn LoadSound(fileName: *const ::std::os::raw::c_char) -> Sound;

    pub fn LoadSoundFromWave(wave: Wave) -> Sound;

    pub fn UpdateSound(
        sound: Sound,
        data: *const ::std::os::raw::c_void,
        samplesCount: ::std::os::raw::c_int,
    );

    pub fn UnloadWave(wave: Wave);

    pub fn UnloadSound(sound: Sound);

    pub fn PlaySound(sound: Sound);

    pub fn PauseSound(sound: Sound);

    pub fn ResumeSound(sound: Sound);

    pub fn StopSound(sound: Sound);

    pub fn IsSoundPlaying(sound: Sound) -> bool;

    pub fn SetSoundVolume(sound: Sound, volume: f32);

    pub fn SetSoundPitch(sound: Sound, pitch: f32);

    pub fn WaveFormat(
        wave: *mut Wave,
        sampleRate: ::std::os::raw::c_int,
        sampleSize: ::std::os::raw::c_int,
        channels: ::std::os::raw::c_int,
    );

    pub fn WaveCopy(wave: Wave) -> Wave;

    pub fn WaveCrop(
        wave: *mut Wave,
        initSample: ::std::os::raw::c_int,
        finalSample: ::std::os::raw::c_int,
    );

    pub fn GetWaveData(wave: Wave) -> *mut f32;

    pub fn LoadMusicStream(fileName: *const ::std::os::raw::c_char) -> Music;

    pub fn UnloadMusicStream(music: Music);

    pub fn PlayMusicStream(music: Music);

    pub fn UpdateMusicStream(music: Music);

    pub fn StopMusicStream(music: Music);

    pub fn PauseMusicStream(music: Music);

    pub fn ResumeMusicStream(music: Music);

    pub fn IsMusicPlaying(music: Music) -> bool;

    pub fn SetMusicVolume(music: Music, volume: f32);

    pub fn SetMusicPitch(music: Music, pitch: f32);

    pub fn SetMusicLoopCount(music: Music, count: ::std::os::raw::c_int);

    pub fn GetMusicTimeLength(music: Music) -> f32;

    pub fn GetMusicTimePlayed(music: Music) -> f32;

    pub fn InitAudioStream(
        sampleRate: ::std::os::raw::c_uint,
        sampleSize: ::std::os::raw::c_uint,
        channels: ::std::os::raw::c_uint,
    ) -> AudioStream;

    pub fn UpdateAudioStream(
        stream: AudioStream,
        data: *const ::std::os::raw::c_void,
        samplesCount: ::std::os::raw::c_int,
    );

    pub fn CloseAudioStream(stream: AudioStream);

    pub fn IsAudioBufferProcessed(stream: AudioStream) -> bool;

    pub fn PlayAudioStream(stream: AudioStream);

    pub fn PauseAudioStream(stream: AudioStream);

    pub fn ResumeAudioStream(stream: AudioStream);

    pub fn IsAudioStreamPlaying(stream: AudioStream) -> bool;

    pub fn StopAudioStream(stream: AudioStream);

    pub fn SetAudioStreamVolume(stream: AudioStream, volume: f32);

    pub fn SetAudioStreamPitch(stream: AudioStream, pitch: f32);
}

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]
    use raylib::*;

    #[test]
    fn bindgen_test_layout_Vector2() {
        assert_eq!(
            ::std::mem::size_of::<Vector2>(),
            8usize,
            concat!("Size of: ", stringify!(Vector2))
        );
        assert_eq!(
            ::std::mem::align_of::<Vector2>(),
            4usize,
            concat!("Alignment of ", stringify!(Vector2))
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<Vector2>())).x as *const _ as usize },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(Vector2),
                "::",
                stringify!(x)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<Vector2>())).y as *const _ as usize },
            4usize,
            concat!(
                "Offset of field: ",
                stringify!(Vector2),
                "::",
                stringify!(y)
            )
        );
    }

    #[test]
    fn bindgen_test_layout_Vector3() {
        assert_eq!(
            ::std::mem::size_of::<Vector3>(),
            12usize,
            concat!("Size of: ", stringify!(Vector3))
        );
        assert_eq!(
            ::std::mem::align_of::<Vector3>(),
            4usize,
            concat!("Alignment of ", stringify!(Vector3))
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<Vector3>())).x as *const _ as usize },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(Vector3),
                "::",
                stringify!(x)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<Vector3>())).y as *const _ as usize },
            4usize,
            concat!(
                "Offset of field: ",
                stringify!(Vector3),
                "::",
                stringify!(y)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<Vector3>())).z as *const _ as usize },
            8usize,
            concat!(
                "Offset of field: ",
                stringify!(Vector3),
                "::",
                stringify!(z)
            )
        );
    }

    #[test]
    fn bindgen_test_layout_Vector4() {
        assert_eq!(
            ::std::mem::size_of::<Vector4>(),
            16usize,
            concat!("Size of: ", stringify!(Vector4))
        );
        assert_eq!(
            ::std::mem::align_of::<Vector4>(),
            4usize,
            concat!("Alignment of ", stringify!(Vector4))
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<Vector4>())).x as *const _ as usize },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(Vector4),
                "::",
                stringify!(x)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<Vector4>())).y as *const _ as usize },
            4usize,
            concat!(
                "Offset of field: ",
                stringify!(Vector4),
                "::",
                stringify!(y)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<Vector4>())).z as *const _ as usize },
            8usize,
            concat!(
                "Offset of field: ",
                stringify!(Vector4),
                "::",
                stringify!(z)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<Vector4>())).w as *const _ as usize },
            12usize,
            concat!(
                "Offset of field: ",
                stringify!(Vector4),
                "::",
                stringify!(w)
            )
        );
    }

    #[test]
    fn bindgen_test_layout_Matrix() {
        assert_eq!(
            ::std::mem::size_of::<Matrix>(),
            64usize,
            concat!("Size of: ", stringify!(Matrix))
        );
        assert_eq!(
            ::std::mem::align_of::<Matrix>(),
            4usize,
            concat!("Alignment of ", stringify!(Matrix))
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<Matrix>())).m0 as *const _ as usize },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(Matrix),
                "::",
                stringify!(m0)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<Matrix>())).m4 as *const _ as usize },
            4usize,
            concat!(
                "Offset of field: ",
                stringify!(Matrix),
                "::",
                stringify!(m4)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<Matrix>())).m8 as *const _ as usize },
            8usize,
            concat!(
                "Offset of field: ",
                stringify!(Matrix),
                "::",
                stringify!(m8)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<Matrix>())).m12 as *const _ as usize },
            12usize,
            concat!(
                "Offset of field: ",
                stringify!(Matrix),
                "::",
                stringify!(m12)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<Matrix>())).m1 as *const _ as usize },
            16usize,
            concat!(
                "Offset of field: ",
                stringify!(Matrix),
                "::",
                stringify!(m1)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<Matrix>())).m5 as *const _ as usize },
            20usize,
            concat!(
                "Offset of field: ",
                stringify!(Matrix),
                "::",
                stringify!(m5)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<Matrix>())).m9 as *const _ as usize },
            24usize,
            concat!(
                "Offset of field: ",
                stringify!(Matrix),
                "::",
                stringify!(m9)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<Matrix>())).m13 as *const _ as usize },
            28usize,
            concat!(
                "Offset of field: ",
                stringify!(Matrix),
                "::",
                stringify!(m13)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<Matrix>())).m2 as *const _ as usize },
            32usize,
            concat!(
                "Offset of field: ",
                stringify!(Matrix),
                "::",
                stringify!(m2)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<Matrix>())).m6 as *const _ as usize },
            36usize,
            concat!(
                "Offset of field: ",
                stringify!(Matrix),
                "::",
                stringify!(m6)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<Matrix>())).m10 as *const _ as usize },
            40usize,
            concat!(
                "Offset of field: ",
                stringify!(Matrix),
                "::",
                stringify!(m10)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<Matrix>())).m14 as *const _ as usize },
            44usize,
            concat!(
                "Offset of field: ",
                stringify!(Matrix),
                "::",
                stringify!(m14)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<Matrix>())).m3 as *const _ as usize },
            48usize,
            concat!(
                "Offset of field: ",
                stringify!(Matrix),
                "::",
                stringify!(m3)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<Matrix>())).m7 as *const _ as usize },
            52usize,
            concat!(
                "Offset of field: ",
                stringify!(Matrix),
                "::",
                stringify!(m7)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<Matrix>())).m11 as *const _ as usize },
            56usize,
            concat!(
                "Offset of field: ",
                stringify!(Matrix),
                "::",
                stringify!(m11)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<Matrix>())).m15 as *const _ as usize },
            60usize,
            concat!(
                "Offset of field: ",
                stringify!(Matrix),
                "::",
                stringify!(m15)
            )
        );
    }

    #[test]
    fn bindgen_test_layout_Color() {
        assert_eq!(
            ::std::mem::size_of::<Color>(),
            4usize,
            concat!("Size of: ", stringify!(Color))
        );
        assert_eq!(
            ::std::mem::align_of::<Color>(),
            1usize,
            concat!("Alignment of ", stringify!(Color))
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<Color>())).r as *const _ as usize },
            0usize,
            concat!("Offset of field: ", stringify!(Color), "::", stringify!(r))
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<Color>())).g as *const _ as usize },
            1usize,
            concat!("Offset of field: ", stringify!(Color), "::", stringify!(g))
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<Color>())).b as *const _ as usize },
            2usize,
            concat!("Offset of field: ", stringify!(Color), "::", stringify!(b))
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<Color>())).a as *const _ as usize },
            3usize,
            concat!("Offset of field: ", stringify!(Color), "::", stringify!(a))
        );
    }

    #[test]
    fn bindgen_test_layout_Rectangle() {
        assert_eq!(
            ::std::mem::size_of::<Rectangle>(),
            16usize,
            concat!("Size of: ", stringify!(Rectangle))
        );
        assert_eq!(
            ::std::mem::align_of::<Rectangle>(),
            4usize,
            concat!("Alignment of ", stringify!(Rectangle))
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<Rectangle>())).x as *const _ as usize },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(Rectangle),
                "::",
                stringify!(x)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<Rectangle>())).y as *const _ as usize },
            4usize,
            concat!(
                "Offset of field: ",
                stringify!(Rectangle),
                "::",
                stringify!(y)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<Rectangle>())).width as *const _ as usize },
            8usize,
            concat!(
                "Offset of field: ",
                stringify!(Rectangle),
                "::",
                stringify!(width)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<Rectangle>())).height as *const _ as usize },
            12usize,
            concat!(
                "Offset of field: ",
                stringify!(Rectangle),
                "::",
                stringify!(height)
            )
        );
    }

    #[test]
    fn bindgen_test_layout_Image() {
        assert_eq!(
            ::std::mem::size_of::<Image>(),
            24usize,
            concat!("Size of: ", stringify!(Image))
        );
        assert_eq!(
            ::std::mem::align_of::<Image>(),
            8usize,
            concat!("Alignment of ", stringify!(Image))
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<Image>())).data as *const _ as usize },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(Image),
                "::",
                stringify!(data)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<Image>())).width as *const _ as usize },
            8usize,
            concat!(
                "Offset of field: ",
                stringify!(Image),
                "::",
                stringify!(width)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<Image>())).height as *const _ as usize },
            12usize,
            concat!(
                "Offset of field: ",
                stringify!(Image),
                "::",
                stringify!(height)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<Image>())).mipmaps as *const _ as usize },
            16usize,
            concat!(
                "Offset of field: ",
                stringify!(Image),
                "::",
                stringify!(mipmaps)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<Image>())).format as *const _ as usize },
            20usize,
            concat!(
                "Offset of field: ",
                stringify!(Image),
                "::",
                stringify!(format)
            )
        );
    }

    #[test]
    fn bindgen_test_layout_Texture2D() {
        assert_eq!(
            ::std::mem::size_of::<Texture2D>(),
            20usize,
            concat!("Size of: ", stringify!(Texture2D))
        );
        assert_eq!(
            ::std::mem::align_of::<Texture2D>(),
            4usize,
            concat!("Alignment of ", stringify!(Texture2D))
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<Texture2D>())).id as *const _ as usize },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(Texture2D),
                "::",
                stringify!(id)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<Texture2D>())).width as *const _ as usize },
            4usize,
            concat!(
                "Offset of field: ",
                stringify!(Texture2D),
                "::",
                stringify!(width)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<Texture2D>())).height as *const _ as usize },
            8usize,
            concat!(
                "Offset of field: ",
                stringify!(Texture2D),
                "::",
                stringify!(height)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<Texture2D>())).mipmaps as *const _ as usize },
            12usize,
            concat!(
                "Offset of field: ",
                stringify!(Texture2D),
                "::",
                stringify!(mipmaps)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<Texture2D>())).format as *const _ as usize },
            16usize,
            concat!(
                "Offset of field: ",
                stringify!(Texture2D),
                "::",
                stringify!(format)
            )
        );
    }

    #[test]
    fn bindgen_test_layout_RenderTexture2D() {
        assert_eq!(
            ::std::mem::size_of::<RenderTexture2D>(),
            44usize,
            concat!("Size of: ", stringify!(RenderTexture2D))
        );
        assert_eq!(
            ::std::mem::align_of::<RenderTexture2D>(),
            4usize,
            concat!("Alignment of ", stringify!(RenderTexture2D))
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<RenderTexture2D>())).id as *const _ as usize },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(RenderTexture2D),
                "::",
                stringify!(id)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<RenderTexture2D>())).texture as *const _ as usize },
            4usize,
            concat!(
                "Offset of field: ",
                stringify!(RenderTexture2D),
                "::",
                stringify!(texture)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<RenderTexture2D>())).depth as *const _ as usize },
            24usize,
            concat!(
                "Offset of field: ",
                stringify!(RenderTexture2D),
                "::",
                stringify!(depth)
            )
        );
    }

    #[test]
    fn bindgen_test_layout_CharInfo() {
        assert_eq!(
            ::std::mem::size_of::<CharInfo>(),
            40usize,
            concat!("Size of: ", stringify!(CharInfo))
        );
        assert_eq!(
            ::std::mem::align_of::<CharInfo>(),
            8usize,
            concat!("Alignment of ", stringify!(CharInfo))
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<CharInfo>())).value as *const _ as usize },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(CharInfo),
                "::",
                stringify!(value)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<CharInfo>())).rec as *const _ as usize },
            4usize,
            concat!(
                "Offset of field: ",
                stringify!(CharInfo),
                "::",
                stringify!(rec)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<CharInfo>())).offset_x as *const _ as usize },
            20usize,
            concat!(
                "Offset of field: ",
                stringify!(CharInfo),
                "::",
                stringify!(offsetX)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<CharInfo>())).offset_y as *const _ as usize },
            24usize,
            concat!(
                "Offset of field: ",
                stringify!(CharInfo),
                "::",
                stringify!(offsetY)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<CharInfo>())).advance_x as *const _ as usize },
            28usize,
            concat!(
                "Offset of field: ",
                stringify!(CharInfo),
                "::",
                stringify!(advanceX)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<CharInfo>())).data as *const _ as usize },
            32usize,
            concat!(
                "Offset of field: ",
                stringify!(CharInfo),
                "::",
                stringify!(data)
            )
        );
    }

    #[test]
    fn bindgen_test_layout_Font() {
        assert_eq!(
            ::std::mem::size_of::<Font>(),
            40usize,
            concat!("Size of: ", stringify!(Font))
        );
        assert_eq!(
            ::std::mem::align_of::<Font>(),
            8usize,
            concat!("Alignment of ", stringify!(Font))
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<Font>())).texture as *const _ as usize },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(Font),
                "::",
                stringify!(texture)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<Font>())).base_size as *const _ as usize },
            20usize,
            concat!(
                "Offset of field: ",
                stringify!(Font),
                "::",
                stringify!(baseSize)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<Font>())).chars_count as *const _ as usize },
            24usize,
            concat!(
                "Offset of field: ",
                stringify!(Font),
                "::",
                stringify!(charsCount)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<Font>())).chars as *const _ as usize },
            32usize,
            concat!(
                "Offset of field: ",
                stringify!(Font),
                "::",
                stringify!(chars)
            )
        );
    }

    #[test]
    fn bindgen_test_layout_Camera3D() {
        assert_eq!(
            ::std::mem::size_of::<Camera3D>(),
            44usize,
            concat!("Size of: ", stringify!(Camera3D))
        );
        assert_eq!(
            ::std::mem::align_of::<Camera3D>(),
            4usize,
            concat!("Alignment of ", stringify!(Camera3D))
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<Camera3D>())).position as *const _ as usize },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(Camera3D),
                "::",
                stringify!(position)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<Camera3D>())).target as *const _ as usize },
            12usize,
            concat!(
                "Offset of field: ",
                stringify!(Camera3D),
                "::",
                stringify!(target)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<Camera3D>())).up as *const _ as usize },
            24usize,
            concat!(
                "Offset of field: ",
                stringify!(Camera3D),
                "::",
                stringify!(up)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<Camera3D>())).fovy as *const _ as usize },
            36usize,
            concat!(
                "Offset of field: ",
                stringify!(Camera3D),
                "::",
                stringify!(fovy)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<Camera3D>())).camtype as *const _ as usize },
            40usize,
            concat!(
                "Offset of field: ",
                stringify!(Camera3D),
                "::",
                stringify!(type_)
            )
        );
    }

    #[test]
    fn bindgen_test_layout_Camera2D() {
        assert_eq!(
            ::std::mem::size_of::<Camera2D>(),
            24usize,
            concat!("Size of: ", stringify!(Camera2D))
        );
        assert_eq!(
            ::std::mem::align_of::<Camera2D>(),
            4usize,
            concat!("Alignment of ", stringify!(Camera2D))
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<Camera2D>())).offset as *const _ as usize },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(Camera2D),
                "::",
                stringify!(offset)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<Camera2D>())).target as *const _ as usize },
            8usize,
            concat!(
                "Offset of field: ",
                stringify!(Camera2D),
                "::",
                stringify!(target)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<Camera2D>())).rotation as *const _ as usize },
            16usize,
            concat!(
                "Offset of field: ",
                stringify!(Camera2D),
                "::",
                stringify!(rotation)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<Camera2D>())).zoom as *const _ as usize },
            20usize,
            concat!(
                "Offset of field: ",
                stringify!(Camera2D),
                "::",
                stringify!(zoom)
            )
        );
    }

    #[test]
    fn bindgen_test_layout_BoundingBox() {
        assert_eq!(
            ::std::mem::size_of::<BoundingBox>(),
            24usize,
            concat!("Size of: ", stringify!(BoundingBox))
        );
        assert_eq!(
            ::std::mem::align_of::<BoundingBox>(),
            4usize,
            concat!("Alignment of ", stringify!(BoundingBox))
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<BoundingBox>())).min as *const _ as usize },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(BoundingBox),
                "::",
                stringify!(min)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<BoundingBox>())).max as *const _ as usize },
            12usize,
            concat!(
                "Offset of field: ",
                stringify!(BoundingBox),
                "::",
                stringify!(max)
            )
        );
    }

    #[test]
    fn bindgen_test_layout_Mesh() {
        assert_eq!(
            ::std::mem::size_of::<Mesh>(),
            96usize,
            concat!("Size of: ", stringify!(Mesh))
        );
        assert_eq!(
            ::std::mem::align_of::<Mesh>(),
            8usize,
            concat!("Alignment of ", stringify!(Mesh))
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<Mesh>())).vertex_count as *const _ as usize },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(Mesh),
                "::",
                stringify!(vertexCount)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<Mesh>())).triangle_count as *const _ as usize },
            4usize,
            concat!(
                "Offset of field: ",
                stringify!(Mesh),
                "::",
                stringify!(triangleCount)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<Mesh>())).vertices as *const _ as usize },
            8usize,
            concat!(
                "Offset of field: ",
                stringify!(Mesh),
                "::",
                stringify!(vertices)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<Mesh>())).texcoords as *const _ as usize },
            16usize,
            concat!(
                "Offset of field: ",
                stringify!(Mesh),
                "::",
                stringify!(texcoords)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<Mesh>())).texcoords2 as *const _ as usize },
            24usize,
            concat!(
                "Offset of field: ",
                stringify!(Mesh),
                "::",
                stringify!(texcoords2)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<Mesh>())).normals as *const _ as usize },
            32usize,
            concat!(
                "Offset of field: ",
                stringify!(Mesh),
                "::",
                stringify!(normals)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<Mesh>())).tangents as *const _ as usize },
            40usize,
            concat!(
                "Offset of field: ",
                stringify!(Mesh),
                "::",
                stringify!(tangents)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<Mesh>())).colors as *const _ as usize },
            48usize,
            concat!(
                "Offset of field: ",
                stringify!(Mesh),
                "::",
                stringify!(colors)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<Mesh>())).indices as *const _ as usize },
            56usize,
            concat!(
                "Offset of field: ",
                stringify!(Mesh),
                "::",
                stringify!(indices)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<Mesh>())).vao_id as *const _ as usize },
            64usize,
            concat!(
                "Offset of field: ",
                stringify!(Mesh),
                "::",
                stringify!(vaoId)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<Mesh>())).vbo_id as *const _ as usize },
            68usize,
            concat!(
                "Offset of field: ",
                stringify!(Mesh),
                "::",
                stringify!(vboId)
            )
        );
    }

    #[test]
    fn bindgen_test_layout_Shader() {
        assert_eq!(
            ::std::mem::size_of::<Shader>(),
            132usize,
            concat!("Size of: ", stringify!(Shader))
        );
        assert_eq!(
            ::std::mem::align_of::<Shader>(),
            4usize,
            concat!("Alignment of ", stringify!(Shader))
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<Shader>())).id as *const _ as usize },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(Shader),
                "::",
                stringify!(id)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<Shader>())).locs as *const _ as usize },
            4usize,
            concat!(
                "Offset of field: ",
                stringify!(Shader),
                "::",
                stringify!(locs)
            )
        );
    }

    #[test]
    fn bindgen_test_layout_MaterialMap() {
        assert_eq!(
            ::std::mem::size_of::<MaterialMap>(),
            28usize,
            concat!("Size of: ", stringify!(MaterialMap))
        );
        assert_eq!(
            ::std::mem::align_of::<MaterialMap>(),
            4usize,
            concat!("Alignment of ", stringify!(MaterialMap))
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<MaterialMap>())).texture as *const _ as usize },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(MaterialMap),
                "::",
                stringify!(texture)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<MaterialMap>())).color as *const _ as usize },
            20usize,
            concat!(
                "Offset of field: ",
                stringify!(MaterialMap),
                "::",
                stringify!(color)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<MaterialMap>())).value as *const _ as usize },
            24usize,
            concat!(
                "Offset of field: ",
                stringify!(MaterialMap),
                "::",
                stringify!(value)
            )
        );
    }

    #[test]
    fn bindgen_test_layout_Material() {
        assert_eq!(
            ::std::mem::size_of::<Material>(),
            480usize,
            concat!("Size of: ", stringify!(Material))
        );
        assert_eq!(
            ::std::mem::align_of::<Material>(),
            8usize,
            concat!("Alignment of ", stringify!(Material))
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<Material>())).shader as *const _ as usize },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(Material),
                "::",
                stringify!(shader)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<Material>())).maps as *const _ as usize },
            132usize,
            concat!(
                "Offset of field: ",
                stringify!(Material),
                "::",
                stringify!(maps)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<Material>())).params as *const _ as usize },
            472usize,
            concat!(
                "Offset of field: ",
                stringify!(Material),
                "::",
                stringify!(params)
            )
        );
    }

    #[test]
    fn bindgen_test_layout_Model() {
        assert_eq!(
            ::std::mem::size_of::<Model>(),
            640usize,
            concat!("Size of: ", stringify!(Model))
        );
        assert_eq!(
            ::std::mem::align_of::<Model>(),
            8usize,
            concat!("Alignment of ", stringify!(Model))
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<Model>())).mesh as *const _ as usize },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(Model),
                "::",
                stringify!(mesh)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<Model>())).transform as *const _ as usize },
            96usize,
            concat!(
                "Offset of field: ",
                stringify!(Model),
                "::",
                stringify!(transform)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<Model>())).material as *const _ as usize },
            160usize,
            concat!(
                "Offset of field: ",
                stringify!(Model),
                "::",
                stringify!(material)
            )
        );
    }

    #[test]
    fn bindgen_test_layout_Ray() {
        assert_eq!(
            ::std::mem::size_of::<Ray>(),
            24usize,
            concat!("Size of: ", stringify!(Ray))
        );
        assert_eq!(
            ::std::mem::align_of::<Ray>(),
            4usize,
            concat!("Alignment of ", stringify!(Ray))
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<Ray>())).position as *const _ as usize },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(Ray),
                "::",
                stringify!(position)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<Ray>())).direction as *const _ as usize },
            12usize,
            concat!(
                "Offset of field: ",
                stringify!(Ray),
                "::",
                stringify!(direction)
            )
        );
    }

    #[test]
    fn bindgen_test_layout_RayHitInfo() {
        assert_eq!(
            ::std::mem::size_of::<RayHitInfo>(),
            32usize,
            concat!("Size of: ", stringify!(RayHitInfo))
        );
        assert_eq!(
            ::std::mem::align_of::<RayHitInfo>(),
            4usize,
            concat!("Alignment of ", stringify!(RayHitInfo))
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<RayHitInfo>())).hit as *const _ as usize },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(RayHitInfo),
                "::",
                stringify!(hit)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<RayHitInfo>())).distance as *const _ as usize },
            4usize,
            concat!(
                "Offset of field: ",
                stringify!(RayHitInfo),
                "::",
                stringify!(distance)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<RayHitInfo>())).position as *const _ as usize },
            8usize,
            concat!(
                "Offset of field: ",
                stringify!(RayHitInfo),
                "::",
                stringify!(position)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<RayHitInfo>())).normal as *const _ as usize },
            20usize,
            concat!(
                "Offset of field: ",
                stringify!(RayHitInfo),
                "::",
                stringify!(normal)
            )
        );
    }

    #[test]
    fn bindgen_test_layout_Wave() {
        assert_eq!(
            ::std::mem::size_of::<Wave>(),
            24usize,
            concat!("Size of: ", stringify!(Wave))
        );
        assert_eq!(
            ::std::mem::align_of::<Wave>(),
            8usize,
            concat!("Alignment of ", stringify!(Wave))
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<Wave>())).sample_count as *const _ as usize },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(Wave),
                "::",
                stringify!(sampleCount)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<Wave>())).sample_rate as *const _ as usize },
            4usize,
            concat!(
                "Offset of field: ",
                stringify!(Wave),
                "::",
                stringify!(sampleRate)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<Wave>())).sample_size as *const _ as usize },
            8usize,
            concat!(
                "Offset of field: ",
                stringify!(Wave),
                "::",
                stringify!(sampleSize)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<Wave>())).channels as *const _ as usize },
            12usize,
            concat!(
                "Offset of field: ",
                stringify!(Wave),
                "::",
                stringify!(channels)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<Wave>())).data as *const _ as usize },
            16usize,
            concat!(
                "Offset of field: ",
                stringify!(Wave),
                "::",
                stringify!(data)
            )
        );
    }

    #[test]
    fn bindgen_test_layout_Sound() {
        assert_eq!(
            ::std::mem::size_of::<Sound>(),
            24usize,
            concat!("Size of: ", stringify!(Sound))
        );
        assert_eq!(
            ::std::mem::align_of::<Sound>(),
            8usize,
            concat!("Alignment of ", stringify!(Sound))
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<Sound>())).audio_buffer as *const _ as usize },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(Sound),
                "::",
                stringify!(audioBuffer)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<Sound>())).source as *const _ as usize },
            8usize,
            concat!(
                "Offset of field: ",
                stringify!(Sound),
                "::",
                stringify!(source)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<Sound>())).buffer as *const _ as usize },
            12usize,
            concat!(
                "Offset of field: ",
                stringify!(Sound),
                "::",
                stringify!(buffer)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<Sound>())).format as *const _ as usize },
            16usize,
            concat!(
                "Offset of field: ",
                stringify!(Sound),
                "::",
                stringify!(format)
            )
        );
    }

    #[test]
    fn bindgen_test_layout_AudioStream() {
        assert_eq!(
            ::std::mem::size_of::<AudioStream>(),
            40usize,
            concat!("Size of: ", stringify!(AudioStream))
        );
        assert_eq!(
            ::std::mem::align_of::<AudioStream>(),
            8usize,
            concat!("Alignment of ", stringify!(AudioStream))
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<AudioStream>())).sample_rate as *const _ as usize },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(AudioStream),
                "::",
                stringify!(sampleRate)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<AudioStream>())).sample_size as *const _ as usize },
            4usize,
            concat!(
                "Offset of field: ",
                stringify!(AudioStream),
                "::",
                stringify!(sampleSize)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<AudioStream>())).channels as *const _ as usize },
            8usize,
            concat!(
                "Offset of field: ",
                stringify!(AudioStream),
                "::",
                stringify!(channels)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<AudioStream>())).audio_buffer as *const _ as usize },
            16usize,
            concat!(
                "Offset of field: ",
                stringify!(AudioStream),
                "::",
                stringify!(audioBuffer)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<AudioStream>())).format as *const _ as usize },
            24usize,
            concat!(
                "Offset of field: ",
                stringify!(AudioStream),
                "::",
                stringify!(format)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<AudioStream>())).source as *const _ as usize },
            28usize,
            concat!(
                "Offset of field: ",
                stringify!(AudioStream),
                "::",
                stringify!(source)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<AudioStream>())).buffers as *const _ as usize },
            32usize,
            concat!(
                "Offset of field: ",
                stringify!(AudioStream),
                "::",
                stringify!(buffers)
            )
        );
    }

    #[test]
    fn bindgen_test_layout_VrDeviceInfo() {
        assert_eq!(
            ::std::mem::size_of::<VrDeviceInfo>(),
            64usize,
            concat!("Size of: ", stringify!(VrDeviceInfo))
        );
        assert_eq!(
            ::std::mem::align_of::<VrDeviceInfo>(),
            4usize,
            concat!("Alignment of ", stringify!(VrDeviceInfo))
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<VrDeviceInfo>())).h_resolution as *const _ as usize },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(VrDeviceInfo),
                "::",
                stringify!(hResolution)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<VrDeviceInfo>())).v_resolution as *const _ as usize },
            4usize,
            concat!(
                "Offset of field: ",
                stringify!(VrDeviceInfo),
                "::",
                stringify!(vResolution)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<VrDeviceInfo>())).h_screen_size as *const _ as usize },
            8usize,
            concat!(
                "Offset of field: ",
                stringify!(VrDeviceInfo),
                "::",
                stringify!(hScreenSize)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<VrDeviceInfo>())).v_screen_size as *const _ as usize },
            12usize,
            concat!(
                "Offset of field: ",
                stringify!(VrDeviceInfo),
                "::",
                stringify!(vScreenSize)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<VrDeviceInfo>())).v_screen_center as *const _ as usize },
            16usize,
            concat!(
                "Offset of field: ",
                stringify!(VrDeviceInfo),
                "::",
                stringify!(vScreenCenter)
            )
        );
        assert_eq!(
            unsafe {
                &(*(::std::ptr::null::<VrDeviceInfo>())).eye_to_screen_distance as *const _ as usize
            },
            20usize,
            concat!(
                "Offset of field: ",
                stringify!(VrDeviceInfo),
                "::",
                stringify!(eyeToScreenDistance)
            )
        );
        assert_eq!(
            unsafe {
                &(*(::std::ptr::null::<VrDeviceInfo>())).lens_separation_distance as *const _ as usize
            },
            24usize,
            concat!(
                "Offset of field: ",
                stringify!(VrDeviceInfo),
                "::",
                stringify!(lensSeparationDistance)
            )
        );
        assert_eq!(
            unsafe {
                &(*(::std::ptr::null::<VrDeviceInfo>())).interpupillary_distance as *const _ as usize
            },
            28usize,
            concat!(
                "Offset of field: ",
                stringify!(VrDeviceInfo),
                "::",
                stringify!(interpupillaryDistance)
            )
        );
        assert_eq!(
            unsafe {
                &(*(::std::ptr::null::<VrDeviceInfo>())).lens_distortion_values as *const _ as usize
            },
            32usize,
            concat!(
                "Offset of field: ",
                stringify!(VrDeviceInfo),
                "::",
                stringify!(lensDistortionValues)
            )
        );
        assert_eq!(
            unsafe { &(*(::std::ptr::null::<VrDeviceInfo>())).chroma_ab_correction as *const _ as usize },
            48usize,
            concat!(
                "Offset of field: ",
                stringify!(VrDeviceInfo),
                "::",
                stringify!(chromaAbCorrection)
            )
        );
    }
}