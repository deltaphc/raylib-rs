use std::mem::transmute;

use super::{Camera3D, CameraProjection};
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

    /*
        RLAPI Vector3 GetCameraForward(Camera *camera);
    RLAPI Vector3 GetCameraUp(Camera *camera);
    RLAPI Vector3 GetCameraRight(Camera *camera);

    // Camera movement
    RLAPI void CameraMoveForward(Camera *camera, float distance, bool moveInWorldPlane);
    RLAPI void CameraMoveUp(Camera *camera, float distance);
    RLAPI void CameraMoveRight(Camera *camera, float distance, bool moveInWorldPlane);
    RLAPI void CameraMoveToTarget(Camera *camera, float delta);

    // Camera rotation
    RLAPI void CameraYaw(Camera *camera, float angle, bool rotateAroundTarget);
    RLAPI void CameraPitch(Camera *camera, float angle, bool lockView, bool rotateAroundTarget, bool rotateUp);
    RLAPI void CameraRoll(Camera *camera, float angle);

    RLAPI Matrix GetCameraViewMatrix(Camera *camera);
    RLAPI Matrix GetCameraProjectionMatrix(Camera* camera, float aspect);
     */

    pub fn forward(&self) -> Vector3<f32> {
        unsafe { super::GetCameraForward(self as *const _ as *mut _).into() }
    }
}
