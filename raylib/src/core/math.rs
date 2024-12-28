/* raylib-rs
   raymath.rs - Structs and functions for game-related math and linear algebra

Copyright (c) 2018-2019 Paul Clement (@deltaphc)

This software is provided "as-is", without any express or implied warranty. In no event will the authors be held liable for any damages arising from the use of this software.

Permission is granted to anyone to use this software for any purpose, including commercial applications, and to alter it and redistribute it freely, subject to the following restrictions:

  1. The origin of this software must not be misrepresented; you must not claim that you wrote the original software. If you use this software in a product, an acknowledgment in the product documentation would be appreciated but is not required.

  2. Altered source versions must be plainly marked as such, and must not be misrepresented as being the original software.

  3. This notice may not be removed or altered from any source distribution.
*/

use crate::ffi;
use crate::misc::AsF32;
use std::f32::consts::PI;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Range, Sub, SubAssign};

#[cfg(feature = "with_serde")]
use serde::{Deserialize, Serialize};

make_rslice!(RSliceVec4, Vector4, ffi::MemFree);

macro_rules! optional_serde_struct {
    ($def:item) => {
        cfg_if::cfg_if! {
            if #[cfg(feature = "with_serde")] {
                #[repr(C)]
                #[derive(Default, Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
                $def
            } else {
                #[repr(C)]
                #[derive(Default, Debug, Copy, Clone, PartialEq)]
                $def
            }
        }
    };
}

optional_serde_struct! {
    pub struct Vector2 {
        pub x: f32,
        pub y: f32,
    }
}

#[cfg(feature = "convert_mint")]
impl From<mint::Vector2<f32>> for Vector2 {
    fn from(v: mint::Vector2<f32>) -> Vector2 {
        Vector2 { x: v.x, y: v.y }
    }
}

#[cfg(feature = "convert_mint")]
impl From<mint::Point2<f32>> for Vector2 {
    fn from(v: mint::Point2<f32>) -> Vector2 {
        Vector2 { x: v.x, y: v.y }
    }
}

#[cfg(feature = "convert_mint")]
impl From<Vector2> for mint::Vector2<f32> {
    fn from(v: Vector2) -> Self {
        Self { x: v.x, y: v.y }
    }
}

impl From<ffi::Vector2> for Vector2 {
    fn from(v: ffi::Vector2) -> Vector2 {
        unsafe { std::mem::transmute(v) }
    }
}

impl From<Vector2> for ffi::Vector2 {
    fn from(v: Vector2) -> Self {
        unsafe { std::mem::transmute(v) }
    }
}

impl From<&Vector2> for ffi::Vector2 {
    fn from(v: &Vector2) -> ffi::Vector2 {
        ffi::Vector2 { x: v.x, y: v.y }
    }
}

/// A convenience function for linearly interpolating an `f32`.
#[inline]
pub fn lerp(v0: f32, v1: f32, amount: f32) -> f32 {
    v0 + amount * (v1 - v0)
}

/// A convenience function for making a new `Vector2`.
#[inline]
pub fn rvec2<T1: AsF32, T2: AsF32>(x: T1, y: T2) -> Vector2 {
    Vector2::new(x.as_f32(), y.as_f32())
}

/// A convenience function for making a new `Vector3`.
#[inline]
pub fn rvec3<T1: AsF32, T2: AsF32, T3: AsF32>(x: T1, y: T2, z: T3) -> Vector3 {
    Vector3::new(x.as_f32(), y.as_f32(), z.as_f32())
}

/// A convenience function for making a new `Quaternion`.
#[inline]
pub fn rquat<T1: AsF32, T2: AsF32, T3: AsF32, T4: AsF32>(x: T1, y: T2, z: T3, w: T4) -> Quaternion {
    Quaternion::new(x.as_f32(), y.as_f32(), z.as_f32(), w.as_f32())
}

/// A convenience function for making a new `Rectangle`.
#[inline]
pub fn rrect<T1: AsF32, T2: AsF32, T3: AsF32, T4: AsF32>(
    x: T1,
    y: T2,
    width: T3,
    height: T4,
) -> Rectangle {
    Rectangle::new(x.as_f32(), y.as_f32(), width.as_f32(), height.as_f32())
}

impl Vector2 {
    /// Constant `Vector2` with both components set to zero.
    const ZERO: Vector2 = Vector2 { x: 0.0, y: 0.0 };

    /// Constant `Vector2` with both components set to one.
    const ONE: Vector2 = Vector2 { x: 1.0, y: 1.0 };

    /// Returns a new `Vector2` with specified components.
    pub const fn new(x: f32, y: f32) -> Vector2 {
        Vector2 { x, y }
    }

    /// Returns a new `Vector2` with both components set to zero.
    #[inline]
    pub const fn zero() -> Vector2 {
        Vector2 { x: 0.0, y: 0.0 }
    }

    /// Returns a new `Vector2` with both components set to one.
    #[inline]
    pub const fn one() -> Vector2 {
        Vector2 { x: 1.0, y: 1.0 }
    }

    /// Calculates the vector length.
    pub fn length(&self) -> f32 {
        ((self.x * self.x) + (self.y * self.y)).sqrt()
    }

    /// Calculates the vector length square (**2);
    pub fn length_sqr(&self) -> f32 {
        (self.x * self.x) + (self.y * self.y)
    }

    /// Calculates the dot product with vector `v`.
    pub fn dot(&self, v: Vector2) -> f32 {
        self.x * v.x + self.y * v.y
    }

    /// Calculates the distance towards vector `v`.
    pub fn distance_to(&self, v: Vector2) -> f32 {
        ((self.x - v.x) * (self.x - v.x) + (self.y - v.y) * (self.y - v.y)).sqrt()
    }

    /// Calculates the angle towards vector `v` in radians.
    pub fn angle_to(&self, v: Vector2) -> f32 {
        let mut result = (v.y - self.y).atan2(v.x - self.x);
        if result < 0.0 {
            result += 2.0 * PI;
        }
        result
    }

    /// Scales the vector by multiplying both components by `scale`.
    pub fn scale(&mut self, scale: f32) {
        *self *= scale;
    }

    /// Returns a new `Vector2` with components scaled by `scale`.
    pub fn scale_by(&self, scale: f32) -> Vector2 {
        *self * scale
    }

    /// Normalizes the vector.
    pub fn normalize(&mut self) {
        *self = self.normalized();
    }

    /// Returns a new `Vector2` with normalized components from the current vector.
    pub fn normalized(&self) -> Vector2 {
        let length_sqr = self.length_sqr();
        if length_sqr == 0.0 {
            return *self;
        }
        *self / length_sqr.sqrt()
    }

    /// Rotates the vector by `angle` radians.
    pub fn rotate(&mut self, angle: f32) {
        let cos_res = angle.cos();
        let sin_res = angle.sin();

        let result = Vector2::new(
            self.x * cos_res - self.y * sin_res,
            self.x * sin_res + self.y * cos_res,
        );

        self.x = result.x;
        self.y = result.y;
    }

    /// Returns a new `Vector2` rotated by `angle` radians.
    pub fn rotated(&self, angle: f32) -> Vector2 {
        let cos_res = angle.cos();
        let sin_res = angle.sin();

        Vector2::new(
            self.x * cos_res - self.y * sin_res,
            self.x * sin_res + self.y * cos_res,
        )
    }

    /// Returns a new `Vector2` with componenets linearly interpolated by `amount` towards vector `v`.
    pub fn lerp(&self, v: Vector2, amount: f32) -> Vector2 {
        Vector2 {
            x: self.x + amount * (v.x - self.x),
            y: self.y + amount * (v.y - self.y),
        }
    }

    /// Returns a new `Vector2` with componenets clamp to a certain interval.
    pub fn clamp(&self, num: Range<f32>) -> Vector2 {
        Vector2 {
            x: self.x.clamp(num.start, num.end),
            y: self.y.clamp(num.start, num.end),
        }
    }
}

