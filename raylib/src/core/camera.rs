use crate::core::*;
use crate::ffi;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Camera3D {
    pub position: Vector3,
    pub target: Vector3,
    pub up: Vector3,
    pub fovy: f32,
    type_: ffi::CameraType,
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
            type_: (self.type_ as u32) as i32,
        }
    }
}

impl Camera3D {
    pub fn perspective(position: Vector3, target: Vector3, up: Vector3, fovy: f32) -> Camera3D {
        Camera3D {
            position,
            target,
            up,
            fovy,
            type_: ffi::CameraType::CAMERA_PERSPECTIVE,
        }
    }
    pub fn orthographic(position: Vector3, target: Vector3, up: Vector3, fovy: f32) -> Camera3D {
        let mut c = Self::perspective(position, target, up, fovy);
        c.type_ = ffi::CameraType::CAMERA_ORTHOGRAPHIC;
        c
    }
}

impl RaylibHandle {
    /// Sets camera mode.
    #[inline]
    pub fn set_camera_mode(&mut self, camera: &Camera3D, mode: crate::consts::CameraMode) {
        unsafe {
            ffi::SetCameraMode(camera.into(), mode as i32);
        }
    }

    /// Updates camera position for selected mode.
    #[inline]
    pub fn update_camera(&self, camera: &mut Camera3D) {
        unsafe {
            let mut fficam: ffi::Camera3D = (*camera).into();
            ffi::UpdateCamera(&mut fficam);
            *camera = fficam.into();
        }
    }

    /// Sets camera pan key to combine with mouse movement (free camera).
    #[inline]
    pub fn set_camera_pan_control(&mut self, pan_key: crate::consts::KeyboardKey) {
        unsafe {
            ffi::SetCameraPanControl(pan_key as i32);
        }
    }

    /// Sets camera alt key to combine with mouse movement (free camera).
    #[inline]
    pub fn set_camera_alt_control(&mut self, alt_key: crate::consts::KeyboardKey) {
        unsafe {
            ffi::SetCameraAltControl(alt_key as i32);
        }
    }

    /// Sets camera smooth zoom key to combine with mouse (free camera).
    #[inline]
    pub fn set_camera_smooth_zoom_control(&mut self, sz_key: crate::consts::KeyboardKey) {
        unsafe {
            ffi::SetCameraSmoothZoomControl(sz_key as i32);
        }
    }

    /// Sets camera move controls (1st person and 3rd person cameras).
    #[inline]
    pub fn set_camera_move_controls(
        &mut self,
        front_key: crate::consts::KeyboardKey,
        back_key: crate::consts::KeyboardKey,
        right_key: crate::consts::KeyboardKey,
        left_key: crate::consts::KeyboardKey,
        up_key: crate::consts::KeyboardKey,
        down_key: crate::consts::KeyboardKey,
    ) {
        unsafe {
            ffi::SetCameraMoveControls(
                front_key as i32,
                back_key as i32,
                right_key as i32,
                left_key as i32,
                up_key as i32,
                down_key as i32,
            );
        }
    }
}
