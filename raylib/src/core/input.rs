use crate::consts::GestureType;
use crate::core::math::Vector2;
use crate::core::RaylibHandle;
use crate::ffi;

use std::ffi::{CStr, CString};

impl RaylibHandle {
    /// Detect if a key has been pressed once.
    #[inline]
    pub fn is_key_pressed(&self, key: crate::consts::KeyboardKey) -> bool {
        unsafe { ffi::IsKeyPressed((key as u32) as i32) }
    }

    /// Detect if a key is being pressed.
    #[inline]
    pub fn is_key_down(&self, key: crate::consts::KeyboardKey) -> bool {
        unsafe { ffi::IsKeyDown((key as u32) as i32) }
    }

    /// Detect if a key has been released once.
    #[inline]
    pub fn is_key_released(&self, key: crate::consts::KeyboardKey) -> bool {
        unsafe { ffi::IsKeyReleased((key as u32) as i32) }
    }

    /// Detect if a key is NOT being pressed.
    #[inline]
    pub fn is_key_up(&self, key: crate::consts::KeyboardKey) -> bool {
        unsafe { ffi::IsKeyUp((key as u32) as i32) }
    }

    /// Gets latest key pressed.
    #[inline]
    pub fn get_key_pressed(&self) -> Option<crate::consts::KeyboardKey> {
        let key = unsafe { ffi::GetKeyPressed() };
        if key > 0 {
            return Some(unsafe { std::mem::transmute(key as u32) });
        }
        None
    }

    /// Sets a custom key to exit program (default is ESC).
    #[inline]
    pub fn set_exit_key(&mut self, key: crate::consts::KeyboardKey) {
        unsafe {
            ffi::SetExitKey((key as u32) as i32);
        }
    }

    /// Detect if a gamepad is available.
    #[inline]
    pub fn is_gamepad_available(&self, gamepad: u32) -> bool {
        unsafe { ffi::IsGamepadAvailable(gamepad as i32) }
    }

    /// Checks gamepad name (if available).
    #[inline]
    pub fn is_gamepad_name(&self, gamepad: u32, name: &str) -> bool {
        let c_name = CString::new(name).unwrap();
        unsafe { ffi::IsGamepadName(gamepad as i32, c_name.as_ptr()) }
    }

    /// Returns gamepad internal name id.
    #[inline]
    pub fn get_gamepad_name(&self, gamepad: u32) -> Option<String> {
        unsafe {
            let name = ffi::GetGamepadName(gamepad as i32);
            match name.is_null() {
                false => Some(CStr::from_ptr(name).to_str().unwrap().to_owned()),
                true => None,
            }
        }
    }

    /// Detect if a gamepad button has been pressed once.
    #[inline]
    pub fn is_gamepad_button_pressed(
        &self,
        gamepad: u32,
        button: crate::consts::GamepadButton,
    ) -> bool {
        unsafe { ffi::IsGamepadButtonPressed(gamepad as i32, (button as u32) as i32) }
    }

    /// Detect if a gamepad button is being pressed.
    #[inline]
    pub fn is_gamepad_button_down(
        &self,
        gamepad: u32,
        button: crate::consts::GamepadButton,
    ) -> bool {
        unsafe { ffi::IsGamepadButtonDown(gamepad as i32, (button as u32) as i32) }
    }

    /// Detect if a gamepad button has been released once.
    #[inline]
    pub fn is_gamepad_button_released(
        &self,
        gamepad: u32,
        button: crate::consts::GamepadButton,
    ) -> bool {
        unsafe { ffi::IsGamepadButtonReleased(gamepad as i32, (button as u32) as i32) }
    }

    /// Detect if a gamepad button is NOT being pressed.
    #[inline]
    pub fn is_gamepad_button_up(&self, gamepad: u32, button: crate::consts::GamepadButton) -> bool {
        unsafe { ffi::IsGamepadButtonUp(gamepad as i32, (button as u32) as i32) }
    }

    /// Gets the last gamepad button pressed.
    #[inline]
    pub fn get_gamepad_button_pressed(&self) -> Option<crate::consts::GamepadButton> {
        let button = unsafe { ffi::GetGamepadButtonPressed() };
        if button >= 0 {
            return Some(unsafe { std::mem::transmute(button as u32) });
        }
        None
    }

