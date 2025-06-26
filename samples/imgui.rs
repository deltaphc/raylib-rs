// imgui support based off of
use imgui::{ConfigFlags, internal::RawWrapper};
use raylib::color::Color;
use raylib::consts;
use raylib::consts::{GamepadAxis, GamepadButton, KeyboardKey, MouseCursor};
use raylib::drawing::RaylibDrawHandle;
use raylib::ffi;
use raylib::math::Vector2;
use raylib::texture::{Image, Texture2D};
use raylib::{RaylibHandle, RaylibThread};
use std::ffi::{CStr, CString};

use ffi::{GetClipboardText, SetClipboardText};
use imgui::{BackendFlags, ClipboardBackend, ColorStackToken, Context, TextureId};

struct Clipboard;

impl ClipboardBackend for Clipboard {
    #[must_use]
    fn get(&mut self) -> Option<String> {
        unsafe {
            let c = GetClipboardText();
            let c = CStr::from_ptr(c);
            c.to_str().map(|s| s.to_owned()).ok()
        }
    }
    fn set(&mut self, value: &str) {
        let s = CString::new(value);
        unsafe {
            match s {
                Ok(item) => {
                    SetClipboardText(item.as_ptr());
                }
                Err(e) => {
                    dbg!(e);
                }
            }
        }
    }
}
// based off of: https://github.com/raylib-extras/rlImGui
pub struct RaylibImguiSupport {
    pub context: imgui::Context,
    renderer: RaylibRenderer,
    platform: RaylibPlatform,
}

impl RaylibImguiSupport {
    #[must_use]
    pub fn setup(
        rl: &mut raylib::RaylibHandle,
        thread: &raylib::RaylibThread,
    ) -> RaylibImguiSupport {
        let mut context = imgui::Context::create();
        context.set_ini_filename(None);
        context.set_log_filename(None);
        context
            .fonts()
            .add_font(&[imgui::FontSource::DefaultFontData { config: None }]);
        let style = context.style_mut();
        let renderer = RaylibRenderer::init(rl, thread, &mut context);
        let platform = RaylibPlatform::init(rl, &mut context);

        RaylibImguiSupport {
            context,
            renderer,
            platform,
        }
    }

    #[must_use]
    pub fn start_frame(&mut self, rl: &mut raylib::RaylibHandle) -> &mut imgui::Ui {
        self.platform.new_frame(rl, &mut self.context);
        self.platform.process_events(rl, &mut self.context);

        self.context.new_frame()
    }

    pub fn end_frame(&mut self, rl: &mut raylib::drawing::RaylibDrawHandle) {
        let [fb_x, fb_y] = self.context.io_mut().display_framebuffer_scale;
        let draw_data = self.context.render();

        self.renderer.render(rl, draw_data, [fb_x, fb_y]);
    }
}

struct LastFrame {
    pub focused: bool,
    pub ctrl: bool,
    pub shift: bool,
    pub alt: bool,
    pub super_key: bool,
}
pub struct RaylibPlatform {
    cursor: Option<imgui::MouseCursor>,

    last: LastFrame,
}

impl RaylibPlatform {
    pub fn init(rl: &mut RaylibHandle, imgui: &mut Context) -> RaylibPlatform {
        imgui.set_platform_name(Some(String::from("imgui_impl_raylib")));
        let io = imgui.io_mut();

        io.backend_flags |= BackendFlags::HAS_GAMEPAD
            | BackendFlags::HAS_SET_MOUSE_POS
            | BackendFlags::HAS_MOUSE_CURSORS;
        io.config_flags |= ConfigFlags::NAV_ENABLE_GAMEPAD;
        io.mouse_pos = [0.0, 0.0];

        imgui.set_clipboard_backend(Clipboard);

        RaylibPlatform {
            cursor: None,

            last: LastFrame {
                focused: rl.is_window_focused(),
                ctrl: false,
                shift: false,
                alt: false,
                super_key: false,
            },
        }
    }