impl From<(f32, f32)> for Vector2 {
    #[inline]
    fn from((x, y): (f32, f32)) -> Vector2 {
        Vector2 { x, y }
    }
}

impl Add for Vector2 {
    type Output = Vector2;
    fn add(self, v: Vector2) -> Self {
        Vector2 {
            x: self.x + v.x,
            y: self.y + v.y,
        }
    }
}

impl Add<f32> for Vector2 {
    type Output = Vector2;
    fn add(self, value: f32) -> Self {
        Vector2 {
            x: self.x + value,
            y: self.y + value,
        }
    }
}

impl AddAssign for Vector2 {
    fn add_assign(&mut self, v: Vector2) {
        *self = *self + v;
    }
}

impl AddAssign<f32> for Vector2 {
    fn add_assign(&mut self, value: f32) {
        *self = *self + value;
    }
}

impl Sub for Vector2 {
    type Output = Vector2;
    fn sub(self, v: Vector2) -> Self {
        Vector2 {
            x: self.x - v.x,
            y: self.y - v.y,
        }
    }
}

impl Sub<f32> for Vector2 {
    type Output = Vector2;
    fn sub(self, value: f32) -> Self {
        Vector2 {
            x: self.x - value,
            y: self.y - value,
        }
    }
}

impl SubAssign for Vector2 {
    fn sub_assign(&mut self, v: Vector2) {
        *self = *self - v;
    }
}

impl SubAssign<f32> for Vector2 {
    fn sub_assign(&mut self, value: f32) {
        *self = *self - value;
    }
}

impl Mul for Vector2 {
    type Output = Vector2;
    fn mul(self, v: Vector2) -> Self {
        Vector2 {
            x: self.x * v.x,
            y: self.y * v.y,
        }
    }
}

impl Mul<f32> for Vector2 {
    type Output = Vector2;
    fn mul(self, value: f32) -> Self {
        Vector2 {
            x: self.x * value,
            y: self.y * value,
        }
    }
}

impl MulAssign for Vector2 {
    fn mul_assign(&mut self, v: Vector2) {
        *self = *self * v;
    }
}

impl MulAssign<f32> for Vector2 {
    fn mul_assign(&mut self, value: f32) {
        *self = *self * value;
    }
}

impl Div for Vector2 {
    type Output = Vector2;
    fn div(self, v: Vector2) -> Self {
        Vector2 {
            x: self.x / v.x,
            y: self.y / v.y,
        }
    }
}

impl Div<f32> for Vector2 {
    type Output = Vector2;
    fn div(self, value: f32) -> Self {
        Vector2 {
            x: self.x / value,
            y: self.y / value,
        }
    }
}

impl DivAssign for Vector2 {
    fn div_assign(&mut self, v: Vector2) {
        *self = *self / v;
    }
}

impl DivAssign<f32> for Vector2 {
    fn div_assign(&mut self, value: f32) {
        *self = *self / value;
    }
}

impl Neg for Vector2 {
    type Output = Vector2;
    fn neg(self) -> Self {
        Vector2 {
            x: -self.x,
            y: -self.y,
        }
    }
}

optional_serde_struct! {
    pub struct Vector3 {
        pub x: f32,
        pub y: f32,
        pub z: f32,
    }
}

#[cfg(feature = "convert_mint")]
impl From<mint::Vector3<f32>> for Vector3 {
    fn from(v: mint::Vector3<f32>) -> Vector3 {
        Vector3 {
            x: v.x,
            y: v.y,
            z: v.z,
        }
    }
}

#[cfg(feature = "convert_mint")]
impl From<mint::Point3<f32>> for Vector3 {
    fn from(v: mint::Point3<f32>) -> Vector3 {
        Vector3 {
            x: v.x,
            y: v.y,
            z: v.z,
        }
    }
}

#[cfg(feature = "convert_mint")]
impl From<Vector3> for mint::Vector3<f32> {
    fn from(v: Vector3) -> Self {
        Self {
            x: v.x,
            y: v.y,
            z: v.z,
        }
    }
}

impl From<ffi::Vector3> for Vector3 {
    fn from(v: ffi::Vector3) -> Vector3 {
        unsafe { std::mem::transmute(v) }
    }
}

impl From<Vector3> for ffi::Vector3 {
    fn from(v: Vector3) -> Self {
        unsafe { std::mem::transmute(v) }
    }
}

impl From<&Vector3> for ffi::Vector3 {
    fn from(v: &Vector3) -> ffi::Vector3 {
        ffi::Vector3 {
            x: v.x,
            y: v.y,
            z: v.z,
        }
    }
}

impl Vector3 {
    /// Returns a new `Vector3` with specified components.
    pub const fn new(x: f32, y: f32, z: f32) -> Vector3 {
        Vector3 { x, y, z }
    }

    pub fn up() -> Vector3 {
        Vector3::new(0.0, 1.0, 0.0)
    }

    pub fn forward() -> Vector3 {
        Vector3::new(0.0, 0.0, 1.0)
    }

    pub fn right() -> Vector3 {
        Vector3::new(1.0, 0.0, 0.0)
    }

    pub fn left() -> Vector3 {
        Vector3::new(-1.0, 0.0, 0.0)
    }