    /// Returns gamepad axis count for a gamepad.
    #[inline]
    pub fn get_gamepad_axis_count(&self, gamepad: u32) -> i32 {
        unsafe { ffi::GetGamepadAxisCount(gamepad as i32) }
    }

    /// Returns axis movement value for a gamepad axis.
    #[inline]
    pub fn get_gamepad_axis_movement(&self, gamepad: u32, axis: u32) -> f32 {
        unsafe { ffi::GetGamepadAxisMovement(gamepad as i32, axis as i32) }
    }

    /// Detect if a mouse button has been pressed once.
    #[inline]
    pub fn is_mouse_button_pressed(&self, button: crate::consts::MouseButton) -> bool {
        unsafe { ffi::IsMouseButtonPressed(button as i32) }
    }

    /// Detect if a mouse button is being pressed.
    #[inline]
    pub fn is_mouse_button_down(&self, button: crate::consts::MouseButton) -> bool {
        unsafe { ffi::IsMouseButtonDown(button as i32) }
    }

    /// Detect if a mouse button has been released once.
    #[inline]
    pub fn is_mouse_button_released(&self, button: crate::consts::MouseButton) -> bool {
        unsafe { ffi::IsMouseButtonReleased(button as i32) }
    }

    /// Detect if a mouse button is NOT being pressed.
    #[inline]
    pub fn is_mouse_button_up(&self, button: crate::consts::MouseButton) -> bool {
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
    pub fn get_mouse_position(&self) -> Vector2 {
        unsafe { ffi::GetMousePosition().into() }
    }

    /// Sets mouse position.
    #[inline]
    pub fn set_mouse_position(&mut self, position: impl Into<Vector2>) {
        unsafe {
            let Vector2 { x, y } = position.into();
            ffi::SetMousePosition(x as i32, y as i32);
        }
    }

    /// Sets mouse scaling.
    #[inline]
    pub fn set_mouse_scale(&mut self, scale_x: f32, scale_y: f32) {
        unsafe {
            ffi::SetMouseScale(scale_x, scale_y);
        }
    }

    /// Returns mouse wheel movement Y.
    #[inline]
    pub fn get_mouse_wheel_move(&self) -> i32 {
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
    pub fn get_touch_position(&self, index: u32) -> Vector2 {
        unsafe { ffi::GetTouchPosition(index as i32).into() }
    }

    /// Enables a set of gestures using flags.
    #[inline]
    pub fn set_gestures_enabled(&self, gesture_flags: u32) {
        unsafe {
            ffi::SetGesturesEnabled(gesture_flags as u32);
        }
    }

    /// Checks if a gesture have been detected.
    #[inline]
    pub fn is_gesture_detected(&self, gesture: GestureType) -> bool {
        unsafe { ffi::IsGestureDetected(gesture as i32) }
    }

    /// Gets latest detected gesture.
    #[inline]
    pub fn get_gesture_detected(&self) -> u32 {
        unsafe { ffi::GetGestureDetected() as u32 }
    }

    /// Gets touch points count.
    #[inline]
    pub fn get_touch_points_count(&self) -> u32 {
        unsafe { ffi::GetTouchPointsCount() as u32 }
    }

    /// Gets gesture hold time in milliseconds.
    #[inline]
    pub fn get_gesture_hold_duration(&self) -> f32 {
        unsafe { ffi::GetGestureHoldDuration() }
    }

    /// Gets gesture drag vector.
    #[inline]
    pub fn get_gesture_drag_vector(&self) -> Vector2 {
        unsafe { ffi::GetGestureDragVector().into() }
    }

    /// Gets gesture drag angle.
    #[inline]
    pub fn get_gesture_drag_angle(&self) -> f32 {
        unsafe { ffi::GetGestureDragAngle() }
    }

    /// Gets gesture pinch delta.
    #[inline]
    pub fn get_gesture_pinch_vector(&self) -> Vector2 {
        unsafe { ffi::GetGesturePinchVector().into() }
    }

    /// Gets gesture pinch angle.
    #[inline]
    pub fn get_gesture_pinch_angle(&self) -> f32 {
        unsafe { ffi::GetGesturePinchAngle() }
    }
}
