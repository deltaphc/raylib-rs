//! Utility code for using Raylib [`Camera3D`] and [`Camera2D`]
use raylib_sys::CameraMode;

use crate::core::math::{Vector2, Vector3};
use crate::core::RaylibHandle;
use crate::ffi;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Camera3D {
    pub position: Vector3,
    pub target: Vector3,
    pub up: Vector3,
    pub fovy: f32,
    projection_: ffi::CameraProjection,
}
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

#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct Camera2D {
    pub offset: Vector2,
    pub target: Vector2,
    pub rotation: f32,
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
    pub fn camera_type(&self) -> crate::consts::CameraProjection {
        unsafe { std::mem::transmute(self.projection_.clone()) }
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
    
    pub fn update_camera_pro(&self, camera: &mut Camera3D, movement: Vector3, rotation: Vector3, zoom: f32) {
        unsafe {
            let mut fficam: ffi::Camera3D = (*camera).into();
            ffi::UpdateCameraPro(&mut fficam, movement.into(), rotation.into(), zoom);
            *camera = fficam.into();
        }
    }
}