    /// Returns a new `Vector3` with all components set to zero.
    pub fn zero() -> Vector3 {
        Vector3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    /// Returns a new `Vector3` with all components set to one.
    pub fn one() -> Vector3 {
        Vector3 {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        }
    }

    /// Returns a new `Vector3` containing the cross product between `self` and vector `v`.
    pub fn cross(&self, v: Vector3) -> Vector3 {
        Vector3 {
            x: self.y * v.z - self.z * v.y,
            y: self.z * v.x - self.x * v.z,
            z: self.x * v.y - self.y * v.x,
        }
    }

    /// Returns a new `Vector3` perpendicular to `self`.
    pub fn perpendicular(&self) -> Vector3 {
        let mut min = self.x.abs();
        let mut cardinal_axis = Vector3 {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        };

        if self.y.abs() < min {
            min = self.y.abs();
            cardinal_axis = Vector3 {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            };
        }

        if self.z.abs() < min {
            cardinal_axis = Vector3 {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            };
        }

        self.cross(cardinal_axis)
    }

    /// Calculates the vector length.
    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    /// Calculates the dot product with vector `v`.
    pub fn dot(&self, v: Vector3) -> f32 {
        self.x * v.x + self.y * v.y + self.z * v.z
    }

    /// Calculates the distance towards vector `v`.
    pub fn distance_to(&self, v: Vector3) -> f32 {
        let dx = v.x - self.x;
        let dy = v.y - self.y;
        let dz = v.z - self.z;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }

    /// Scales the vector by multiplying both components by `scale`.
    pub fn scale(&mut self, scale: f32) {
        *self *= scale;
    }

    /// Returns a new `Vector3` with components scaled by `scale`.
    pub fn scale_by(&self, scale: f32) -> Vector3 {
        *self * scale
    }

    /// Normalizes the current vector.
    pub fn normalize(&mut self) {
        *self = self.normalized();
    }

    /// Returns a new `Vector3` with normalized components from the current vector.
    pub fn normalized(&self) -> Vector3 {
        let mut length = self.length();
        if length == 0.0 {
            length = 1.0;
        }
        let ilength = 1.0 / length;

        Vector3 {
            x: self.x * ilength,
            y: self.y * ilength,
            z: self.z * ilength,
        }
    }

    /// Normalizes and changes both `self` and `v` to be orthogonal to eachother.
    pub fn ortho_normalize(&mut self, v: &mut Vector3) {
        *self = self.normalized();
        let vn = self.cross(*v).normalized();
        *v = vn.cross(*self);
    }

    /// Transforms the current vector using Matrix `mat`.
    pub fn transform(&mut self, mat: Matrix) {
        *self = self.transform_with(mat);
    }

    /// Returns a new `Vector3` containing components transformed by Matrix `mat`.
    pub fn transform_with(&self, mat: Matrix) -> Vector3 {
        Vector3 {
            x: mat.m0 * self.x + mat.m4 * self.y + mat.m8 * self.z + mat.m12,
            y: mat.m1 * self.x + mat.m5 * self.y + mat.m9 * self.z + mat.m13,
            z: mat.m2 * self.x + mat.m6 * self.y + mat.m10 * self.z + mat.m14,
        }
    }

    /// Rotates the current vector using Quaternion `q`.
    pub fn rotate(&mut self, q: Quaternion) {
        *self = self.rotate_by(q);
    }

    /// Returns a new `Vector3` with components rotated by Quaternion `q`.
    pub fn rotate_by(&self, q: Quaternion) -> Vector3 {
        Vector3 {
            x: self.x * (q.x * q.x + q.w * q.w - q.y * q.y - q.z * q.z)
                + self.y * (2.0 * q.x * q.y - 2.0 * q.w * q.z)
                + self.z * (2.0 * q.x * q.z + 2.0 * q.w * q.y),
            y: self.x * (2.0 * q.w * q.z + 2.0 * q.x * q.y)
                + self.y * (q.w * q.w - q.x * q.x + q.y * q.y - q.z * q.z)
                + self.z * (-2.0 * q.w * q.x + 2.0 * q.y * q.z),
            z: self.x * (-2.0 * q.w * q.y + 2.0 * q.x * q.z)
                + self.y * (2.0 * q.w * q.x + 2.0 * q.y * q.z)
                + self.z * (q.w * q.w - q.x * q.x - q.y * q.y + q.z * q.z),
        }
    }

    /// Returns a new `Vector3` with componenets linearly interpolated by `amount` towards vector `v`.
    pub fn lerp(&self, v: Vector3, amount: f32) -> Vector3 {
        Vector3 {
            x: self.x + amount * (v.x - self.x),
            y: self.y + amount * (v.y - self.y),
            z: self.z + amount * (v.z - self.z),
        }
    }

    /// Reflects the current vector from `normal`.
    pub fn reflect(&mut self, normal: Vector3) {
        *self = self.reflect_from(normal);
    }

    /// Returns a new `Vector3` reflected from the current vector using `normal`.
    pub fn reflect_from(&self, normal: Vector3) -> Vector3 {
        let dot_product = self.dot(normal);
        Vector3 {
            x: self.x - (2.0 * normal.x) * dot_product,
            y: self.y - (2.0 * normal.y) * dot_product,
            z: self.z - (2.0 * normal.z) * dot_product,
        }
    }

    /// Returns a new `Vector3` containing the minimum of each corresponding component.
    pub fn min(&self, v: Vector3) -> Vector3 {
        Vector3 {
            x: self.x.min(v.x),
            y: self.y.min(v.y),
            z: self.z.min(v.z),
        }
    }

    /// Returns a new `Vector3` containing the maximum of each corresponding component.
    pub fn max(&self, v: Vector3) -> Vector3 {
        Vector3 {
            x: self.x.max(v.x),
            y: self.y.max(v.y),
            z: self.z.max(v.z),
        }
    }

    /// Returns barycenter coordinates (u, v, w) from point p (current vector) with respect to triangle (`a`, `b`, `c`).
    pub fn barycenter(&self, a: Vector3, b: Vector3, c: Vector3) -> Vector3 {
        let v0 = b - a;
        let v1 = c - a;
        let v2 = *self - a;
        let d00 = v0.dot(v0);
        let d01 = v0.dot(v1);
        let d11 = v1.dot(v1);
        let d20 = v2.dot(v0);
        let d21 = v2.dot(v1);
        let denom = d00 * d11 - d01 * d01;

        let y = (d11 * d20 - d01 * d21) / denom;
        let z = (d00 * d21 - d01 * d20) / denom;
        Vector3 {
            x: 1.0 - (z + y),
            y,
            z,
        }
    }

    /// Returns a 3-length `f32` array containing components `[x, y, z]` of the current vector.
    pub fn to_array(&self) -> [f32; 3] {
        [self.x, self.y, self.z]
    }

    /// Returns a new `Vector3` with componenets clamp to a certain interval.
    pub fn clamp(&self, num: Range<f32>) -> Vector3 {
        Vector3 {
            x: self.x.clamp(num.start, num.end),
            y: self.y.clamp(num.start, num.end),
            z: self.z.clamp(num.start, num.end),
        }
    }
}

impl From<(f32, f32, f32)> for Vector3 {
    #[inline]
    fn from((x, y, z): (f32, f32, f32)) -> Vector3 {
        Vector3 { x, y, z }
    }
}

impl Add for Vector3 {
    type Output = Vector3;
    fn add(self, v: Vector3) -> Self {
        Vector3 {
            x: self.x + v.x,
            y: self.y + v.y,
            z: self.z + v.z,
        }
    }
}

impl Add<f32> for Vector3 {
    type Output = Vector3;
    fn add(self, value: f32) -> Self {
        Vector3 {
            x: self.x + value,
            y: self.y + value,
            z: self.z + value,
        }
    }
}

impl AddAssign for Vector3 {
    fn add_assign(&mut self, v: Vector3) {
        *self = *self + v;
    }
}

impl AddAssign<f32> for Vector3 {
    fn add_assign(&mut self, value: f32) {
        *self = *self + value;
    }
}

impl Sub for Vector3 {
    type Output = Vector3;
    fn sub(self, v: Vector3) -> Self {
        Vector3 {
            x: self.x - v.x,
            y: self.y - v.y,
            z: self.z - v.z,
        }
    }
}

impl Sub<f32> for Vector3 {
    type Output = Vector3;
    fn sub(self, value: f32) -> Self {
        Vector3 {
            x: self.x - value,
            y: self.y - value,
            z: self.z - value,
        }
    }
}

impl SubAssign for Vector3 {
    fn sub_assign(&mut self, v: Vector3) {
        *self = *self - v;
    }
}

impl SubAssign<f32> for Vector3 {
    fn sub_assign(&mut self, value: f32) {
        *self = *self - value;
    }
}

impl Mul for Vector3 {
    type Output = Vector3;
    fn mul(self, v: Vector3) -> Self {
        Vector3 {
            x: self.x * v.x,
            y: self.y * v.y,
            z: self.z * v.z,
        }
    }
}

impl Mul<f32> for Vector3 {
    type Output = Vector3;
    fn mul(self, value: f32) -> Self {
        Vector3 {
            x: self.x * value,
            y: self.y * value,
            z: self.z * value,
        }
    }
}

impl MulAssign for Vector3 {
    fn mul_assign(&mut self, v: Vector3) {
        *self = *self * v;
    }
}

impl MulAssign<f32> for Vector3 {
    fn mul_assign(&mut self, value: f32) {
        *self = *self * value;
    }
}

impl Div for Vector3 {
    type Output = Vector3;
    fn div(self, v: Vector3) -> Self {
        Vector3 {
            x: self.x / v.x,
            y: self.y / v.y,
            z: self.z / v.z,
        }
    }
}

impl Div<f32> for Vector3 {
    type Output = Vector3;
    fn div(self, value: f32) -> Self {
        Vector3 {
            x: self.x / value,
            y: self.y / value,
            z: self.z / value,
        }
    }
}

impl DivAssign for Vector3 {
    fn div_assign(&mut self, v: Vector3) {
        *self = *self / v;
    }
}

impl DivAssign<f32> for Vector3 {
    fn div_assign(&mut self, value: f32) {
        *self = *self / value;
    }
}

impl Neg for Vector3 {
    type Output = Vector3;
    fn neg(self) -> Self {
        Vector3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

optional_serde_struct! {
    pub struct Vector4 {
        pub x: f32,
        pub y: f32,
        pub z: f32,
        pub w: f32,
    }
}

pub type Quaternion = Vector4;

#[cfg(feature = "convert_mint")]
impl From<mint::Vector4<f32>> for Vector4 {
    fn from(v: mint::Vector4<f32>) -> Vector4 {
        Vector4 {
            x: v.x,
            y: v.y,
            z: v.z,
            w: v.w,
        }
    }
}

#[cfg(feature = "convert_mint")]
impl From<Vector4> for mint::Vector4<f32> {
    fn from(v: Vector4) -> Self {
        mint::Vector4 {
            x: v.x,
            y: v.y,
            z: v.z,
            w: v.w,
        }
    }
}

impl From<ffi::Vector4> for Vector4 {
    fn from(v: ffi::Vector4) -> Vector4 {
        unsafe { std::mem::transmute(v) }
    }
}

impl From<Vector4> for ffi::Vector4 {
    fn from(v: Vector4) -> ffi::Vector4 {
        unsafe { std::mem::transmute(v) }
    }
}

impl From<&Vector4> for ffi::Vector4 {
    fn from(v: &Vector4) -> ffi::Vector4 {
        ffi::Vector4 {
            x: v.x,
            y: v.y,
            z: v.z,
            w: v.w,
        }
    }
}

impl Quaternion {
    /// Returns a new `Quaternion` with specified components.
    pub const fn new(x: f32, y: f32, z: f32, w: f32) -> Quaternion {
        Quaternion { x, y, z, w }
    }

    /// Returns the identity quaternion.
    pub fn identity() -> Quaternion {
        Quaternion {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 1.0,
        }
    }

    /// Returns quaternion based on the rotation from one vector to another.
    pub fn from_vec3_pair(from: Vector3, to: Vector3) -> Quaternion {
        let cross = from.cross(to);
        Quaternion {
            x: cross.x,
            y: cross.y,
            z: cross.z,
            w: 1.0 + from.dot(to),
        }
        .normalized()
    }

    /// Returns a quaternion for a given rotation matrix.
    pub fn from_matrix(mat: Matrix) -> Quaternion {
        let trace = mat.trace();

        if trace > 0.0 {
            let s = (trace + 1.0).sqrt() * 2.0;
            let inv_s = 1.0 / s;

            Quaternion {
                w: s * 0.25,
                x: (mat.m6 - mat.m9) * inv_s,
                y: (mat.m8 - mat.m2) * inv_s,
                z: (mat.m1 - mat.m4) * inv_s,
            }
        } else {
            let m00 = mat.m0;
            let m11 = mat.m5;
            let m22 = mat.m10;

            if m00 > m11 && m00 > m22 {
                let s = (1.0 + m00 - m11 - m22).sqrt() * 2.0;
                let inv_s = 1.0 / s;

                Quaternion {
                    w: (mat.m6 - mat.m9) * inv_s,
                    x: s * 0.25,
                    y: (mat.m4 + mat.m1) * inv_s,
                    z: (mat.m8 + mat.m2) * inv_s,
                }
            } else if m11 > m22 {
                let s = (1.0 + m11 - m00 - m22).sqrt() * 2.0;
                let inv_s = 1.0 / s;

                Quaternion {
                    w: (mat.m8 - mat.m2) * inv_s,
                    x: (mat.m4 + mat.m1) * inv_s,
                    y: s * 0.25,
                    z: (mat.m9 + mat.m6) * inv_s,
                }
            } else {
                let s = (1.0 + m22 - m00 - m11).sqrt() * 2.0;
                let inv_s = 1.0 / s;

                Quaternion {
                    w: (mat.m1 - mat.m4) * inv_s,
                    x: (mat.m8 + mat.m2) * inv_s,
                    y: (mat.m9 + mat.m6) * inv_s,
                    z: s * 0.25,
                }
            }
        }
    }

    /// Returns a rotation matrix for the current quaternion.
    pub fn to_matrix(&self) -> Matrix {
        let x = self.x;
        let y = self.y;
        let z = self.z;
        let w = self.w;

        let x2 = x + x;
        let y2 = y + y;
        let z2 = z + z;

        let length = self.length();
        let length_squared = length * length;

        let xx = x * x2 / length_squared;
        let xy = x * y2 / length_squared;
        let xz = x * z2 / length_squared;

        let yy = y * y2 / length_squared;
        let yz = y * z2 / length_squared;
        let zz = z * z2 / length_squared;

        let wx = w * x2 / length_squared;
        let wy = w * y2 / length_squared;
        let wz = w * z2 / length_squared;

        Matrix {
            m0: 1.0 - (yy + zz),
            m1: xy - wz,
            m2: xz + wy,
            m3: 0.0,
            m4: xy + wz,
            m5: 1.0 - (xx + zz),
            m6: yz - wx,
            m7: 0.0,
            m8: xz - wy,
            m9: yz + wx,
            m10: 1.0 - (xx + yy),
            m11: 0.0,
            m12: 0.0,
            m13: 0.0,
            m14: 0.0,
            m15: 1.0,
        }
    }

    /// Returns a quaternion equivalent to Euler angles.
    pub fn from_euler(pitch: f32, yaw: f32, roll: f32) -> Quaternion {
        let x0 = (pitch * 0.5).cos();
        let x1 = (pitch * 0.5).sin();
        let y0 = (yaw * 0.5).cos();
        let y1 = (yaw * 0.5).sin();
        let z0 = (roll * 0.5).cos();
        let z1 = (roll * 0.5).sin();

        Quaternion {
            x: (x1 * y0 * z0) - (x0 * y1 * z1),
            y: (x0 * y1 * z0) + (x1 * y0 * z1),
            z: (x0 * y0 * z1) - (x1 * y1 * z0),
            w: (x0 * y0 * z0) + (x1 * y1 * z1),
        }
    }

    /// Returns a vector containing Euler angles in radians (roll, pitch, yaw), based on the current quaternion.
    pub fn to_euler(&self) -> Vector3 {
        // roll (x-axis rotation)
        let x0 = 2.0 * (self.w * self.x + self.y * self.z);
        let x1 = 1.0 - 2.0 * (self.x * self.x + self.y * self.y);

        // pitch (y-axis rotation)
        let mut y0 = 2.0 * (self.w * self.y - self.z * self.x);
        y0 = if y0 > 1.0 { 1.0 } else { y0 };
        y0 = if y0 < -1.0 { -1.0 } else { y0 };

        // yaw (z-axis rotation)
        let z0 = 2.0 * (self.w * self.z + self.x * self.y);
        let z1 = 1.0 - 2.0 * (self.y * self.y + self.z * self.z);

        Vector3 {
            x: x0.atan2(x1),
            y: y0.asin(),
            z: z0.atan2(z1),
        }
    }

    /// Returns rotation quaternion for an `axis` and `angle` (in radians).
    pub fn from_axis_angle(axis: Vector3, angle: f32) -> Quaternion {
        let mut result = Quaternion::identity();
        let mut axis = axis;
        let mut angle = angle;

        if axis.length() != 0.0 {
            angle *= 0.5;
        }

        axis.normalize();

        let sinres = angle.sin();
        let cosres = angle.cos();

        result.x = axis.x * sinres;
        result.y = axis.y * sinres;
        result.z = axis.z * sinres;
        result.w = cosres;
        result.normalized()
    }

    /// Returns a 2-tuple containing the axis (`Vector3`) and angle (`f32` in radians) for the current quaternion.
    pub fn to_axis_angle(&self) -> (Vector3, f32) {
        let mut q = *self;
        if q.w.abs() > 1.0 {
            q = q.normalized();
        }

        let mut res_axis = Vector3::zero();
        let res_angle = 2.0 * q.w.acos();
        let den = (1.0 - q.w * q.w).sqrt();

        if den > 0.0001 {
            res_axis.x = q.x / den;
            res_axis.y = q.y / den;
            res_axis.z = q.z / den;
        } else {
            // This occurs when the angle is zero.
            // Not a problem: just set an arbitrary normalized axis.
            res_axis.x = 1.0;
        }

        (res_axis, res_angle)
    }

    /// Computes the length of the current quaternion.
    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w).sqrt()
    }

