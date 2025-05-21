//! Utility code for using Raylib [`Camera3D`] and [`Camera2D`]
use raylib_sys::{CameraMode, KeyboardKey, MouseButton, GamepadAxis};
use raylib_sys::{RL_CULL_DISTANCE_NEAR, RL_CULL_DISTANCE_FAR, DEG2RAD};

use crate::core::math::{Vector2, Vector3, Quaternion, Matrix};
use crate::core::RaylibHandle;
use crate::ffi;
pub use ffi::CameraProjection;

const CAMERA_CULL_DISTANCE_NEAR: f64 = RL_CULL_DISTANCE_NEAR;
const CAMERA_CULL_DISTANCE_FAR : f64 = RL_CULL_DISTANCE_FAR;

const CAMERA_MOVE_SPEED     : f32 = 0.09;
const CAMERA_ROTATION_SPEED : f32 = 0.03;
const CAMERA_PAN_SPEED      : f32 = 0.2;

// Camera mouse movement sensitivity
const CAMERA_MOUSE_MOVE_SENSITIVITY  : f32 = 0.003;
const CAMERA_MOUSE_SCROLL_SENSITIVITY: f32 = 1.5;

// Radians per second
const CAMERA_ORBITAL_SPEED: f32 = 0.5;       


const CAMERA_FIRST_PERSON_STEP_TRIGONOMETRIC_DIVIDER: f32 =   8.0;
const CAMERA_FIRST_PERSON_STEP_DIVIDER              : f32 =  30.0;
const CAMERA_FIRST_PERSON_WAVING_DIVIDER            : f32 = 200.0;

// PLAYER (used by camera)
const PLAYER_MOVEMENT_SENSITIVITY: f32 = 20.0;


/// Camera, defines position/orientation in 3d space
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Camera3D {
    /// Camera position
    pub position: Vector3,
    /// Camera target it looks-at
    pub target: Vector3,
    /// Camera up vector (rotation over its axis)
    pub up: Vector3,
    /// Camera field-of-view aperture in Y (degrees) in perspective, used as near plane width in orthographic
    pub fovy: f32,
    /// Camera projection: CAMERA_PERSPECTIVE or CAMERA_ORTHOGRAPHIC
    pub projection: CameraProjection,
}
/// Camera type fallback, defaults to Camera3D
pub type Camera = Camera3D;

impl From<ffi::Camera3D> for Camera3D {
    fn from(v: ffi::Camera3D) -> Camera3D {
        unsafe { std::mem::transmute(v) }
    }
}

impl Into<ffi::Camera3D> for Camera3D {
    fn into(self) -> ffi::Camera3D {
        unsafe { std::mem::transmute(self) }
    }
}

impl Into<ffi::Camera3D> for &Camera3D {
    fn into(self) -> ffi::Camera3D {
        ffi::Camera3D {
            position: self.position.into(),
            target: self.target.into(),
            up: self.up.into(),
            fovy: self.fovy,
            projection: (self.projection as u32) as i32,
        }
    }
}

/// Camera2D, defines position/orientation in 2d space
#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct Camera2D {
    /// Camera offset (displacement from target)
    pub offset: Vector2,
    /// Camera target (rotation and zoom origin)
    pub target: Vector2,
    /// Camera rotation in degrees
    pub rotation: f32,
    /// Camera zoom (scaling), should be 1.0 by default
    pub zoom: f32,
}

impl From<ffi::Camera2D> for Camera2D {
    fn from(v: ffi::Camera2D) -> Camera2D {
        unsafe { std::mem::transmute(v) }
    }
}

impl Into<ffi::Camera2D> for Camera2D {
    fn into(self) -> ffi::Camera2D {
        unsafe { std::mem::transmute(self) }
    }
}

impl Into<ffi::Camera2D> for &Camera2D {
    fn into(self) -> ffi::Camera2D {
        ffi::Camera2D {
            offset: self.offset.into(),
            target: self.target.into(),
            rotation: self.rotation,
            zoom: self.zoom,
        }
    }
}

impl Camera3D {
    /// Camera projection: CAMERA_PERSPECTIVE or CAMERA_ORTHOGRAPHIC
    pub const fn camera_type(&self) -> crate::consts::CameraProjection {
        unsafe { std::mem::transmute_copy(&self.projection) }
    }
    /// Create a perspective camera.
    /// fovy is in degrees
    pub fn perspective(position: Vector3, target: Vector3, up: Vector3, fovy: f32) -> Camera3D {
        Camera3D {
            position,
            target,
            up,
            fovy,
            projection: CameraProjection::CAMERA_PERSPECTIVE,
        }
    }
    /// Create a orthographic camera.
    /// fovy is in degrees
    pub fn orthographic(position: Vector3, target: Vector3, up: Vector3, fovy: f32) -> Camera3D {
        let mut c = Self::perspective(position, target, up, fovy);
        c.projection = CameraProjection::CAMERA_ORTHOGRAPHIC;
        c
    }
    /// Returns the cameras forward vector (normalized)
    pub fn get_forward(&self) -> Vector3 {
        (self.target - self.position).normalized()
    }

