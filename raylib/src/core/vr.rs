//! Vr related functions
use super::{RaylibHandle, RaylibThread};
use crate::ffi;

make_thin_wrapper!(
    VrStereoConfig,
    ffi::VrStereoConfig,
    ffi::UnloadVrStereoConfig
);

impl RaylibHandle<'_> {
    pub fn load_vr_stereo_config(
        &self,
        _: &RaylibThread,
        device: ffi::VrDeviceInfo,
    ) -> VrStereoConfig {
        VrStereoConfig(unsafe { ffi::LoadVrStereoConfig(device) })
    }
}