    /// Returns a normalized version of the current quaternion.
    pub fn normalized(&self) -> Quaternion {
        let mut length = self.length();
        if length == 0.0 {
            length = 1.0;
        }
        let ilength = 1.0 / length;

        Quaternion {
            x: self.x * ilength,
            y: self.y * ilength,
            z: self.z * ilength,
            w: self.w * ilength,
        }
    }

    /// Returns an inverted version of the current quaternion.
    pub fn inverted(&self) -> Quaternion {
        let mut result = *self;
        let length = self.length();
        let length_sq = length * length;

        if length_sq != 0.0 {
            let i = 1.0 / length_sq;
            result.x *= -i;
            result.y *= -i;
            result.z *= -i;
            result.w *= i;
        }
        result
    }

    /// Calculates linear interpolation between current and `q` quaternions.
    pub fn lerp(&self, q: Quaternion, amount: f32) -> Quaternion {
        Quaternion {
            x: self.x + amount * (q.x - self.x),
            y: self.y + amount * (q.y - self.y),
            z: self.z + amount * (q.z - self.z),
            w: self.w + amount * (q.w - self.w),
        }
    }

    /// Calculates slerp-optimized interpolation between current and `q` quaternions.
    pub fn nlerp(&self, q: Quaternion, amount: f32) -> Quaternion {
        self.lerp(q, amount).normalized()
    }

