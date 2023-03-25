//! Utility code for using Raylib [`Camera3D`] and [`Camera2D`]
use mint::Vector3;
use raylib_sys::{Camera3D, CameraMode};

use super::RaylibHandle;
use crate::ffi;

impl RaylibHandle<'_> {
    /// Updates camera position for selected mode.
    #[inline]
    pub fn update_camera(&self, camera: &mut Camera3D, mode: CameraMode) {
        unsafe { ffi::UpdateCamera(camera, mode as i32) }
    }

    pub fn update_camera_pro(
        &self,
        camera: &mut Camera3D,
        movement: Vector3<f32>,
        rotation: Vector3<f32>,
        zoom: f32,
    ) {
        unsafe { ffi::UpdateCameraPro(camera, movement.into(), rotation.into(), zoom) }
    }
}
