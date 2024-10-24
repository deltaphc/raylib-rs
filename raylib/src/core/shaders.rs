//! Code for the safe manipulation of shaders
use thiserror::Error;

use crate::consts::ShaderUniformDataType;
use crate::core::math::Matrix;
use crate::core::math::{Vector2, Vector3, Vector4};
use crate::core::{RaylibHandle, RaylibThread};
use crate::ffi;
use std::ffi::CString;
use std::os::raw::{c_char, c_void};

fn no_drop<T>(_thing: T) {}
make_thin_wrapper!(Shader, ffi::Shader, ffi::UnloadShader);
make_thin_wrapper!(WeakShader, ffi::Shader, no_drop);

// #[cfg(feature = "nightly")]
// impl !Send for Shader {}
// #[cfg(feature = "nightly")]
// unsafe impl Sync for Shader {}

impl RaylibHandle {
    /// Loads a custom shader and binds default locations.
    pub fn load_shader(
        &mut self,
        _: &RaylibThread,
        vs_filename: Option<&str>,
        fs_filename: Option<&str>,
    ) -> Shader {
        let c_vs_filename = vs_filename.map(|f| CString::new(f).unwrap());
        let c_fs_filename = fs_filename.map(|f| CString::new(f).unwrap());

        // Trust me, I have tried ALL the RUST option ergonamics. This is the only way
        // to get this to work without raylib breaking for whatever reason
        // UPDATE FOR 2024 FROM ANOTHER PERSON: Yes this is still true, doing although "for some reason" is likely due to the pointer getting freed too early if you don't do it this way.
        let shader = match (c_vs_filename, c_fs_filename) {
            (Some(vs), Some(fs)) => unsafe { Shader(ffi::LoadShader(vs.as_ptr(), fs.as_ptr())) },
            (None, Some(fs)) => unsafe { Shader(ffi::LoadShader(std::ptr::null(), fs.as_ptr())) },
            (Some(vs), None) => unsafe { Shader(ffi::LoadShader(vs.as_ptr(), std::ptr::null())) },
            (None, None) => unsafe { Shader(ffi::LoadShader(std::ptr::null(), std::ptr::null())) },
        };

        return shader;
    }

    /// Loads shader from code strings and binds default locations.
    pub fn load_shader_from_memory(
        &mut self,
        _: &RaylibThread,
        vs_code: Option<&str>,
        fs_code: Option<&str>,
    ) -> Shader {
        let c_vs_code = vs_code.map(|f| CString::new(f).unwrap());
        let c_fs_code = fs_code.map(|f| CString::new(f).unwrap());
        return match (c_vs_code, c_fs_code) {
            (Some(vs), Some(fs)) => unsafe {
                Shader(ffi::LoadShaderFromMemory(
                    vs.as_ptr() as *mut c_char,
                    fs.as_ptr() as *mut c_char,
                ))
            },
            (None, Some(fs)) => unsafe {
                Shader(ffi::LoadShaderFromMemory(
                    std::ptr::null_mut(),
                    fs.as_ptr() as *mut c_char,
                ))
            },
            (Some(vs), None) => unsafe {
                Shader(ffi::LoadShaderFromMemory(
                    vs.as_ptr() as *mut c_char,
                    std::ptr::null_mut(),
                ))
            },
            (None, None) => unsafe {
                Shader(ffi::LoadShaderFromMemory(
                    std::ptr::null_mut(),
                    std::ptr::null_mut(),
                ))
            },
        };
    }

    /// Get default shader. Modifying it modifies everthing that uses that shader
    #[cfg(target_os = "windows")]
    pub fn get_shader_default() -> WeakShader {
        unsafe {
            WeakShader(ffi::Shader {
                id: ffi::rlGetShaderIdDefault(),
                locs: ffi::rlGetShaderLocsDefault(),
            })
        }
    }
}

pub trait ShaderV {
    const UNIFORM_TYPE: ShaderUniformDataType;
    unsafe fn value(&self) -> *const c_void;
}

impl ShaderV for f32 {
    const UNIFORM_TYPE: ShaderUniformDataType = ShaderUniformDataType::SHADER_UNIFORM_FLOAT;
    unsafe fn value(&self) -> *const c_void {
        self as *const f32 as *const c_void
    }
}