    pub fn new_frame(&mut self, rl: &mut raylib::RaylibHandle, context: &mut Context) {
        let io = context.io_mut();
        let mut resolution_scale = rl.get_window_scale_dpi();

        if rl.is_window_fullscreen() {
            let monitor = raylib::window::get_current_monitor();
            io.display_size[0] = raylib::window::get_monitor_width(monitor) as f32;
            io.display_size[1] = raylib::window::get_monitor_height(monitor) as f32;
        } else {
            io.display_size[0] = rl.get_screen_width() as f32;
            io.display_size[1] = rl.get_screen_height() as f32;
        }
        #[cfg(not(target_os = "macos"))]
        {
            if !rl.get_window_state().window_highdpi() {
                resolution_scale = Vector2::new(1.0, 1.0);
            }
        }

        io.display_framebuffer_scale = [resolution_scale.x, resolution_scale.y];

        if rl.get_frame_time() == 0.0 {
            // avoid triggering imgui assert on game startup for sdl backend
            io.delta_time = 1.0 / 60.0;
        } else {
            io.delta_time = rl.get_frame_time();
        }

        // must process mouse events here for best latency(https://github.com/ocornut/imgui/blob/master/docs/EXAMPLES.md)
        if io.want_set_mouse_pos {
            rl.set_mouse_position(Vector2::new(io.mouse_pos[0], io.mouse_pos[1]));
        } else {
            let mpos = rl.get_mouse_position();
            io.add_mouse_pos_event([mpos.x, mpos.y]);
        }

        #[rustfmt::skip]
        let map_mouse = [
            (consts::MouseButton::MOUSE_BUTTON_LEFT,imgui::MouseButton::Left,),
            (consts::MouseButton::MOUSE_BUTTON_RIGHT,imgui::MouseButton::Right,),
            (consts::MouseButton::MOUSE_BUTTON_MIDDLE,imgui::MouseButton::Middle,),
            (consts::MouseButton::MOUSE_BUTTON_FORWARD,imgui::MouseButton::Extra1,),
            (consts::MouseButton::MOUSE_BUTTON_BACK,imgui::MouseButton::Extra2,),
        ];
        for (ray_mouse, imgui_mouse) in map_mouse {
            if rl.is_mouse_button_pressed(ray_mouse) {
                io.add_mouse_button_event(imgui_mouse, true);
            } else if rl.is_mouse_button_released(ray_mouse) {
                io.add_mouse_button_event(imgui_mouse, false);
            }
        }

        let mouse_wheel = rl.get_mouse_wheel_move_v();
        io.mouse_wheel += mouse_wheel.y;
        io.mouse_wheel_h += mouse_wheel.x;

        if io
            .backend_flags
            .contains(imgui::BackendFlags::HAS_MOUSE_CURSORS)
        {
            let cursor_change = io
                .config_flags
                .contains(ConfigFlags::NO_MOUSE_CURSOR_CHANGE);
            if !cursor_change {
                let mouse_draw_cursor = io.mouse_draw_cursor;
                let imgui_cursor = context.mouse_cursor();
                if imgui_cursor != self.cursor || mouse_draw_cursor {
                    self.cursor = imgui_cursor;
                    if mouse_draw_cursor || imgui_cursor.is_none() {
                        rl.hide_cursor();
                    } else if let Some(cursor) = imgui_cursor {
                        rl.show_cursor();
                        if !cursor_change {
                            rl.set_mouse_cursor(translate_cursor(cursor));
                        }
                    }
                }
            }
        }
    }

