//! VR related functions
use crate::core::{RaylibHandle, RaylibThread};
use crate::ffi;

make_thin_wrapper!(
    /// VrStereoConfig, VR stereo rendering configuration for simulator
    VrStereoConfig,
    ffi::VrStereoConfig,
    ffi::UnloadVrStereoConfig
);

/// VrDeviceInfo, Head-Mounted-Display device parameters
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VrDeviceInfo {
    /// Horizontal resolution in pixels
    pub h_resolution: i32,
    /// Vertical resolution in pixels
    pub v_esolution: i32,
    /// Horizontal size in meters
    pub h_screen_size: f32,
    /// Vertical size in meters
    pub v_screen_size: f32,
    /// Distance between eye and display in meters
    pub eye_to_screen_distance: f32,
    /// Lens separation distance in meters
    pub lens_separation_distance: f32,
    /// IPD (distance between pupils) in meters
    pub interpupillary_distance: f32,
    /// Lens distortion constant parameters
    pub lens_distortion_values: [f32; 4],
    /// Chromatic aberration correction parameters
    pub chroma_ab_correction: [f32; 4],
}

impl From<ffi::VrDeviceInfo> for VrDeviceInfo {
    fn from(v: ffi::VrDeviceInfo) -> VrDeviceInfo {
        unsafe { std::mem::transmute(v) }
    }
}

impl Into<ffi::VrDeviceInfo> for VrDeviceInfo {
    fn into(self) -> ffi::VrDeviceInfo {
        unsafe { std::mem::transmute(self) }
    }
}

impl Into<ffi::VrDeviceInfo> for &VrDeviceInfo {
    fn into(self) -> ffi::VrDeviceInfo {
        ffi::VrDeviceInfo {
            hResolution: self.h_resolution,  // Horizontal resolution in pixels
            vResolution: self.v_esolution,   // Vertical resolution in pixels
            hScreenSize: self.h_screen_size, // Horizontal size in meters
            vScreenSize: self.v_screen_size, // Vertical size in meters
            eyeToScreenDistance: self.eye_to_screen_distance, // Distance between eye and display in meters
            lensSeparationDistance: self.lens_separation_distance, // Lens separation distance in meters
            interpupillaryDistance: self.interpupillary_distance, // IPD (distance between pupils) in meters
            lensDistortionValues: self.lens_distortion_values, // Lens distortion constant parameters
            chromaAbCorrection: self.chroma_ab_correction, // Chromatic aberration correction parameters
        }
    }
}

impl RaylibHandle {
    /// Load VR stereo config for VR simulator device parameters
    #[inline]
    pub fn load_vr_stereo_config(
        &mut self,
        _: &RaylibThread,
        device: impl Into<ffi::VrDeviceInfo>,
    ) -> VrStereoConfig {
        VrStereoConfig(unsafe { ffi::LoadVrStereoConfig(device.into()) })
    }
}