    /// Calculates spherical linear interpolation between current and `q` quaternions.
    pub fn slerp(&self, q: Quaternion, amount: f32) -> Quaternion {
        let cos_half_theta = self.x * q.x + self.y * q.y + self.z * q.z + self.w * q.w;

        if cos_half_theta.abs() >= 1.0 {
            *self
        } else if cos_half_theta > 0.95 {
            self.nlerp(q, amount)
        } else {
            let half_theta = cos_half_theta.acos();
            let sin_half_theta = (1.0 - cos_half_theta * cos_half_theta).sqrt();

            if sin_half_theta.abs() < 0.001 {
                Quaternion {
                    x: (self.x * 0.5 + q.x * 0.5),
                    y: (self.y * 0.5 + q.y * 0.5),
                    z: (self.z * 0.5 + q.z * 0.5),
                    w: (self.w * 0.5 + q.w * 0.5),
                }
            } else {
                let ratio_a = ((1.0 - amount) * half_theta).sin() / sin_half_theta;
                let ratio_b = (amount * half_theta).sin() / sin_half_theta;

                Quaternion {
                    x: (self.x * ratio_a + q.x * ratio_b),
                    y: (self.y * ratio_a + q.y * ratio_b),
                    z: (self.z * ratio_a + q.z * ratio_b),
                    w: (self.w * ratio_a + q.w * ratio_b),
                }
            }
        }
    }

    /// Returns a transformed version of the current quaternion given a transformation matrix.
    pub fn transform(&self, mat: Matrix) -> Quaternion {
        Quaternion {
            x: mat.m0 * self.x + mat.m4 * self.y + mat.m8 * self.z + mat.m12 * self.w,
            y: mat.m1 * self.x + mat.m5 * self.y + mat.m9 * self.z + mat.m13 * self.w,
            z: mat.m2 * self.x + mat.m6 * self.y + mat.m10 * self.z + mat.m14 * self.w,
            w: mat.m3 * self.x + mat.m7 * self.y + mat.m11 * self.z + mat.m15 * self.w,
        }
    }

    /// Returns a new `Quaternion` with componenets clamp to a certain interval.
    pub fn clamp(&self, num: Range<f32>) -> Quaternion {
        Quaternion {
            x: self.x.clamp(num.start, num.end),
            y: self.y.clamp(num.start, num.end),
            z: self.z.clamp(num.start, num.end),
            w: self.w.clamp(num.start, num.end),
        }
    }
}

#[cfg(feature = "convert_mint")]
impl From<mint::Quaternion<f32>> for Quaternion {
    fn from(q: mint::Quaternion<f32>) -> Quaternion {
        Quaternion {
            x: q.v.x,
            y: q.v.y,
            z: q.v.z,
            w: q.s,
        }
    }
}

#[cfg(feature = "convert_mint")]
impl From<Quaternion> for mint::Quaternion<f32> {
    fn from(q: Quaternion) -> Self {
        Self {
            v: mint::Vector3 {
                x: q.x,
                y: q.y,
                z: q.z,
            },
            s: q.w,
        }
    }
}

impl From<(f32, f32, f32, f32)> for Quaternion {
    #[inline]
    fn from((x, y, z, w): (f32, f32, f32, f32)) -> Quaternion {
        Quaternion { x, y, z, w }
    }
}

impl Mul for Quaternion {
    type Output = Quaternion;
    fn mul(self, q: Quaternion) -> Quaternion {
        let qax = self.x;
        let qay = self.y;
        let qaz = self.z;
        let qaw = self.w;
        let qbx = q.x;
        let qby = q.y;
        let qbz = q.z;
        let qbw = q.w;

        Quaternion {
            x: (qax * qbw) + (qaw * qbx) + (qay * qbz) - (qaz * qby),
            y: (qay * qbw) + (qaw * qby) + (qaz * qbx) - (qax * qbz),
            z: (qaz * qbw) + (qaw * qbz) + (qax * qby) - (qay * qbx),
            w: (qaw * qbw) - (qax * qbx) - (qay * qby) - (qaz * qbz),
        }
    }
}

impl MulAssign for Quaternion {
    fn mul_assign(&mut self, q: Quaternion) {
        *self = *self * q;
    }
}

optional_serde_struct! {
    pub struct Matrix {
        pub m0: f32,
        pub m4: f32,
        pub m8: f32,
        pub m12: f32,
        pub m1: f32,
        pub m5: f32,
        pub m9: f32,
        pub m13: f32,
        pub m2: f32,
        pub m6: f32,
        pub m10: f32,
        pub m14: f32,
        pub m3: f32,
        pub m7: f32,
        pub m11: f32,
        pub m15: f32,
    }
}

impl From<ffi::Matrix> for Matrix {
    fn from(r: ffi::Matrix) -> Matrix {
        unsafe { std::mem::transmute(r) }
    }
}

impl From<Matrix> for ffi::Matrix {
    fn from(v: Matrix) -> Self {
        unsafe { std::mem::transmute(v) }
    }
}

impl From<&Matrix> for ffi::Matrix {
    fn from(v: &Matrix) -> Self {
        ffi::Matrix {
            m0: v.m0,
            m4: v.m4,
            m8: v.m8,
            m12: v.m12,
            m1: v.m1,
            m5: v.m5,
            m9: v.m9,
            m13: v.m13,
            m2: v.m2,
            m6: v.m6,
            m10: v.m10,
            m14: v.m14,
            m3: v.m3,
            m7: v.m7,
            m11: v.m11,
            m15: v.m15,
        }
    }
}

impl Matrix {
    /// Returns the identity matrix.
    pub fn identity() -> Matrix {
        Matrix {
            m0: 1.0,
            m4: 0.0,
            m8: 0.0,
            m12: 0.0,
            m1: 0.0,
            m5: 1.0,
            m9: 0.0,
            m13: 0.0,
            m2: 0.0,
            m6: 0.0,
            m10: 1.0,
            m14: 0.0,
            m3: 0.0,
            m7: 0.0,
            m11: 0.0,
            m15: 1.0,
        }
    }