    pub fn process_events(&mut self, rl: &mut raylib::RaylibHandle, context: &mut Context) {
        let io = context.io_mut();

        let focused = rl.is_window_focused();
        if focused != self.last.focused {
            io.app_focus_lost = focused;
            // wait for merge of: https://github.com/imgui-rs/imgui-rs/pull/811
            //io.add_focus_event(focused);
        }
        self.last.focused = focused;

        let ctrl = rl.is_key_down(KeyboardKey::KEY_RIGHT_CONTROL)
            || rl.is_key_down(KeyboardKey::KEY_LEFT_CONTROL);
        if ctrl != self.last.ctrl {
            io.add_key_event(imgui::Key::ReservedForModCtrl, ctrl);
        }
        self.last.ctrl = ctrl;

        let alt =
            rl.is_key_down(KeyboardKey::KEY_RIGHT_ALT) || rl.is_key_down(KeyboardKey::KEY_LEFT_ALT);
        if alt != self.last.alt {
            io.add_key_event(imgui::Key::ReservedForModAlt, alt);
        }
        self.last.alt = alt;

        let shift = rl.is_key_down(KeyboardKey::KEY_RIGHT_SHIFT)
            || rl.is_key_down(KeyboardKey::KEY_LEFT_SHIFT);
        if shift != self.last.shift {
            io.add_key_event(imgui::Key::ReservedForModShift, shift);
        }
        self.last.shift = shift;

        let super_key = rl.is_key_down(KeyboardKey::KEY_RIGHT_SUPER)
            || rl.is_key_down(KeyboardKey::KEY_LEFT_SUPER);
        if super_key != self.last.super_key {
            io.add_key_event(imgui::Key::ReservedForModSuper, super_key);
        }
        self.last.super_key = super_key;
        // get the pressed keys
        // note: we dont use get_key_pressed because it pops it of a queue and is destructive(https://discord.com/channels/426912293134270465/426912293956222978):
        for (rl_key, im_key) in RL_IMGUI_KEYMAP {
            if rl.is_key_released(rl_key) {
                io.add_key_event(im_key, false);
            } else if rl.is_key_pressed(rl_key) {
                io.add_key_event(im_key, true);
            }
        }

        if io.want_capture_keyboard {
            while let Some(char) = rl.get_char_pressed() {
                io.add_input_character(char);
            }
        }
        if io.config_flags.contains(ConfigFlags::NAV_ENABLE_GAMEPAD) && rl.is_gamepad_available(0) {
            #[rustfmt::skip]
            let gamepad_map = [
                (GamepadButton::GAMEPAD_BUTTON_LEFT_FACE_UP, imgui::Key::GamepadDpadUp),
                (GamepadButton::GAMEPAD_BUTTON_LEFT_FACE_RIGHT, imgui::Key::GamepadDpadRight),
                (GamepadButton::GAMEPAD_BUTTON_LEFT_FACE_DOWN, imgui::Key::GamepadDpadDown),
                (GamepadButton::GAMEPAD_BUTTON_LEFT_FACE_LEFT,imgui::Key::GamepadDpadLeft,),
                //right
                (GamepadButton::GAMEPAD_BUTTON_RIGHT_FACE_UP,imgui::Key::GamepadFaceUp),
                (GamepadButton::GAMEPAD_BUTTON_RIGHT_FACE_RIGHT,imgui::Key::GamepadFaceRight),
                (GamepadButton::GAMEPAD_BUTTON_RIGHT_FACE_DOWN,imgui::Key::GamepadFaceDown),
                (GamepadButton::GAMEPAD_BUTTON_RIGHT_FACE_LEFT,imgui::Key::GamepadFaceLeft),
                // triggers
                (GamepadButton::GAMEPAD_BUTTON_LEFT_TRIGGER_1,imgui::Key::GamepadL1),
                (GamepadButton::GAMEPAD_BUTTON_LEFT_TRIGGER_2,imgui::Key::GamepadL2),
                (GamepadButton::GAMEPAD_BUTTON_RIGHT_TRIGGER_1,imgui::Key::GamepadR1),
                (GamepadButton::GAMEPAD_BUTTON_RIGHT_TRIGGER_2,imgui::Key::GamepadR2),
                (GamepadButton::GAMEPAD_BUTTON_LEFT_THUMB,imgui::Key::GamepadL3),
                (GamepadButton::GAMEPAD_BUTTON_RIGHT_THUMB,imgui::Key::GamepadR3),
                // start
                (GamepadButton::GAMEPAD_BUTTON_MIDDLE_LEFT,imgui::Key::GamepadStart),
                (GamepadButton::GAMEPAD_BUTTON_MIDDLE_RIGHT,imgui::Key::GamepadBack),
            ];
            //handle gamepad events
            for (ray_key, im_key) in gamepad_map {
                if rl.is_gamepad_button_pressed(0, ray_key) {
                    io.add_key_event(im_key, true);
                } else if rl.is_gamepad_button_released(0, ray_key) {
                    io.add_key_event(im_key, false);
                }
            }
            #[rustfmt::skip]
            let joystick_map = [
                //left stick
                (GamepadAxis::GAMEPAD_AXIS_LEFT_X, imgui::Key::GamepadLStickLeft, imgui::Key::GamepadLStickRight),
                (GamepadAxis::GAMEPAD_AXIS_LEFT_Y, imgui::Key::GamepadLStickUp, imgui::Key::GamepadLStickDown),
                //right stick
                (GamepadAxis::GAMEPAD_AXIS_RIGHT_X, imgui::Key::GamepadRStickLeft, imgui::Key::GamepadRStickRight),
                (GamepadAxis::GAMEPAD_AXIS_RIGHT_Y, imgui::Key::GamepadRStickUp, imgui::Key::GamepadRStickDown),
            ];
            // handle joystick events
            let dead_zone = 0.20;
            for (axis, neg_key, pos_key) in joystick_map {
                let axis_value = rl.get_gamepad_axis_movement(0, axis);
                io.add_key_analog_event(
                    neg_key,
                    axis_value < -dead_zone,
                    if axis_value < -dead_zone {
                        -axis_value
                    } else {
                        0.0
                    },
                );
                io.add_key_analog_event(
                    pos_key,
                    axis_value > dead_zone,
                    if axis_value > dead_zone {
                        axis_value
                    } else {
                        0.0
                    },
                );
            }
        }
    }
}