    /// Returns the cameras up vector (normalized)
    /// Note: The up vector might not be perpendicular to the forward vector
    pub fn get_up(&self) -> Vector3 {
        self.up.normalized()
    }

    /// Returns the cameras right vector (normalized)
    pub fn get_right(&self) -> Vector3 {
        let forward = self.get_forward();
        let up = self.get_up();

        forward.cross(up)
    }

    /// Moves the camera in its forward direction
    pub fn move_forward(&mut self, distance: f32, move_in_world_plane: bool) {
        let mut forward = self.get_forward();

        if move_in_world_plane
        {
            // Project vector onto world plane
            forward.y = 0.;
            forward.normalize();
        }

        // Scale by distance
        forward.scale(distance);

        // Move position and target
        self.position = self.position + forward;
        self.target = self.target + forward;
    }

    /// Moves the camera in its up direction
    pub fn move_up(&mut self, distance: f32) {
        let mut up = self.get_up();

        // Scale by distance
        up.scale(distance);

        // Move position and target
        self.position = self.position + up;
        self.target = self.target + up;
    }

    /// Moves the camera target in its current right direction
    pub fn move_right(&mut self, distance: f32, move_in_world_plane: bool) {
        let mut right = self.get_right();

        if move_in_world_plane
        {
            // Project vector onto world plane
            right.y = 0.;
            right.normalize();
        }

        // Scale by distance
        right.scale(distance);

        // Move position and target
        self.position = self.position + right;
        self.target = self.target + right;
    }

    /// Moves the camera position closer/farther to/from the camera target
    pub fn move_to_target(&mut self, delta: f32){
        let mut distance = self.position.distance_to(self.target);

        // Apply delta
        distance += delta;

        // Distance must be greater than 0
        if distance <= 0. { distance = 0.001; }

        // Set new distance by moving the position along the forward vector
        let forward = self.get_forward();
        self.position = self.target + forward.scale_by(distance);
    }

    /// Rotates the camera around its up vector
    /// Yaw is "looking left and right"
    /// If rotateAroundTarget is false, the camera rotates around its position
    /// Note: angle must be provided in radians
    pub fn set_yaw(&mut self, angle: f32, rotate_around_target: bool) {
        // Rotation axis
        let up = self.get_up();

        // View vector
        let target_position = self.target - self.position;

        // Rotate view vector around up axis
        let target_position = target_position.rotate_by(Quaternion::from_axis_angle(up, angle));

        if rotate_around_target
        {
            // Move position relative to target
            self.position = self.target - target_position;
        }
        else // rotate around camera.position
        {
            // Move target relative to position
            self.target = self.position + target_position;
        }
    }

    /// Rotates the camera around its right vector, pitch is "looking up and down"
    ///  - lockView prevents camera overrotation (aka "somersaults")
    ///  - rotateAroundTarget defines if rotation is around target or around its position
    ///  - rotateUp rotates the up direction as well (typically only usefull in CAMERA_FREE)
    /// NOTE: angle must be provided in radians
    pub fn set_pitch(&mut self, mut angle: f32, lock_view: bool, rotate_around_target: bool, rotate_up: bool) {
        // Up direction
        let up = self.get_up();

        // View vector
        let mut target_position = self.target - self.position;

        if lock_view {
            // In these camera modes we clamp the Pitch angle
            // to allow only viewing straight up or down.

            // Clamp view up
            let mut max_angle_up = up.angle_to(target_position);
            max_angle_up -= 0.001; // avoid numerical errors
            if angle > max_angle_up { angle = max_angle_up };

            // Clamp view down
            let mut max_angle_down = (-up).angle_to(target_position);
            max_angle_down *= -1.; // downwards angle is negative
            max_angle_down += 0.001; // avoid numerical errors
            if angle < max_angle_down { angle = max_angle_down };
        }

        // Rotation axis
        let right = self.get_right();

        // Rotate view vector around right axis
        target_position = target_position.rotate_by(Quaternion::from_axis_angle( right, angle));

        if rotate_around_target
        {
            // Move position relative to target
            self.position = self.target - target_position;
        }
        else // rotate around camera.position
        {
            // Move target relative to position
            self.target = self.position + target_position;
        }

        if rotate_up
        {
            // Rotate up direction around right axis
            self.up = self.up.rotate_by(Quaternion::from_axis_angle(right, angle));
        }
    }