impl ShaderV for Vector2 {
    const UNIFORM_TYPE: ShaderUniformDataType = ShaderUniformDataType::SHADER_UNIFORM_VEC2;
    unsafe fn value(&self) -> *const c_void {
        self as *const Vector2 as *const c_void
    }
}

impl ShaderV for Vector3 {
    const UNIFORM_TYPE: ShaderUniformDataType = ShaderUniformDataType::SHADER_UNIFORM_VEC3;
    unsafe fn value(&self) -> *const c_void {
        self as *const Vector3 as *const c_void
    }
}

impl ShaderV for Vector4 {
    const UNIFORM_TYPE: ShaderUniformDataType = ShaderUniformDataType::SHADER_UNIFORM_VEC4;
    unsafe fn value(&self) -> *const c_void {
        self as *const Vector4 as *const c_void
    }
}

impl ShaderV for i32 {
    const UNIFORM_TYPE: ShaderUniformDataType = ShaderUniformDataType::SHADER_UNIFORM_INT;
    unsafe fn value(&self) -> *const c_void {
        self as *const i32 as *const c_void
    }
}

impl ShaderV for [i32; 2] {
    const UNIFORM_TYPE: ShaderUniformDataType = ShaderUniformDataType::SHADER_UNIFORM_IVEC2;
    unsafe fn value(&self) -> *const c_void {
        self.as_ptr() as *const c_void
    }
}

impl ShaderV for [i32; 3] {
    const UNIFORM_TYPE: ShaderUniformDataType = ShaderUniformDataType::SHADER_UNIFORM_IVEC3;
    unsafe fn value(&self) -> *const c_void {
        self.as_ptr() as *const c_void
    }
}

impl ShaderV for [i32; 4] {
    const UNIFORM_TYPE: ShaderUniformDataType = ShaderUniformDataType::SHADER_UNIFORM_IVEC4;
    unsafe fn value(&self) -> *const c_void {
        self.as_ptr() as *const c_void
    }
}

impl ShaderV for [f32; 2] {
    const UNIFORM_TYPE: ShaderUniformDataType = ShaderUniformDataType::SHADER_UNIFORM_VEC2;
    unsafe fn value(&self) -> *const c_void {
        self.as_ptr() as *const c_void
    }
}

impl ShaderV for [f32; 3] {
    const UNIFORM_TYPE: ShaderUniformDataType = ShaderUniformDataType::SHADER_UNIFORM_VEC3;
    unsafe fn value(&self) -> *const c_void {
        self.as_ptr() as *const c_void
    }
}

impl ShaderV for [f32; 4] {
    const UNIFORM_TYPE: ShaderUniformDataType = ShaderUniformDataType::SHADER_UNIFORM_VEC4;
    unsafe fn value(&self) -> *const c_void {
        self.as_ptr() as *const c_void
    }
}

impl ShaderV for &[i32] {
    const UNIFORM_TYPE: ShaderUniformDataType = ShaderUniformDataType::SHADER_UNIFORM_SAMPLER2D;
    unsafe fn value(&self) -> *const c_void {
        self.as_ptr() as *const c_void
    }
}

impl Shader {
    pub unsafe fn make_weak(self) -> WeakShader {
        let m = WeakShader(self.0);
        std::mem::forget(self);
        m
    }

    /// Check if shader is valid
    #[inline]
    pub fn is_shader_valid(&self) -> bool {
        unsafe { ffi::IsShaderValid(self.0) }
    }

    /// Sets shader uniform value
    #[inline]
    pub fn set_shader_value<S: ShaderV>(&mut self, uniform_loc: i32, value: S) {
        unsafe {
            ffi::SetShaderValue(
                self.0,
                uniform_loc,
                value.value(),
                (S::UNIFORM_TYPE as u32) as i32,
            );
        }
    }

    /// Set shader uniform value vector
    #[inline]
    pub fn set_shader_value_v<S: ShaderV>(&mut self, uniform_loc: i32, value: &[S]) {
        unsafe {
            ffi::SetShaderValueV(
                self.0,
                uniform_loc,
                value.as_ptr() as *const ::std::os::raw::c_void,
                (S::UNIFORM_TYPE as u32) as i32,
                value.len() as i32,
            );
        }
    }

