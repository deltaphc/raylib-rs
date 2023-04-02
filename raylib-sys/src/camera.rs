use std::mem::transmute;

use super::{Matrix, Camera3D, CameraProjection};
use mint::Vector3;

impl Camera3D {
    pub fn camera_type(&self) -> CameraProjection {
        unsafe { transmute(self.projection as u32) }
    }
    /// Create a perspective camera.
    /// fovy is in degrees
    pub fn perspective(
        position: Vector3<f32>,
        target: Vector3<f32>,
        up: Vector3<f32>,
        fovy: f32,
    ) -> Camera3D {
        Camera3D {
            position: position.into(),
            target: target.into(),
            up: up.into(),
            fovy,
            projection: CameraProjection::CAMERA_PERSPECTIVE as i32,
        }
    }
    /// Create a orthographic camera.
    /// fovy is in degrees
    pub fn orthographic(
        position: Vector3<f32>,
        target: Vector3<f32>,
        up: Vector3<f32>,
        fovy: f32,
    ) -> Camera3D {
        let mut c = Self::perspective(position, target, up, fovy);
        c.projection = CameraProjection::CAMERA_ORTHOGRAPHIC as i32;
        c
    }

    pub fn forward(&self) -> Vector3<f32> {
        unsafe { super::GetCameraForward(self as *const _ as *mut _).into() }
    }

    pub fn up(&self) -> Vector3<f32> {
        unsafe { super::GetCameraUp(self as *const _ as *mut _).into() }
    }

    pub fn move_forward(&mut self, distance: f32, in_world_plane: bool) {
        unsafe { super::CameraMoveForward(self, distance, in_world_plane) }
    }

    pub fn move_up(&mut self, distance: f32) {
        unsafe { super::CameraMoveUp(self, distance) }
    }

    pub fn move_right(&mut self, distance: f32, in_world_plane: bool) {
        unsafe { super::CameraMoveRight(self, distance, in_world_plane) }
    }

    pub fn move_to_target(&mut self, delta: f32) {
        unsafe { super::CameraMoveToTarget(self, delta) }
    }

    pub fn yaw(&mut self, angle: f32, rotate_around_target: bool) {
        unsafe { super::CameraYaw(self, angle, rotate_around_target) }
    }

    pub fn pitch(
        &mut self,
        angle: f32,
        lock_view: bool,
        rotate_around_target: bool,
        rotate_up: bool,
    ) {
        unsafe { super::CameraPitch(self, angle, lock_view, rotate_around_target, rotate_up) }
    }

    pub fn roll(&mut self, angle: f32) {
        unsafe { super::CameraRoll(self, angle) }
    }

    pub fn view_matrix(&self) -> Matrix {
        unsafe { super::GetCameraViewMatrix(self as *const _ as *mut _) }
    }

    pub fn projection_matrix(&self, aspect: f32) -> Matrix {
        unsafe { super::GetCameraProjectionMatrix(self as *const _ as *mut _, aspect) }
    }
}