    /// Rotates the camera around its forward vector
    /// Roll is "turning your head sideways to the left or right"
    /// Note: angle must be provided in radians
    pub fn set_roll(&mut self, angle: f32) {
        // Rotation axis
        let forward = self.get_forward();

        // Rotate up direction around forward axis
        self.up = self.up.rotate_by(Quaternion::from_axis_angle(forward, angle));
    }

    /// Returns the camera view matrix
    pub fn get_view_matrix(&self) -> Matrix {
        Matrix::look_at(self.position, self.target, self.up)
    }

    /// Returns the camera projection matrix
    pub fn get_projection_matrix(&self, aspect: f32) -> Matrix {
        if self.projection == CameraProjection::CAMERA_PERSPECTIVE
        {
            return Matrix::perspective(self.fovy * DEG2RAD as f32, aspect, CAMERA_CULL_DISTANCE_NEAR as f32, CAMERA_CULL_DISTANCE_FAR as f32);
        }
        else if self.projection == CameraProjection::CAMERA_ORTHOGRAPHIC
        {
            let top = self.fovy/2.0;
            let right = top*aspect;

            return Matrix::ortho(-right, right, -top, top, CAMERA_CULL_DISTANCE_NEAR as f32, CAMERA_CULL_DISTANCE_FAR as f32);
        }

        return Matrix::identity();
    }

    /// Update camera position for selected mode
    /// Camera mode: CAMERA_FREE, CAMERA_FIRST_PERSON, CAMERA_THIRD_PERSON, CAMERA_ORBITAL or CUSTOM
    pub fn update_camera(&mut self, rl: &mut RaylibHandle, mode: CameraMode) {
        let mouse_position_delta = rl.get_mouse_delta();
        
        let move_in_world_plane = (mode == CameraMode::CAMERA_FIRST_PERSON) || (mode == CameraMode::CAMERA_THIRD_PERSON);
        let rotate_around_target = (mode == CameraMode::CAMERA_THIRD_PERSON) || (mode == CameraMode::CAMERA_ORBITAL);
        let lock_view = (mode == CameraMode::CAMERA_FIRST_PERSON) || (mode == CameraMode::CAMERA_THIRD_PERSON) || (mode == CameraMode::CAMERA_ORBITAL);
        let rotate_up = false;

        if mode == CameraMode::CAMERA_ORBITAL
        {
            // Orbital can just orbit
            let rotation = Matrix::rotate(self.get_up(), CAMERA_ORBITAL_SPEED*rl.get_frame_time());
            let mut view = self.position - self.target;
            view.transform(rotation);
            self.position = self.target + view;
        }
        else
        {
            // Camera rotation
            if rl.is_key_down(KeyboardKey::KEY_DOWN) {self.set_pitch(-CAMERA_ROTATION_SPEED, lock_view, rotate_around_target, rotate_up); }
            if rl.is_key_down(KeyboardKey::KEY_UP) {self.set_pitch(CAMERA_ROTATION_SPEED, lock_view, rotate_around_target, rotate_up);}
            if rl.is_key_down(KeyboardKey::KEY_RIGHT) {self.set_yaw(-CAMERA_ROTATION_SPEED, rotate_around_target);}
            if rl.is_key_down(KeyboardKey::KEY_LEFT) {self.set_yaw(CAMERA_ROTATION_SPEED, rotate_around_target);}
            if rl.is_key_down(KeyboardKey::KEY_Q) {self.set_roll(-CAMERA_ROTATION_SPEED);}
            if rl.is_key_down(KeyboardKey::KEY_E) {self.set_roll(CAMERA_ROTATION_SPEED);}

            // Camera movement
            if !rl.is_gamepad_available(0)
            {
                // Camera pan (for CAMERA_FREE)
                if (mode == CameraMode::CAMERA_FREE) && (rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_MIDDLE))
                {
                    let mouse_delta = rl.get_mouse_delta();
                    if mouse_delta.x > 0.0 { self.move_right(CAMERA_PAN_SPEED, move_in_world_plane);  }
                    if mouse_delta.x < 0.0 { self.move_right(-CAMERA_PAN_SPEED, move_in_world_plane); }
                    if mouse_delta.y > 0.0 { self.move_up(-CAMERA_PAN_SPEED);}
                    if mouse_delta.y < 0.0 { self.move_up(CAMERA_PAN_SPEED);}
                }
                else
                {
                    // Mouse support
                    self.set_yaw(-mouse_position_delta.x*CAMERA_MOUSE_MOVE_SENSITIVITY, rotate_around_target);
                    self.set_pitch(-mouse_position_delta.y*CAMERA_MOUSE_MOVE_SENSITIVITY, lock_view, rotate_around_target, rotate_up);
                }

                // Keyboard support
                if rl.is_key_down(KeyboardKey::KEY_W) { self.move_forward(CAMERA_MOVE_SPEED, move_in_world_plane);}
                if rl.is_key_down(KeyboardKey::KEY_A) { self.move_right(-CAMERA_MOVE_SPEED, move_in_world_plane);}
                if rl.is_key_down(KeyboardKey::KEY_S) { self.move_forward(-CAMERA_MOVE_SPEED, move_in_world_plane);}
                if rl.is_key_down(KeyboardKey::KEY_D) { self.move_right(CAMERA_MOVE_SPEED, move_in_world_plane);    }
            }
            else
            {
                // Gamepad controller support
                self.set_yaw(-(rl.get_gamepad_axis_movement(0, GamepadAxis::GAMEPAD_AXIS_RIGHT_X) * 2.)*CAMERA_MOUSE_MOVE_SENSITIVITY, rotate_around_target);
                self.set_pitch(-(rl.get_gamepad_axis_movement(0, GamepadAxis::GAMEPAD_AXIS_RIGHT_Y) * 2.)*CAMERA_MOUSE_MOVE_SENSITIVITY, lock_view, rotate_around_target, rotate_up);

                if rl.get_gamepad_axis_movement(0, GamepadAxis::GAMEPAD_AXIS_LEFT_Y) <= -0.25 { self.move_forward(CAMERA_MOVE_SPEED, move_in_world_plane);}
                if rl.get_gamepad_axis_movement(0, GamepadAxis::GAMEPAD_AXIS_LEFT_X) <= -0.25 { self.move_right(-CAMERA_MOVE_SPEED, move_in_world_plane);}
                if rl.get_gamepad_axis_movement(0, GamepadAxis::GAMEPAD_AXIS_LEFT_Y) >= 0.25  { self.move_forward(-CAMERA_MOVE_SPEED, move_in_world_plane);}
                if rl.get_gamepad_axis_movement(0, GamepadAxis::GAMEPAD_AXIS_LEFT_X) >= 0.25  { self.move_right(CAMERA_MOVE_SPEED, move_in_world_plane);     }
            }

            if mode == CameraMode::CAMERA_FREE
            {
                if rl.is_key_down(KeyboardKey::KEY_SPACE) { self.move_up(CAMERA_MOVE_SPEED); }
                if rl.is_key_down(KeyboardKey::KEY_LEFT_CONTROL) { self.move_up(-CAMERA_MOVE_SPEED); }
            }
        }

