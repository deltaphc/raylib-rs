//! Keyboard, Controller, and Mouse related functions
use std::ffi::CStr;

use mint::Vector2;

use super::RaylibHandle;
use crate::{
    consts::{GamepadButton, Gesture, KeyboardKey, MouseButton, GamepadAxis},
    ffi,
};

impl RaylibHandle<'_> {
    /// Detect if a key has been pressed once.
    #[inline]
    pub fn is_key_pressed(&self, key: KeyboardKey) -> bool {
        unsafe { ffi::IsKeyPressed((key as u32) as i32) }
    }

    /// Detect if a key is being pressed.
    #[inline]
    pub fn is_key_down(&self, key: KeyboardKey) -> bool {
        unsafe { ffi::IsKeyDown((key as u32) as i32) }
    }

    /// Detect if a key has been released once.
    #[inline]
    pub fn is_key_released(&self, key: KeyboardKey) -> bool {
        unsafe { ffi::IsKeyReleased((key as u32) as i32) }
    }

    /// Detect if a key is NOT being pressed.
    #[inline]
    pub fn is_key_up(&self, key: KeyboardKey) -> bool {
        unsafe { ffi::IsKeyUp((key as u32) as i32) }
    }

    /// Gets latest key pressed.
    #[inline]
    pub fn get_key_pressed(&self) -> Option<KeyboardKey> {
        let key = unsafe { ffi::GetKeyPressed() };
        if key > 0 {
            return key_from_i32(key);
        }
        None
    }

    /// Gets latest key pressed.
    #[inline]
    pub fn get_key_pressed_number(&self) -> Option<u32> {
        let key = unsafe { ffi::GetKeyPressed() };
        if key > 0 {
            return Some(key as u32);
        }
        None
    }

    /// Gets latest char (unicode) pressed
    #[inline]
    pub fn get_char_pressed(&self) -> Option<char> {
        let char_code = unsafe { ffi::GetCharPressed() };
        if char_code > 0 {
            return char::from_u32(char_code as u32);
        }
        None
    }

    /// Sets a custom key to exit program (default is ESC).
    // #[inline]
    pub fn set_exit_key(&self, key: Option<KeyboardKey>) {
        unsafe {
            match key {
                Some(k) => ffi::SetExitKey((k as u32) as i32),
                None => ffi::SetExitKey(0),
            }
        }
    }

    /// Detect if a gamepad is available.
    #[inline]
    pub fn is_gamepad_available(&self, gamepad: i32) -> bool {
        unsafe { ffi::IsGamepadAvailable(gamepad) }
    }

    /// Returns gamepad internal name id.
    #[inline]
    pub fn get_gamepad_name(&self, gamepad: i32) -> Option<String> {
        unsafe {
            let name = ffi::GetGamepadName(gamepad);
            match name.is_null() {
                false => Some(CStr::from_ptr(name).to_str().unwrap().to_owned()),
                true => None,
            }
        }
    }

    /// Detect if a gamepad button has been pressed once.
    #[inline]
    pub fn is_gamepad_button_pressed(&self, gamepad: i32, button: GamepadButton) -> bool {
        unsafe { ffi::IsGamepadButtonPressed(gamepad, (button as u32) as i32) }
    }

    /// Detect if a gamepad button is being pressed.
    #[inline]
    pub fn is_gamepad_button_down(&self, gamepad: i32, button: GamepadButton) -> bool {
        unsafe { ffi::IsGamepadButtonDown(gamepad, (button as u32) as i32) }
    }

    /// Detect if a gamepad button has been released once.
    #[inline]
    pub fn is_gamepad_button_released(&self, gamepad: i32, button: GamepadButton) -> bool {
        unsafe { ffi::IsGamepadButtonReleased(gamepad, (button as u32) as i32) }
    }

    /// Detect if a gamepad button is NOT being pressed.
    #[inline]
    pub fn is_gamepad_button_up(&self, gamepad: i32, button: GamepadButton) -> bool {
        unsafe { ffi::IsGamepadButtonUp(gamepad, (button as u32) as i32) }
    }

    /// Gets the last gamepad button pressed.
    #[inline]
    pub fn get_gamepad_button_pressed(&self) -> Option<GamepadButton> {
        let button = unsafe { ffi::GetGamepadButtonPressed() };
        if button >= 0 {
            return Some(unsafe { std::mem::transmute(button as u32) });
        }
        None
    }

    /// Returns gamepad axis count for a gamepad.
    #[inline]
    pub fn get_gamepad_axis_count(&self, gamepad: i32) -> i32 {
        unsafe { ffi::GetGamepadAxisCount(gamepad) }
    }

    /// Returns axis movement value for a gamepad axis.
    #[inline]
    pub fn get_gamepad_axis_movement(&self, gamepad: i32, axis: GamepadAxis) -> f32 {
        unsafe { ffi::GetGamepadAxisMovement(gamepad, axis as i32) }
    }

    /// Detect if a mouse button has been pressed once.
    #[inline]
    pub fn is_mouse_button_pressed(&self, button: MouseButton) -> bool {
        unsafe { ffi::IsMouseButtonPressed(button as i32) }
    }

    /// Detect if a mouse button is being pressed.
    #[inline]
    pub fn is_mouse_button_down(&self, button: MouseButton) -> bool {
        unsafe { ffi::IsMouseButtonDown(button as i32) }
    }

    /// Detect if a mouse button has been released once.
    #[inline]
    pub fn is_mouse_button_released(&self, button: MouseButton) -> bool {
        unsafe { ffi::IsMouseButtonReleased(button as i32) }
    }

    /// Detect if a mouse button is NOT being pressed.
    #[inline]
    pub fn is_mouse_button_up(&self, button: MouseButton) -> bool {
        unsafe { ffi::IsMouseButtonUp(button as i32) }
    }

    /// Returns mouse position X.
    #[inline]
    pub fn get_mouse_x(&self) -> i32 {
        unsafe { ffi::GetMouseX() }
    }

    /// Returns mouse position Y.
    #[inline]
    pub fn get_mouse_y(&self) -> i32 {
        unsafe { ffi::GetMouseY() }
    }

    /// Returns mouse position.
    #[inline]
    pub fn get_mouse_position(&self) -> Vector2<f32> {
        unsafe { ffi::GetMousePosition().into() }
    }

    /// Returns mouse delta between frames.
    #[inline]
    pub fn get_mouse_delta(&self) -> Vector2<f32> {
        unsafe { ffi::GetMouseDelta().into() }
    }

    /// Sets mouse position.
    #[inline]
    pub fn set_mouse_position(&self, position: Vector2<f32>) {
        unsafe {
            let Vector2 { x, y } = position;
            ffi::SetMousePosition(x as i32, y as i32);
        }
    }

    /// Sets mouse offset.
    #[inline]
    pub fn set_mouse_offset(&self, offset: Vector2<f32>) {
        unsafe {
            let Vector2 { x, y } = offset;
            ffi::SetMouseOffset(x as i32, y as i32);
        }
    }

    /// Sets mouse scaling.
    #[inline]
    pub fn set_mouse_scale(&self, scale_x: f32, scale_y: f32) {
        unsafe {
            ffi::SetMouseScale(scale_x, scale_y);
        }
    }

    /// Returns mouse wheel movement Y.
    #[inline]
    pub fn get_mouse_wheel_move(&self) -> f32 {
        unsafe { ffi::GetMouseWheelMove() }
    }

    /// Returns touch position X for touch point 0 (relative to screen size).
    #[inline]
    pub fn get_touch_x(&self) -> i32 {
        unsafe { ffi::GetTouchX() }
    }

    /// Returns touch position Y for touch point 0 (relative to screen size).
    #[inline]
    pub fn get_touch_y(&self) -> i32 {
        unsafe { ffi::GetTouchY() }
    }

    /// Returns touch position XY for a touch point index (relative to screen size).
    #[inline]
    pub fn get_touch_position(&self, index: u32) -> Vector2<f32> {
        unsafe { ffi::GetTouchPosition(index as i32).into() }
    }

    /// Enables a set of gestures using flags.
    #[inline]
    pub fn set_gestures_enabled(&self, gesture_flags: u32) {
        unsafe {
            ffi::SetGesturesEnabled(gesture_flags);
        }
    }

    /// Checks if a gesture have been detected.
    #[inline]
    pub fn is_gesture_detected(&self, gesture: Gesture) -> bool {
        unsafe { ffi::IsGestureDetected(gesture as i32) }
    }

    /// Gets latest detected gesture.
    #[inline]
    pub fn get_gesture_detected(&self) -> Gesture {
        unsafe { std::mem::transmute(ffi::GetGestureDetected()) }
    }

    /// Get touch point identifier for given index
    #[inline]
    pub fn get_touch_point_id(&self, index: u32) -> i32 {
        unsafe { ffi::GetTouchPointId(index as i32) }
    }

    /// Gets touch points count.
    #[inline]
    pub fn get_touch_point_count(&self) -> u32 {
        unsafe { ffi::GetTouchPointCount() as u32 }
    }

    /// Gets gesture hold time in milliseconds.
    #[inline]
    pub fn get_gesture_hold_duration(&self) -> f32 {
        unsafe { ffi::GetGestureHoldDuration() }
    }

    /// Gets gesture drag vector.
    #[inline]
    pub fn get_gesture_drag_vector(&self) -> Vector2<f32> {
        unsafe { ffi::GetGestureDragVector().into() }
    }

    /// Gets gesture drag angle.
    #[inline]
    pub fn get_gesture_drag_angle(&self) -> f32 {
        unsafe { ffi::GetGestureDragAngle() }
    }

    /// Gets gesture pinch delta.
    #[inline]
    pub fn get_gesture_pinch_vector(&self) -> Vector2<f32> {
        unsafe { ffi::GetGesturePinchVector().into() }
    }

    /// Gets gesture pinch angle.
    #[inline]
    pub fn get_gesture_pinch_angle(&self) -> f32 {
        unsafe { ffi::GetGesturePinchAngle() }
    }
}

