use crate::core::math::Matrix;
use crate::core::{RaylibHandle, RaylibThread};
use crate::ffi;
use std::ffi::CString;

fn no_drop<T>(_thing: T) {}
make_thin_wrapper!(Shader, ffi::Shader, ffi::UnloadShader);
/// WeakShader can be sent between threads, but will be leak memory if
/// unload_material is not called on it.
/// has nothing to prevent dataraces when cloned
make_thin_wrapper!(WeakShader, ffi::Shader, no_drop);

#[cfg(feature = "nightly")]
impl !Send for Shader {}
#[cfg(feature = "nightly")]
unsafe impl Sync for Shader {}

impl RaylibHandle {
    /// Loads a custom shader and binds default locations.
    #[inline]
    pub fn load_shader(
        &mut self,
        _: &RaylibThread,
        vs_filename: Option<&str>,
        fs_filename: Option<&str>,
    ) -> Result<Shader, String> {
        if let Some(f) = vs_filename {
            if !crate::core::file::file_exists(f) {
                return Err(format!("could not load shader file {}", f));
            }
        }
        if let Some(f) = fs_filename {
            if !crate::core::file::file_exists(f) {
                return Err(format!("could not load shader file {}", f));
            }
        }
        let c_vs_filename = CString::new(vs_filename.unwrap_or("")).unwrap();
        let c_fs_filename = CString::new(fs_filename.unwrap_or("")).unwrap();
        unsafe {
            Ok(Shader(ffi::LoadShader(
                c_vs_filename.as_ptr(),
                c_fs_filename.as_ptr(),
            )))
        }
    }

    /// Loads shader from code strings and binds default locations.
    #[inline]
    pub fn load_shader_code(
        &mut self,
        _: &RaylibThread,
        vs_code: Option<&str>,
        fs_code: Option<&str>,
    ) -> Shader {
        let c_vs_code = CString::new(vs_code.unwrap_or("")).unwrap();
        let c_fs_code = CString::new(fs_code.unwrap_or("")).unwrap();
        unsafe {
            Shader(ffi::LoadShaderCode(
                c_vs_code.as_ptr() as *mut i8,
                c_fs_code.as_ptr() as *mut i8,
            ))
        }
    }
}

impl Shader {
    /// Sets shader uniform value (`f32`).
    #[inline]
    fn set_shader_value(&mut self, uniform_loc: i32, value: &[f32]) {
        unsafe {
            ffi::SetShaderValue(
                self.0,
                uniform_loc,
                value.as_ptr() as *const ::std::os::raw::c_void,
                value.len() as i32,
            );
        }
    }

    /// Sets shader uniform value (matrix 4x4).
    #[inline]
    fn set_shader_value_matrix(&mut self, uniform_loc: i32, mat: Matrix) {
        unsafe {
            ffi::SetShaderValueMatrix(self.0, uniform_loc, mat.into());
        }
    }
}

impl RaylibShader for WeakShader {}
impl RaylibShader for Shader {}

trait RaylibShader: AsRef<ffi::Shader> {
    /// Gets shader uniform location by name.
    #[inline]
    fn get_shader_location(&self, uniform_name: &str) -> i32 {
        let c_uniform_name = CString::new(uniform_name).unwrap();
        unsafe { ffi::GetShaderLocation(*self.as_ref(), c_uniform_name.as_ptr()) }
    }
}

impl RaylibHandle {
    /// Sets a custom projection matrix (replaces internal projection matrix).
    #[inline]
    pub fn set_matrix_projection(&mut self, proj: Matrix) {
        unsafe {
            ffi::SetMatrixProjection(proj.into());
        }
    }

    /// Sets a custom modelview matrix (replaces internal modelview matrix).
    #[inline]
    pub fn set_matrix_modelview(&mut self, view: Matrix) {
        unsafe {
            ffi::SetMatrixModelview(view.into());
        }
    }

    /// Gets internal modelview matrix.
    #[inline]
    pub fn get_matrix_modelview(&self) -> Matrix {
        unsafe { ffi::GetMatrixModelview().into() }
    }
}