    /// Sets shader uniform value (matrix 4x4).
    #[inline]
    pub fn set_shader_value_matrix(&mut self, uniform_loc: i32, mat: Matrix) {
        unsafe {
            ffi::SetShaderValueMatrix(self.0, uniform_loc, mat.into());
        }
    }

    /// Sets shader uniform value (matrix 4x4).
    #[inline]
    pub fn set_shader_value_texture(
        &mut self,
        uniform_loc: i32,
        texture: impl AsRef<ffi::Texture2D>,
    ) {
        unsafe {
            ffi::SetShaderValueTexture(self.0, uniform_loc, *texture.as_ref());
        }
    }
}

impl RaylibShader for WeakShader {}
impl RaylibShader for Shader {}

pub trait RaylibShader: AsRef<ffi::Shader> + AsMut<ffi::Shader> {
    #[inline]
    fn locs(&self) -> &[i32] {
        unsafe { std::slice::from_raw_parts(self.as_ref().locs, 32) }
    }

    #[inline]
    fn locs_mut(&mut self) -> &mut [i32] {
        unsafe { std::slice::from_raw_parts_mut(self.as_mut().locs, 32) }
    }

    /// Gets shader uniform location by name.
    #[inline]
    fn get_shader_location(&self, uniform_name: &str) -> i32 {
        let c_uniform_name = CString::new(uniform_name).unwrap();
        unsafe { ffi::GetShaderLocation(*self.as_ref(), c_uniform_name.as_ptr()) }
    }

    /// Gets shader attribute location by name.
    #[inline]
    fn get_shader_location_attribute(&self, attribute_name: &str) -> i32 {
        let c_attribute_name = CString::new(attribute_name).unwrap();
        unsafe { ffi::GetShaderLocationAttrib(*self.as_ref(), c_attribute_name.as_ptr()) }
    }

    /// Sets shader uniform value
    #[inline]
    fn set_shader_value<S: ShaderV>(&mut self, uniform_loc: i32, value: S) {
        unsafe {
            ffi::SetShaderValue(
                *self.as_mut(),
                uniform_loc,
                value.value(),
                (S::UNIFORM_TYPE as u32) as i32,
            );
        }
    }

    /// et shader uniform value vector
    #[inline]
    fn set_shader_value_v<S: ShaderV>(&mut self, uniform_loc: i32, value: &[S]) {
        unsafe {
            ffi::SetShaderValueV(
                *self.as_mut(),
                uniform_loc,
                value.as_ptr() as *const ::std::os::raw::c_void,
                (S::UNIFORM_TYPE as u32) as i32,
                value.len() as i32,
            );
        }
    }

    /// Sets shader uniform value (matrix 4x4).
    #[inline]
    fn set_shader_value_matrix(&mut self, uniform_loc: i32, mat: Matrix) {
        unsafe {
            ffi::SetShaderValueMatrix(*self.as_mut(), uniform_loc, mat.into());
        }
    }

    /// Sets shader uniform value (matrix 4x4).
    #[inline]
    fn set_shader_value_texture(&mut self, uniform_loc: i32, texture: impl AsRef<ffi::Texture2D>) {
        unsafe {
            ffi::SetShaderValueTexture(*self.as_mut(), uniform_loc, *texture.as_ref());
        }
    }
}

impl RaylibHandle {
    /// Sets a custom projection matrix (replaces internal projection matrix).
    #[inline]
    #[cfg(target_os = "windows")]
    pub fn set_matrix_projection(&mut self, _: &RaylibThread, proj: Matrix) {
        unsafe {
            ffi::rlSetMatrixProjection(proj.into());
        }
    }

    /// Sets a custom modelview matrix (replaces internal modelview matrix).
    #[inline]
    #[cfg(target_os = "windows")]
    pub fn set_matrix_modelview(&mut self, _: &RaylibThread, view: Matrix) {
        unsafe {
            ffi::rlSetMatrixModelview(view.into());
        }
    }

    /// Gets internal modelview matrix.
    #[inline]
    #[cfg(target_os = "windows")]
    pub fn get_matrix_modelview(&self) -> Matrix {
        unsafe { ffi::rlGetMatrixModelview().into() }
    }

    /// Gets internal projection matrix.
    #[inline]
    #[cfg(target_os = "windows")]
    pub fn get_matrix_projection(&self) -> Matrix {
        unsafe { ffi::rlGetMatrixProjection().into() }
    }
}
