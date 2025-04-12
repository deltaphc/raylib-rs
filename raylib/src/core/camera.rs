//! Utility code for using Raylib [`Camera3D`] and [`Camera2D`]
use raylib_sys::CameraMode;

use crate::core::math::{Vector2, Vector3};
use crate::core::RaylibHandle;
use crate::ffi;

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
    projection_: ffi::CameraProjection,
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
            projection: (self.projection_ as u32) as i32,
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
        unsafe { std::mem::transmute_copy(&self.projection_) }
    }
    /// Create a perspective camera.
    /// fovy is in degrees
    pub fn perspective(position: Vector3, target: Vector3, up: Vector3, fovy: f32) -> Camera3D {
        Camera3D {
            position,
            target,
            up,
            fovy,
            projection_: ffi::CameraProjection::CAMERA_PERSPECTIVE,
        }
    }
    /// Create a orthographic camera.
    /// fovy is in degrees
    pub fn orthographic(position: Vector3, target: Vector3, up: Vector3, fovy: f32) -> Camera3D {
        let mut c = Self::perspective(position, target, up, fovy);
        c.projection_ = ffi::CameraProjection::CAMERA_ORTHOGRAPHIC;
        c
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