pub struct RaylibRenderer {
    _font_texture: Option<Texture2D>,
}

impl RaylibRenderer {
    #[must_use]
    pub fn init(
        rl: &mut RaylibHandle,
        thread: &RaylibThread,
        imgui: &mut imgui::Context,
    ) -> RaylibRenderer {
        let mut res = RaylibRenderer {
            _font_texture: None,
        };
        res.reload_fonts(rl, thread, imgui);
        res
    }
    #[must_use]
    pub fn load_fonts(
        rl: &mut RaylibHandle,
        thread: &RaylibThread,
        imgui: &mut imgui::Context,
    ) -> Texture2D {
        let texture = imgui.fonts().build_rgba32_texture();
        let image =
            Image::gen_image_color(texture.width as i32, texture.height as i32, Color::BLANK);

        let pixel_len = texture.width * texture.height * 4;

        unsafe {
            (image.data as *mut u8)
                .copy_from_nonoverlapping(texture.data.as_ptr(), pixel_len as usize)
        };

        let texture = rl.load_texture_from_image(thread, &image).unwrap();
        imgui.fonts().tex_id = (texture.id as usize).into();
        return texture;
    }

    pub fn reload_fonts(
        &mut self,
        rl: &mut RaylibHandle,
        thread: &RaylibThread,
        imgui: &mut imgui::Context,
    ) {
        // old font texture will auto drop when overwritten
        self._font_texture = Some(RaylibRenderer::load_fonts(rl, thread, imgui));
    }

    pub fn render(
        &mut self,
        rl: &RaylibDrawHandle,
        draw_data: &imgui::DrawData,
        framebuffer_scale: [f32; 2],
    ) {
        unsafe {
            ffi::rlDrawRenderBatchActive();
            ffi::rlDisableBackfaceCulling();
        }

        for list in draw_data.draw_lists() {
            for command in list.commands() {
                match command {
                    imgui::DrawCmd::Elements { count, cmd_params } => {
                        let [x, y, z, w] = cmd_params.clip_rect;
                        self.enable_scissor(
                            rl,
                            x - draw_data.display_pos[0],
                            y - draw_data.display_pos[1],
                            z - (x - draw_data.display_pos[0]),
                            w - (y - draw_data.display_pos[1]),
                            framebuffer_scale,
                        );

                        self.render_triangles(
                            count,
                            cmd_params.idx_offset,
                            list.idx_buffer(),
                            list.vtx_buffer(),
                            cmd_params.texture_id,
                        );
                        unsafe {
                            ffi::rlDrawRenderBatchActive();
                        }
                    }
                    imgui::DrawCmd::RawCallback { callback, raw_cmd } => {
                        let clip_rect = unsafe { *raw_cmd }.ClipRect;

                        self.enable_scissor(
                            rl,
                            clip_rect.x - draw_data.display_pos[0],
                            clip_rect.y - draw_data.display_pos[1],
                            clip_rect.z - (clip_rect.x - draw_data.display_pos[0]),
                            clip_rect.w - (clip_rect.y - draw_data.display_pos[1]),
                            framebuffer_scale,
                        );

                        unsafe { callback(list.raw(), raw_cmd) }
                    }
                    imgui::DrawCmd::ResetRenderState => (),
                }
            }
        }

        unsafe {
            ffi::rlSetTexture(0);
            ffi::rlDisableScissorTest();
            ffi::rlEnableBackfaceCulling();
        }
    }