        if (mode == CameraMode::CAMERA_THIRD_PERSON) || (mode == CameraMode::CAMERA_ORBITAL) || (mode == CameraMode::CAMERA_FREE)
        {
            // Zoom target distance
            self.move_to_target(-rl.get_mouse_wheel_move());
            if rl.is_key_pressed(KeyboardKey::KEY_KP_SUBTRACT) {self.move_to_target(2.0);}
            if rl.is_key_pressed(KeyboardKey::KEY_KP_ADD) {self.move_to_target(-2.0); }
        }
    }

    /// Update camera movement, movement/rotation values should be provided by user
    pub fn update_camera_pro(&mut self, movement: Vector3, rotation: Vector3, zoom: f32) {
        // Required values
        // movement.x - Move forward/backward
        // movement.y - Move right/left
        // movement.z - Move up/down
        // rotation.x - yaw
        // rotation.y - pitch
        // rotation.z - roll
        // zoom - Move towards target

        let lock_view = true;
        let rotate_around_target = false;
        let rotate_up = false;
        let move_in_world_plane = true;

        // Camera rotation
        self.set_pitch(-rotation.y*DEG2RAD as f32, lock_view, rotate_around_target, rotate_up);
        self.set_yaw(-rotation.x*DEG2RAD as f32, rotate_around_target);
        self.set_roll(rotation.z*DEG2RAD as f32);

        // Camera movement
        self.move_forward(movement.x, move_in_world_plane);
        self.move_right(movement.y, move_in_world_plane);
        self.move_up(movement.z);

        // Zoom target distance
        self.move_to_target(zoom);
    }

}

impl RaylibHandle {
    /// Updates camera position for selected mode.
    #[inline]
    pub fn update_camera(&self, camera: &mut Camera3D, mode: CameraMode) {
        unsafe {
            let mut fficam: ffi::Camera3D = (*camera).into();
            ffi::UpdateCamera(&mut fficam, mode as i32);
            *camera = fficam.into();
        }
    }

    /// Update camera movement/rotation
    pub fn update_camera_pro(&self, camera: &mut Camera3D, movement: Vector3, rotation: Vector3, zoom: f32) {
        unsafe {
            let mut fficam: ffi::Camera3D = (*camera).into();
            ffi::UpdateCameraPro(&mut fficam, movement.into(), rotation.into(), zoom);
            *camera = fficam.into();
        }
    }
}