pub fn key_from_i32(key: i32) -> Option<KeyboardKey> {
    use crate::consts::KeyboardKey::*;
    match key {
        39 => Some(KEY_APOSTROPHE),
        44 => Some(KEY_COMMA),
        45 => Some(KEY_MINUS),
        46 => Some(KEY_PERIOD),
        47 => Some(KEY_SLASH),
        48 => Some(KEY_ZERO),
        49 => Some(KEY_ONE),
        50 => Some(KEY_TWO),
        51 => Some(KEY_THREE),
        52 => Some(KEY_FOUR),
        53 => Some(KEY_FIVE),
        54 => Some(KEY_SIX),
        55 => Some(KEY_SEVEN),
        56 => Some(KEY_EIGHT),
        57 => Some(KEY_NINE),
        59 => Some(KEY_SEMICOLON),
        61 => Some(KEY_EQUAL),
        65 => Some(KEY_A),
        66 => Some(KEY_B),
        67 => Some(KEY_C),
        68 => Some(KEY_D),
        69 => Some(KEY_E),
        70 => Some(KEY_F),
        71 => Some(KEY_G),
        72 => Some(KEY_H),
        73 => Some(KEY_I),
        74 => Some(KEY_J),
        75 => Some(KEY_K),
        76 => Some(KEY_L),
        77 => Some(KEY_M),
        78 => Some(KEY_N),
        79 => Some(KEY_O),
        80 => Some(KEY_P),
        81 => Some(KEY_Q),
        82 => Some(KEY_R),
        83 => Some(KEY_S),
        84 => Some(KEY_T),
        85 => Some(KEY_U),
        86 => Some(KEY_V),
        87 => Some(KEY_W),
        88 => Some(KEY_X),
        89 => Some(KEY_Y),
        90 => Some(KEY_Z),
        32 => Some(KEY_SPACE),
        256 => Some(KEY_ESCAPE),
        257 => Some(KEY_ENTER),
        258 => Some(KEY_TAB),
        259 => Some(KEY_BACKSPACE),
        260 => Some(KEY_INSERT),
        261 => Some(KEY_DELETE),
        262 => Some(KEY_RIGHT),
        263 => Some(KEY_LEFT),
        264 => Some(KEY_DOWN),
        265 => Some(KEY_UP),
        266 => Some(KEY_PAGE_UP),
        267 => Some(KEY_PAGE_DOWN),
        268 => Some(KEY_HOME),
        269 => Some(KEY_END),
        280 => Some(KEY_CAPS_LOCK),
        281 => Some(KEY_SCROLL_LOCK),
        282 => Some(KEY_NUM_LOCK),
        283 => Some(KEY_PRINT_SCREEN),
        284 => Some(KEY_PAUSE),
        290 => Some(KEY_F1),
        291 => Some(KEY_F2),
        292 => Some(KEY_F3),
        293 => Some(KEY_F4),
        294 => Some(KEY_F5),
        295 => Some(KEY_F6),
        296 => Some(KEY_F7),
        297 => Some(KEY_F8),
        298 => Some(KEY_F9),
        299 => Some(KEY_F10),
        300 => Some(KEY_F11),
        301 => Some(KEY_F12),
        340 => Some(KEY_LEFT_SHIFT),
        341 => Some(KEY_LEFT_CONTROL),
        342 => Some(KEY_LEFT_ALT),
        343 => Some(KEY_LEFT_SUPER),
        344 => Some(KEY_RIGHT_SHIFT),
        345 => Some(KEY_RIGHT_CONTROL),
        346 => Some(KEY_RIGHT_ALT),
        347 => Some(KEY_RIGHT_SUPER),
        348 => Some(KEY_KB_MENU),
        91 => Some(KEY_LEFT_BRACKET),
        92 => Some(KEY_BACKSLASH),
        93 => Some(KEY_RIGHT_BRACKET),
        96 => Some(KEY_GRAVE),
        320 => Some(KEY_KP_0),
        321 => Some(KEY_KP_1),
        322 => Some(KEY_KP_2),
        323 => Some(KEY_KP_3),
        324 => Some(KEY_KP_4),
        325 => Some(KEY_KP_5),
        326 => Some(KEY_KP_6),
        327 => Some(KEY_KP_7),
        328 => Some(KEY_KP_8),
        329 => Some(KEY_KP_9),
        330 => Some(KEY_KP_DECIMAL),
        331 => Some(KEY_KP_DIVIDE),
        332 => Some(KEY_KP_MULTIPLY),
        333 => Some(KEY_KP_SUBTRACT),
        334 => Some(KEY_KP_ADD),
        335 => Some(KEY_KP_ENTER),
        336 => Some(KEY_KP_EQUAL),
        _ => None,
    }
}