    fn enable_scissor(
        &self,
        rl: &RaylibDrawHandle,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        framebuffer_scale: [f32; 2],
    ) {
        unsafe {
            ffi::rlEnableScissorTest();
        }
        let [mut scale_x, mut scale_y] = framebuffer_scale;
        // does not work on mac, see jeffm imgui bindings
        #[cfg(not(target_os = "macos"))]
        {
            if !rl.get_window_state().window_highdpi() {
                scale_x = 1.0;
                scale_y = 1.0;
            }
        }

        let scissor_x = (x * scale_x) as i32;
        let scissor_y = ((rl.get_screen_height() as f32 - (y + height)) * scale_y) as i32;
        let scissor_width = (width * scale_x) as i32;
        let scissor_height = (height * scale_y) as i32;
        unsafe {
            ffi::rlScissor(scissor_x, scissor_y, scissor_width, scissor_height);
        }
    }

    fn render_triangles(
        &self,
        count: usize,
        start: usize,
        index_buffer: &[imgui::DrawIdx],
        vertex_buffer: &[imgui::DrawVert],
        texture: imgui::TextureId,
    ) {
        if count < 3 {
            return;
        }

        let texture_id = texture.id() as u32;

        unsafe {
            ffi::rlBegin(ffi::RL_TRIANGLES as i32);
            ffi::rlSetTexture(texture_id);
        }

        for i in (0..count).step_by(3) {
            unsafe {
                if ffi::rlCheckRenderBatchLimit(3) {
                    ffi::rlBegin(ffi::RL_TRIANGLES as i32);
                    ffi::rlSetTexture(texture_id);
                }
            }

            let idx1 = index_buffer[start + i];
            let idx2 = index_buffer[start + i + 1];
            let idx3 = index_buffer[start + i + 2];

            let vert1 = &vertex_buffer[idx1 as usize];
            let vert2 = &vertex_buffer[idx2 as usize];
            let vert3 = &vertex_buffer[idx3 as usize];

            RaylibRenderer::triangle_vertex(vert1);
            RaylibRenderer::triangle_vertex(vert2);
            RaylibRenderer::triangle_vertex(vert3);
        }

        unsafe {
            ffi::rlEnd();
        }
    }

    fn triangle_vertex(vertex: &imgui::DrawVert) {
        let [r, g, b, a] = vertex.col;
        unsafe {
            ffi::rlColor4ub(r, g, b, a);
            ffi::rlTexCoord2f(vertex.uv[0], vertex.uv[1]);
            ffi::rlVertex2f(vertex.pos[0], vertex.pos[1]);
        }
    }
}

#[must_use]
fn translate_cursor(imgui_cursor: imgui::MouseCursor) -> MouseCursor {
    match imgui_cursor {
        imgui::MouseCursor::Arrow => MouseCursor::MOUSE_CURSOR_ARROW,
        imgui::MouseCursor::TextInput => MouseCursor::MOUSE_CURSOR_IBEAM,
        imgui::MouseCursor::ResizeAll => MouseCursor::MOUSE_CURSOR_RESIZE_ALL,
        imgui::MouseCursor::ResizeNS => MouseCursor::MOUSE_CURSOR_RESIZE_NS,
        imgui::MouseCursor::ResizeEW => MouseCursor::MOUSE_CURSOR_RESIZE_EW,
        imgui::MouseCursor::ResizeNESW => MouseCursor::MOUSE_CURSOR_RESIZE_NESW,
        imgui::MouseCursor::ResizeNWSE => MouseCursor::MOUSE_CURSOR_RESIZE_NWSE,
        imgui::MouseCursor::Hand => MouseCursor::MOUSE_CURSOR_POINTING_HAND,
        imgui::MouseCursor::NotAllowed => MouseCursor::MOUSE_CURSOR_NOT_ALLOWED,
    }
}

