//! Vr related functions
use crate::core::camera::Camera3D;
use crate::core::RaylibThread;
use crate::ffi;
use std::sync::atomic::{AtomicBool, Ordering};

static IS_INITIALIZED: AtomicBool = AtomicBool::new(false);

/// This token is used to indicate VR is initialized
#[derive(Debug)]
pub struct RaylibVR(());
// #[cfg(feature = "nightly")]
// impl !Send for RaylibVR {}
// #[cfg(feature = "nightly")]
// impl !Sync for RaylibVR {}

impl RaylibVR {
    pub fn init_vr_simulator(_: &RaylibThread) -> RaylibVR {
        if IS_INITIALIZED.load(Ordering::Relaxed) {
            panic!("Attempted to initialize vr mode  more than once");
        } else {
            unsafe {
                ffi::InitVrSimulator();
            }
            IS_INITIALIZED.store(true, Ordering::Relaxed);
            RaylibVR(())
        }
    }
    /// Updates VR tracking (position and orientation) and camera.
    pub fn update_vr_tracking(&mut self, camera: &mut Camera3D) {
        unsafe {
            let mut fficam: ffi::Camera3D = (*camera).into();
            ffi::UpdateVrTracking(&mut fficam);
            *camera = fficam.into();
        }
    }

    /// Set stereo rendering configuration parameters
    pub fn set_vr_configuration(
        &mut self,
        _: &RaylibThread,
        info: ffi::VrDeviceInfo,
        distortion: impl AsRef<ffi::Shader>,
    ) {
        unsafe { ffi::SetVrConfiguration(info, *distortion.as_ref()) }
    }

    /// Detects if VR simulator is ready.
    #[inline]
    pub fn is_vr_simulator_ready(&self) -> bool {
        unsafe { ffi::IsVrSimulatorReady() }
    }

    /// Enables or disables VR experience.
    #[inline]
    pub fn toggle_vr_mode(&self, _: &RaylibThread) {
        unsafe {
            ffi::ToggleVrMode();
        }
    }
}

impl Drop for RaylibVR {
    fn drop(&mut self) {
        unsafe {
            IS_INITIALIZED.store(false, Ordering::Relaxed);
            ffi::CloseVrSimulator()
        }
    }
}