    /// Returns the zero matriz.
    pub fn zero() -> Matrix {
        Matrix {
            m0: 0.0,
            m4: 0.0,
            m8: 0.0,
            m12: 0.0,
            m1: 0.0,
            m5: 0.0,
            m9: 0.0,
            m13: 0.0,
            m2: 0.0,
            m6: 0.0,
            m10: 0.0,
            m14: 0.0,
            m3: 0.0,
            m7: 0.0,
            m11: 0.0,
            m15: 0.0,
        }
    }

    /// Returns a translation matrix.
    pub fn translate(x: f32, y: f32, z: f32) -> Matrix {
        Matrix {
            m0: 1.0,
            m4: 0.0,
            m8: 0.0,
            m12: x,
            m1: 0.0,
            m5: 1.0,
            m9: 0.0,
            m13: y,
            m2: 0.0,
            m6: 0.0,
            m10: 1.0,
            m14: z,
            m3: 0.0,
            m7: 0.0,
            m11: 0.0,
            m15: 1.0,
        }
    }

    /// Returns a rotation matrix.
    pub fn rotate(axis: Vector3, angle: f32) -> Matrix {
        let mut x = axis.x;
        let mut y = axis.y;
        let mut z = axis.z;
        let mut length = (x * x + y * y + z * z).sqrt();

        if (length != 1.0) && (length != 0.0) {
            length = 1.0 / length;
            x *= length;
            y *= length;
            z *= length;
        }

        let sinres = angle.sin();
        let cosres = angle.cos();
        let t = 1.0 - cosres;

        Matrix {
            m0: (x * x * t) + cosres,
            m1: (y * x * t) + (z * sinres),
            m2: (z * x * t) - (y * sinres),
            m3: 0.0,

            m4: (x * y * t) - (z * sinres),
            m5: (y * y * t) + cosres,
            m6: (z * y * t) + (x * sinres),
            m7: 0.0,

            m8: (x * z * t) + (y * sinres),
            m9: (y * z * t) - (x * sinres),
            m10: (z * z * t) + cosres,
            m11: 0.0,

            m12: 0.0,
            m13: 0.0,
            m14: 0.0,
            m15: 1.0,
        }
    }

    /// Returns a translation matrix around the X axis.
    pub fn rotate_x(angle: f32) -> Matrix {
        let mut result = Matrix::identity();

        let cosres = angle.cos();
        let sinres = angle.sin();

        result.m5 = cosres;
        result.m6 = sinres;
        result.m9 = -sinres;
        result.m10 = cosres;
        result
    }

    /// Returns a translation matrix around the Y axis.
    pub fn rotate_y(angle: f32) -> Matrix {
        let mut result = Matrix::identity();

        let cosres = angle.cos();
        let sinres = angle.sin();

        result.m0 = cosres;
        result.m2 = -sinres;
        result.m8 = sinres;
        result.m10 = cosres;
        result
    }

    /// Returns a translation matrix around the Z axis.
    pub fn rotate_z(angle: f32) -> Matrix {
        let mut result = Matrix::identity();

        let cosres = angle.cos();
        let sinres = angle.sin();

        result.m0 = cosres;
        result.m1 = sinres;
        result.m4 = -sinres;
        result.m5 = cosres;
        result
    }

    /// Returns xyz-rotation matrix (angles in radians)
    pub fn rotate_xyz(ang: Vector3) -> Self {
        let mut result = Self::identity();

        let cosz = -ang.z.cos();
        let sinz = -ang.z.sin();
        let cosy = -ang.y.cos();
        let siny = -ang.y.sin();
        let cosx = -ang.x.cos();
        let sinx = -ang.x.sin();

        result.m0 = cosz * cosy;
        result.m4 = (cosz * siny * sinx) - (sinz * cosx);
        result.m8 = (cosz * siny * cosx) + (sinz * sinx);

        result.m1 = sinz * cosy;
        result.m5 = (sinz * siny * sinx) + (cosz * cosx);
        result.m9 = (sinz * siny * cosx) - (cosz * sinx);

        result.m2 = -siny;
        result.m6 = cosy * sinx;
        result.m10 = cosy * cosx;

        result
    }

    /// Returns a scaling matrix.
    pub fn scale(x: f32, y: f32, z: f32) -> Matrix {
        Matrix {
            m0: x,
            m4: 0.0,
            m8: 0.0,
            m12: 0.0,
            m1: 0.0,
            m5: y,
            m9: 0.0,
            m13: 0.0,
            m2: 0.0,
            m6: 0.0,
            m10: z,
            m14: 0.0,
            m3: 0.0,
            m7: 0.0,
            m11: 0.0,
            m15: 1.0,
        }
    }