#[rustfmt::skip]
const RL_IMGUI_KEYMAP: [(KeyboardKey, imgui::Key); 105] = [
    (KeyboardKey::KEY_A ,imgui::Key::A,),
    (KeyboardKey::KEY_B ,imgui::Key::B,),
    (KeyboardKey::KEY_C ,imgui::Key::C,),
    (KeyboardKey::KEY_D ,imgui::Key::D,),
    (KeyboardKey::KEY_E ,imgui::Key::E,),
    (KeyboardKey::KEY_F ,imgui::Key::F,),
    (KeyboardKey::KEY_G ,imgui::Key::G,),
    (KeyboardKey::KEY_H ,imgui::Key::H,),
    (KeyboardKey::KEY_I ,imgui::Key::I,),
    (KeyboardKey::KEY_J ,imgui::Key::J,),
    (KeyboardKey::KEY_K ,imgui::Key::K,),
    (KeyboardKey::KEY_L ,imgui::Key::L,),
    (KeyboardKey::KEY_M ,imgui::Key::M,),
    (KeyboardKey::KEY_N ,imgui::Key::N,),
    (KeyboardKey::KEY_O ,imgui::Key::O,),
    (KeyboardKey::KEY_P ,imgui::Key::P,),
    (KeyboardKey::KEY_Q ,imgui::Key::Q,),
    (KeyboardKey::KEY_R ,imgui::Key::R,),
    (KeyboardKey::KEY_S ,imgui::Key::S,),
    (KeyboardKey::KEY_T ,imgui::Key::T,),
    (KeyboardKey::KEY_U ,imgui::Key::U,),
    (KeyboardKey::KEY_V ,imgui::Key::V,),
    (KeyboardKey::KEY_W ,imgui::Key::W,),
    (KeyboardKey::KEY_X ,imgui::Key::X,),
    (KeyboardKey::KEY_Y ,imgui::Key::Y,),
    (KeyboardKey::KEY_Z ,imgui::Key::Z,),
    (KeyboardKey::KEY_ONE ,imgui::Key::Keypad1,),
    (KeyboardKey::KEY_TWO ,imgui::Key::Keypad2,),
    (KeyboardKey::KEY_THREE ,imgui::Key::Keypad3,),
    (KeyboardKey::KEY_FOUR ,imgui::Key::Keypad4,),
    (KeyboardKey::KEY_FIVE ,imgui::Key::Keypad5,),
    (KeyboardKey::KEY_SIX ,imgui::Key::Keypad6,),
    (KeyboardKey::KEY_SEVEN ,imgui::Key::Keypad7,),
    (KeyboardKey::KEY_EIGHT ,imgui::Key::Keypad8,),
    (KeyboardKey::KEY_NINE ,imgui::Key::Keypad9,),
    (KeyboardKey::KEY_ZERO ,imgui::Key::Keypad0,),
    (KeyboardKey::KEY_ENTER ,imgui::Key::Enter,),
    (KeyboardKey::KEY_ESCAPE ,imgui::Key::Escape,),
    (KeyboardKey::KEY_BACKSPACE ,imgui::Key::Backspace,),
    (KeyboardKey::KEY_TAB ,imgui::Key::Tab,),
    (KeyboardKey::KEY_SPACE ,imgui::Key::Space,),
    (KeyboardKey::KEY_MINUS ,imgui::Key::Minus,),
    (KeyboardKey::KEY_EQUAL ,imgui::Key::Equal,),
    (KeyboardKey::KEY_LEFT_BRACKET ,imgui::Key::LeftBracket,),
    (KeyboardKey::KEY_RIGHT_BRACKET ,imgui::Key::RightBracket,),
    (KeyboardKey::KEY_BACKSLASH ,imgui::Key::Backslash,),
    (KeyboardKey::KEY_SEMICOLON ,imgui::Key::Semicolon,),
    (KeyboardKey::KEY_APOSTROPHE ,imgui::Key::Apostrophe,),
    (KeyboardKey::KEY_GRAVE ,imgui::Key::GraveAccent,),
    (KeyboardKey::KEY_COMMA ,imgui::Key::Comma,),
    (KeyboardKey::KEY_PERIOD ,imgui::Key::Period,),
    (KeyboardKey::KEY_SLASH ,imgui::Key::Slash,),
    (KeyboardKey::KEY_CAPS_LOCK ,imgui::Key::CapsLock,),
    (KeyboardKey::KEY_F1 ,imgui::Key::F1,),
    (KeyboardKey::KEY_F2 ,imgui::Key::F2,),
    (KeyboardKey::KEY_F3 ,imgui::Key::F3,),
    (KeyboardKey::KEY_F4 ,imgui::Key::F4,),
    (KeyboardKey::KEY_F5 ,imgui::Key::F5,),
    (KeyboardKey::KEY_F6 ,imgui::Key::F6,),
    (KeyboardKey::KEY_F7 ,imgui::Key::F7,),
    (KeyboardKey::KEY_F8 ,imgui::Key::F8,),
    (KeyboardKey::KEY_F9 ,imgui::Key::F9,),
    (KeyboardKey::KEY_F10 ,imgui::Key::F10,),
    (KeyboardKey::KEY_F11 ,imgui::Key::F11,),
    (KeyboardKey::KEY_F12 ,imgui::Key::F12,),
    (KeyboardKey::KEY_PRINT_SCREEN ,imgui::Key::PrintScreen,),
    (KeyboardKey::KEY_SCROLL_LOCK ,imgui::Key::ScrollLock,),
    (KeyboardKey::KEY_PAUSE ,imgui::Key::Pause,),
    (KeyboardKey::KEY_INSERT ,imgui::Key::Insert,),
    (KeyboardKey::KEY_HOME ,imgui::Key::Home,),
    (KeyboardKey::KEY_PAGE_UP ,imgui::Key::PageUp,),
    (KeyboardKey::KEY_DELETE ,imgui::Key::Delete,),
    (KeyboardKey::KEY_END ,imgui::Key::End,),
    (KeyboardKey::KEY_PAGE_DOWN ,imgui::Key::PageDown,),
    (KeyboardKey::KEY_RIGHT ,imgui::Key::RightArrow,),
    (KeyboardKey::KEY_LEFT ,imgui::Key::LeftArrow,),
    (KeyboardKey::KEY_DOWN ,imgui::Key::DownArrow,),
    (KeyboardKey::KEY_UP ,imgui::Key::UpArrow,),
    (KeyboardKey::KEY_KP_DIVIDE ,imgui::Key::KeypadDivide,),
    (KeyboardKey::KEY_KP_MULTIPLY ,imgui::Key::KeypadMultiply,),
    (KeyboardKey::KEY_KP_SUBTRACT ,imgui::Key::KeypadSubtract,),
    (KeyboardKey::KEY_KP_ADD ,imgui::Key::KeypadAdd,),
    (KeyboardKey::KEY_KP_ENTER ,imgui::Key::KeypadEnter,),
    (KeyboardKey::KEY_KP_1 ,imgui::Key::Keypad1,),
    (KeyboardKey::KEY_KP_2 ,imgui::Key::Keypad2,),
    (KeyboardKey::KEY_KP_3 ,imgui::Key::Keypad3,),
    (KeyboardKey::KEY_KP_4 ,imgui::Key::Keypad4,),
    (KeyboardKey::KEY_KP_5 ,imgui::Key::Keypad5,),
    (KeyboardKey::KEY_KP_6 ,imgui::Key::Keypad6,),
    (KeyboardKey::KEY_KP_7 ,imgui::Key::Keypad7,),
    (KeyboardKey::KEY_KP_8 ,imgui::Key::Keypad8,),
    (KeyboardKey::KEY_KP_9 ,imgui::Key::Keypad9,),
    (KeyboardKey::KEY_KP_0 ,imgui::Key::Keypad0,),
    (KeyboardKey::KEY_KP_DECIMAL ,imgui::Key::KeypadDecimal,),
    (KeyboardKey::KEY_KB_MENU ,imgui::Key::Menu,),
    (KeyboardKey::KEY_KP_EQUAL ,imgui::Key::KeypadEqual,),
    (KeyboardKey::KEY_LEFT_CONTROL ,imgui::Key::LeftCtrl,),
    (KeyboardKey::KEY_LEFT_SHIFT ,imgui::Key::LeftShift,),
    (KeyboardKey::KEY_LEFT_ALT ,imgui::Key::LeftAlt,),
    (KeyboardKey::KEY_LEFT_SUPER ,imgui::Key::LeftSuper,),
    (KeyboardKey::KEY_RIGHT_CONTROL ,imgui::Key::RightCtrl,),
    (KeyboardKey::KEY_RIGHT_SHIFT ,imgui::Key::RightShift,),
    (KeyboardKey::KEY_RIGHT_ALT ,imgui::Key::RightAlt,),
    (KeyboardKey::KEY_RIGHT_SUPER ,imgui::Key::RightSuper,),
    (KeyboardKey::KEY_NUM_LOCK ,imgui::Key::NumLock,),
];

#[must_use]
/// calculates the uv for imgui
/// might have to use it as:
/// NOTE: for RenderTexture(FrameBuffers), use `calc_uv_rendertexture` instead or else you will have to swap the uvs:.uv0([uv0[0], uv1[1]]).uv1([uv1[0], uv0[1]]) to prevent upsidedown and flip on the x-axis
pub fn calc_uv(
    tex_width: f32,
    tex_height: f32,
    frame_x: f32,
    frame_y: f32,
    frame_w: f32,
    frame_h: f32,
) -> ([f32; 2], [f32; 2]) {
    let uv0 = [frame_x / tex_width, frame_y / tex_height];
    let uv1 = [
        (frame_x + frame_w) / tex_width,
        (frame_y + frame_h) / tex_height,
    ];
    return (uv0, uv1);
}
/// render texture uvs are calculated differently from regular texture2d uvs in that they are y-flipped so we need negative uvs
#[must_use]
pub fn calc_uv_rendertexture(
    tex_width: f32,
    tex_height: f32,
    frame_x: f32,
    frame_y: f32,
    frame_w: f32,
    frame_h: f32,
) -> ([f32; 2], [f32; 2]) {
    let (mut uv0, mut uv1) = calc_uv(tex_width, tex_height, frame_x, frame_y, frame_w, frame_h);
    uv0[1] *= -1.0;
    uv1[1] *= -1.0;
    (uv0, uv1)
}