    /// Returns perspective projection matrix based on frustum parameters.
    pub fn frustum(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> Matrix {
        let rl = right - left;
        let tb = top - bottom;
        let fne = far - near;

        Matrix {
            m0: (near * 2.0) / rl,
            m1: 0.0,
            m2: 0.0,
            m3: 0.0,

            m4: 0.0,
            m5: (near * 2.0) / tb,
            m6: 0.0,
            m7: 0.0,

            m8: (right + left) / rl,
            m9: (top + bottom) / tb,
            m10: -(far + near) / fne,
            m11: -1.0,

            m12: 0.0,
            m13: 0.0,
            m14: -(far * near * 2.0) / fne,
            m15: 0.0,
        }
    }

    /// Returns perspective projection matrix.
    pub fn perspective(fovy: f32, aspect: f32, near: f32, far: f32) -> Matrix {
        let top = near * (fovy * 0.5).tan();
        let right = top * aspect;
        Matrix::frustum(-right, right, -top, top, near, far)
    }

    /// Returns orthographic projection matrix.
    pub fn ortho(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> Matrix {
        let rl = right - left;
        let tb = top - bottom;
        let fne = far - near;

        Matrix {
            m0: 2.0 / rl,
            m1: 0.0,
            m2: 0.0,
            m3: 0.0,
            m4: 0.0,
            m5: 2.0 / tb,
            m6: 0.0,
            m7: 0.0,
            m8: 0.0,
            m9: 0.0,
            m10: -2.0 / fne,
            m11: 0.0,
            m12: -(left + right) / rl,
            m13: -(top + bottom) / tb,
            m14: -(far + near) / fne,
            m15: 1.0,
        }
    }

    /// Returns camera look-at matrix (view matrix).
    pub fn look_at(eye: Vector3, target: Vector3, up: Vector3) -> Matrix {
        let z = (eye - target).normalized();
        let x = up.cross(z).normalized();
        let y = z.cross(x).normalized();

        Matrix {
            m0: x.x,
            m1: x.y,
            m2: x.z,
            m3: 0.0,
            m4: y.x,
            m5: y.y,
            m6: y.z,
            m7: 0.0,
            m8: z.x,
            m9: z.y,
            m10: z.z,
            m11: 0.0,
            m12: eye.x,
            m13: eye.y,
            m14: eye.z,
            m15: 1.0,
        }
        .inverted()
    }

    /// Calculates the determinant of the current matrix.
    pub fn determinant(&self) -> f32 {
        let a00 = self.m0;
        let a01 = self.m1;
        let a02 = self.m2;
        let a03 = self.m3;
        let a10 = self.m4;
        let a11 = self.m5;
        let a12 = self.m6;
        let a13 = self.m7;
        let a20 = self.m8;
        let a21 = self.m9;
        let a22 = self.m10;
        let a23 = self.m11;
        let a30 = self.m12;
        let a31 = self.m13;
        let a32 = self.m14;
        let a33 = self.m15;

        a30 * a21 * a12 * a03 - a20 * a31 * a12 * a03 - a30 * a11 * a22 * a03
            + a10 * a31 * a22 * a03
            + a20 * a11 * a32 * a03
            - a10 * a21 * a32 * a03
            - a30 * a21 * a02 * a13
            + a20 * a31 * a02 * a13
            + a30 * a01 * a22 * a13
            - a00 * a31 * a22 * a13
            - a20 * a01 * a32 * a13
            + a00 * a21 * a32 * a13
            + a30 * a11 * a02 * a23
            - a10 * a31 * a02 * a23
            - a30 * a01 * a12 * a23
            + a00 * a31 * a12 * a23
            + a10 * a01 * a32 * a23
            - a00 * a11 * a32 * a23
            - a20 * a11 * a02 * a33
            + a10 * a21 * a02 * a33
            + a20 * a01 * a12 * a33
            - a00 * a21 * a12 * a33
            - a10 * a01 * a22 * a33
            + a00 * a11 * a22 * a33
    }

    /// Calculates the trace of the matrix (sum of the values along the diagonal).
    pub fn trace(&self) -> f32 {
        self.m0 + self.m5 + self.m10 + self.m15
    }

    /// Returns a new `Matrix` transposed from the current one.
    pub fn transposed(&self) -> Matrix {
        Matrix {
            m0: self.m0,
            m1: self.m4,
            m2: self.m8,
            m3: self.m12,
            m4: self.m1,
            m5: self.m5,
            m6: self.m9,
            m7: self.m13,
            m8: self.m2,
            m9: self.m6,
            m10: self.m10,
            m11: self.m14,
            m12: self.m3,
            m13: self.m7,
            m14: self.m11,
            m15: self.m15,
        }
    }

    /// Returns a new `Matrix` inverted from the current one.
    pub fn inverted(&self) -> Matrix {
        let a00 = self.m0;
        let a01 = self.m1;
        let a02 = self.m2;
        let a03 = self.m3;
        let a10 = self.m4;
        let a11 = self.m5;
        let a12 = self.m6;
        let a13 = self.m7;
        let a20 = self.m8;
        let a21 = self.m9;
        let a22 = self.m10;
        let a23 = self.m11;
        let a30 = self.m12;
        let a31 = self.m13;
        let a32 = self.m14;
        let a33 = self.m15;

        let b00 = (a00 * a11) - (a01 * a10);
        let b01 = (a00 * a12) - (a02 * a10);
        let b02 = (a00 * a13) - (a03 * a10);
        let b03 = (a01 * a12) - (a02 * a11);
        let b04 = (a01 * a13) - (a03 * a11);
        let b05 = (a02 * a13) - (a03 * a12);
        let b06 = (a20 * a31) - (a21 * a30);
        let b07 = (a20 * a32) - (a22 * a30);
        let b08 = (a20 * a33) - (a23 * a30);
        let b09 = (a21 * a32) - (a22 * a31);
        let b10 = (a21 * a33) - (a23 * a31);
        let b11 = (a22 * a33) - (a23 * a32);

        let inv_det = 1.0
            / ((b00 * b11) - (b01 * b10) + (b02 * b09) + (b03 * b08) - (b04 * b07) + (b05 * b06));

        Matrix {
            m0: ((a11 * b11) - (a12 * b10) + (a13 * b09)) * inv_det,
            m1: ((-a01 * b11) + (a02 * b10) - (a03 * b09)) * inv_det,
            m2: ((a31 * b05) - (a32 * b04) + (a33 * b03)) * inv_det,
            m3: ((-a21 * b05) + (a22 * b04) - (a23 * b03)) * inv_det,
            m4: ((-a10 * b11) + (a12 * b08) - (a13 * b07)) * inv_det,
            m5: ((a00 * b11) - (a02 * b08) + (a03 * b07)) * inv_det,
            m6: ((-a30 * b05) + (a32 * b02) - (a33 * b01)) * inv_det,
            m7: ((a20 * b05) - (a22 * b02) + (a23 * b01)) * inv_det,
            m8: ((a10 * b10) - (a11 * b08) + (a13 * b06)) * inv_det,
            m9: ((-a00 * b10) + (a01 * b08) - (a03 * b06)) * inv_det,
            m10: ((a30 * b04) - (a31 * b02) + (a33 * b00)) * inv_det,
            m11: ((-a20 * b04) + (a21 * b02) - (a23 * b00)) * inv_det,
            m12: ((-a10 * b09) + (a11 * b07) - (a12 * b06)) * inv_det,
            m13: ((a00 * b09) - (a01 * b07) + (a02 * b06)) * inv_det,
            m14: ((-a30 * b03) + (a31 * b01) - (a32 * b00)) * inv_det,
            m15: ((a20 * b03) - (a21 * b01) + (a22 * b00)) * inv_det,
        }
    }

    /// Returns a new `Matrix` normalized from the current one.
    pub fn normalized(&self) -> Matrix {
        let det = self.determinant();
        Matrix {
            m0: self.m0 / det,
            m1: self.m1 / det,
            m2: self.m2 / det,
            m3: self.m3 / det,
            m4: self.m4 / det,
            m5: self.m5 / det,
            m6: self.m6 / det,
            m7: self.m7 / det,
            m8: self.m8 / det,
            m9: self.m9 / det,
            m10: self.m10 / det,
            m11: self.m11 / det,
            m12: self.m12 / det,
            m13: self.m13 / det,
            m14: self.m14 / det,
            m15: self.m15 / det,
        }
    }

    /// Returns a 16-length `f32` array containing the current matrix data.
    pub fn to_array(&self) -> [f32; 16] {
        [
            self.m0, self.m1, self.m2, self.m3, self.m4, self.m5, self.m6, self.m7, self.m8,
            self.m9, self.m10, self.m11, self.m12, self.m13, self.m14, self.m15,
        ]
    }
}

impl Add for Matrix {
    type Output = Matrix;
    fn add(self, mat: Matrix) -> Matrix {
        Matrix {
            m0: self.m0 + mat.m0,
            m1: self.m1 + mat.m1,
            m2: self.m2 + mat.m2,
            m3: self.m3 + mat.m3,
            m4: self.m4 + mat.m4,
            m5: self.m5 + mat.m5,
            m6: self.m6 + mat.m6,
            m7: self.m7 + mat.m7,
            m8: self.m8 + mat.m8,
            m9: self.m9 + mat.m9,
            m10: self.m10 + mat.m10,
            m11: self.m11 + mat.m11,
            m12: self.m12 + mat.m12,
            m13: self.m13 + mat.m13,
            m14: self.m14 + mat.m14,
            m15: self.m15 + mat.m15,
        }
    }
}

impl AddAssign for Matrix {
    fn add_assign(&mut self, mat: Matrix) {
        *self = *self + mat;
    }
}

impl Sub for Matrix {
    type Output = Matrix;
    fn sub(self, mat: Matrix) -> Matrix {
        Matrix {
            m0: self.m0 - mat.m0,
            m1: self.m1 - mat.m1,
            m2: self.m2 - mat.m2,
            m3: self.m3 - mat.m3,
            m4: self.m4 - mat.m4,
            m5: self.m5 - mat.m5,
            m6: self.m6 - mat.m6,
            m7: self.m7 - mat.m7,
            m8: self.m8 - mat.m8,
            m9: self.m9 - mat.m9,
            m10: self.m10 - mat.m10,
            m11: self.m11 - mat.m11,
            m12: self.m12 - mat.m12,
            m13: self.m13 - mat.m13,
            m14: self.m14 - mat.m14,
            m15: self.m15 - mat.m15,
        }
    }
}

impl SubAssign for Matrix {
    fn sub_assign(&mut self, mat: Matrix) {
        *self = *self - mat;
    }
}

impl Mul for Matrix {
    type Output = Matrix;
    fn mul(self, mat: Matrix) -> Matrix {
        Matrix {
            m0: self.m0 * mat.m0 + self.m1 * mat.m4 + self.m2 * mat.m8 + self.m3 * mat.m12,
            m1: self.m0 * mat.m1 + self.m1 * mat.m5 + self.m2 * mat.m9 + self.m3 * mat.m13,
            m2: self.m0 * mat.m2 + self.m1 * mat.m6 + self.m2 * mat.m10 + self.m3 * mat.m14,
            m3: self.m0 * mat.m3 + self.m1 * mat.m7 + self.m2 * mat.m11 + self.m3 * mat.m15,
            m4: self.m4 * mat.m0 + self.m5 * mat.m4 + self.m6 * mat.m8 + self.m7 * mat.m12,
            m5: self.m4 * mat.m1 + self.m5 * mat.m5 + self.m6 * mat.m9 + self.m7 * mat.m13,
            m6: self.m4 * mat.m2 + self.m5 * mat.m6 + self.m6 * mat.m10 + self.m7 * mat.m14,
            m7: self.m4 * mat.m3 + self.m5 * mat.m7 + self.m6 * mat.m11 + self.m7 * mat.m15,
            m8: self.m8 * mat.m0 + self.m9 * mat.m4 + self.m10 * mat.m8 + self.m11 * mat.m12,
            m9: self.m8 * mat.m1 + self.m9 * mat.m5 + self.m10 * mat.m9 + self.m11 * mat.m13,
            m10: self.m8 * mat.m2 + self.m9 * mat.m6 + self.m10 * mat.m10 + self.m11 * mat.m14,
            m11: self.m8 * mat.m3 + self.m9 * mat.m7 + self.m10 * mat.m11 + self.m11 * mat.m15,
            m12: self.m12 * mat.m0 + self.m13 * mat.m4 + self.m14 * mat.m8 + self.m15 * mat.m12,
            m13: self.m12 * mat.m1 + self.m13 * mat.m5 + self.m14 * mat.m9 + self.m15 * mat.m13,
            m14: self.m12 * mat.m2 + self.m13 * mat.m6 + self.m14 * mat.m10 + self.m15 * mat.m14,
            m15: self.m12 * mat.m3 + self.m13 * mat.m7 + self.m14 * mat.m11 + self.m15 * mat.m15,
        }
    }
}

impl MulAssign for Matrix {
    fn mul_assign(&mut self, mat: Matrix) {
        *self = *self * mat;
    }
}

optional_serde_struct! {
    pub struct Ray {
        pub position: Vector3,
        pub direction: Vector3,
    }
}

impl From<ffi::Ray> for Ray {
    fn from(r: ffi::Ray) -> Ray {
        unsafe { std::mem::transmute(r) }
    }
}

impl From<Ray> for ffi::Ray {
    fn from(v: Ray) -> ffi::Ray {
        unsafe { std::mem::transmute(v) }
    }
}

impl From<&Ray> for ffi::Ray {
    fn from(v: &Ray) -> ffi::Ray {
        ffi::Ray {
            position: v.position.into(),
            direction: v.direction.into(),
        }
    }
}

impl Ray {
    pub const fn new(position: Vector3, direction: Vector3) -> Self {
        Self {
            position,
            direction,
        }
    }
}

optional_serde_struct! {
    pub struct Rectangle {
        pub x: f32,
        pub y: f32,
        pub width: f32,
        pub height: f32,
    }
}

impl From<ffi::Rectangle> for Rectangle {
    fn from(r: ffi::Rectangle) -> Rectangle {
        unsafe { std::mem::transmute(r) }
    }
}

impl From<Rectangle> for ffi::Rectangle {
    fn from(v: Rectangle) -> ffi::Rectangle {
        unsafe { std::mem::transmute(v) }
    }
}

impl From<&Rectangle> for ffi::Rectangle {
    fn from(v: &Rectangle) -> ffi::Rectangle {
        ffi::Rectangle {
            x: v.x,
            y: v.y,
            width: v.width,
            height: v.height,
        }
    }
}

impl Rectangle {
    pub const EMPTY: Rectangle = Rectangle::new(0.0, 0.0, 0.0, 0.0);
    pub const fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }
}

optional_serde_struct! {
    pub struct BoundingBox {
        pub min: Vector3,
        pub max: Vector3,
    }
}

impl BoundingBox {
    pub fn new(min: Vector3, max: Vector3) -> BoundingBox {
        BoundingBox { min, max }
    }
}

impl From<ffi::BoundingBox> for BoundingBox {
    fn from(r: ffi::BoundingBox) -> BoundingBox {
        unsafe { std::mem::transmute(r) }
    }
}

impl From<BoundingBox> for ffi::BoundingBox {
    fn from(v: BoundingBox) -> ffi::BoundingBox {
        unsafe { std::mem::transmute(v) }
    }
}

impl From<&BoundingBox> for ffi::BoundingBox {
    fn from(v: &BoundingBox) -> ffi::BoundingBox {
        ffi::BoundingBox {
            min: v.min.into(),
            max: v.max.into(),
        }
    }
}

optional_serde_struct! {
    pub struct RayCollision {
        pub hit: bool,
        pub distance: f32,
        pub point: Vector3,
        pub normal: Vector3,
    }
}

impl From<ffi::RayCollision> for RayCollision {
    fn from(r: ffi::RayCollision) -> RayCollision {
        unsafe { std::mem::transmute(r) }
    }
}

impl From<RayCollision> for ffi::RayCollision {
    fn from(v: RayCollision) -> ffi::RayCollision {
        unsafe { std::mem::transmute(v) }
    }
}

impl From<&RayCollision> for ffi::RayCollision {
    fn from(v: &RayCollision) -> ffi::RayCollision {
        ffi::RayCollision {
            hit: v.hit,
            distance: v.distance,
            point: v.point.into(),
            normal: v.normal.into(),
        }
    }
}

optional_serde_struct! {
    pub struct Transform {
        pub translation: Vector3,
        pub rotation: Quaternion,
        pub scale: Vector3,
    }
}

impl From<ffi::Transform> for Transform {
    fn from(r: ffi::Transform) -> Transform {
        unsafe { std::mem::transmute(r) }
    }
}

impl From<Transform> for ffi::Transform {
    fn from(v: Transform) -> ffi::Transform {
        unsafe { std::mem::transmute(v) }
    }
}

impl From<&Transform> for ffi::Transform {
    fn from(v: &Transform) -> ffi::Transform {
        ffi::Transform {
            translation: v.translation.into(),
            rotation: v.rotation.into(),
            scale: v.scale.into(),
        }
    }
}

#[cfg(test)]
mod math_test {
    use super::{Ray, Vector2, Vector3, Vector4};
    use crate::ffi;

    #[test]
    fn test_into() {
        let v2: ffi::Vector2 = (Vector2 { x: 1.0, y: 2.0 }).into();
        assert!(v2.x == 1.0 && v2.y == 2.0, "bad memory transmutation");

        let v3: ffi::Vector3 = (Vector3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        })
        .into();
        assert!(
            v3.x == 1.0 && v3.y == 2.0 && v3.z == 3.0,
            "bad memory transmutation"
        );

        let v4: ffi::Vector4 = (Vector4 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
            w: 4.0,
        })
        .into();
        assert!(
            v4.x == 1.0 && v4.y == 2.0 && v4.z == 3.0 && v4.w == 4.0,
            "bad memory transmutation"
        );

        let r: ffi::Ray = (Ray {
            position: v3.into(),
            direction: Vector3 {
                x: 3.0,
                y: 2.0,
                z: 1.0,
            },
        })
        .into();
        assert!(
            r.position.x == 1.0
                && r.position.y == 2.0
                && r.position.z == 3.0
                && r.direction.x == 3.0
                && r.direction.y == 2.0
                && r.direction.z == 1.0,
            "bad memory transmutation"
        )
    }
}