/// must be called first before any other imgui calls or window mouse position will be messed up
/// calling it after imgui widget calls will have 0.0 start after all the widgets called before
#[must_use]
pub fn get_imgui_window_mouse_position(ui: &imgui::Ui) -> (f32, f32) {
    // disable rounded corners for this
    let mpos = ui.io().mouse_pos;
    //grabs the absolute coordinates of the opened window
    let pos = ui.cursor_screen_pos();
    ((mpos[0] - pos[0]).round(), (mpos[1] - pos[1]).round())
}

use raylib::prelude::*;

fn main() {
    let (mut rl, thread) = raylib::init().width(1280).height(720).build();
    let mut imgui_rl = RaylibImguiSupport::setup(&mut rl, &thread);

    let mut close_demo = false;
    while (!rl.window_should_close()) {
        let ui = imgui_rl.start_frame(&mut rl);
        let mut d = rl.begin_drawing(&thread);
        {
            d.clear_background(Color::WHITE);
            ui.show_demo_window(&mut close_demo);
            imgui_rl.end_frame(&mut d);
        }

        // this does not work due to borrowing issues with closures:
        //rl.draw(&thread, |mut d| {
        //    d.clear_background(Color::WHITE);
        //    ui.show_demo_window(&mut close_demo);
        //    imgui_rl.end_frame(&mut d);
        //});
    }
}
