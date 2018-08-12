/* raylib-rs
   raymath.rs - Structs and functions for game-related math and linear algebra

Copyright (c) 2018 Paul Clement (@deltaphc)

This software is provided "as-is", without any express or implied warranty. In no event will the authors be held liable for any damages arising from the use of this software.

Permission is granted to anyone to use this software for any purpose, including commercial applications, and to alter it and redistribute it freely, subject to the following restrictions:

  1. The origin of this software must not be misrepresented; you must not claim that you wrote the original software. If you use this software in a product, an acknowledgment in the product documentation would be appreciated but is not required.

  2. Altered source versions must be plainly marked as such, and must not be misrepresented as being the original software.

  3. This notice may not be removed or altered from any source distribution.
*/

use std::f32::consts::PI;
use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Neg};

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

impl Vector2 {
    /// Returns a new `Vector2` with both components set to zero.
    pub fn zero() -> Vector2 {
        Vector2 {
            x: 0.0,
            y: 0.0,
        }
    }

    /// Returns a new `Vector2` with both components set to one.
    pub fn one() -> Vector2 {
        Vector2 {
            x: 1.0,
            y: 1.0,
        }
    }

    /// Calculates the vector length.
    pub fn length(&self) -> f32 {
        ((self.x * self.x) + (self.y * self.y)).sqrt()
    }

    /// Calculates the dot product with vector `v`.
    pub fn dot(&self, v: Vector2) -> f32 {
        (self.x * v.x + self.y * v.y)
    }

    /// Calculates the distance towards vector `v`.
    pub fn distance_to(&self, v: Vector2) -> f32 {
        ((self.x - v.x) * (self.x - v.x) + (self.y - v.y) * (self.y - v.y)).sqrt()
    }

    /// Calculates the angle towards vector `v` in radians.
    pub fn angle_to(&self, v: Vector2) -> f32 {
        let mut result = (v.y - self.y).atan2(v.x - self.x);
        if result < 0.0 { result += 2.0 * PI; }
        result
    }

    /// Scales the vector by multiplying both components by `scale`.
    pub fn scale(&mut self, scale: f32) {
        *self *= scale;
    }

    /// Normalizes the vector.
    pub fn normalize(&mut self) {
        *self /= self.length();
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

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3 {
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
        let mut cardinal_axis = Vector3 { x: 1.0, y: 0.0, z: 0.0 };

        if self.y.abs() < min {
            min = self.y.abs();
            cardinal_axis = Vector3 { x: 0.0, y: 1.0, z: 0.0 };
        }

        if self.z.abs() < min {
            cardinal_axis = Vector3 { x: 0.0, y: 0.0, z: 1.0 };
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

    /// Returns a new `Vector3` with components scaled by `scale`.
    pub fn scale_by(&self, scale: f32) -> Vector3 {
        *self * scale
    }

    /// Returns a new `Vector3` with normalized components from the current vector.
    pub fn normalized(&self) -> Vector3 {
        let mut length = self.length();
        if length == 0.0 { length = 1.0; }
        let ilength = 1.0 / length;

        Vector3 {
            x: self.x * ilength,
            y: self.x * ilength,
            z: self.x * ilength,
        }
    }

    /// Normalizes and changes both `self` and `v` to be orthogonal to eachother.
    pub fn ortho_normalize(&mut self, v: &mut Vector3) {
        *self = self.normalized();
        let vn = self.cross(*v).normalized();
        *v = vn.cross(*self);
    }

    /// Returns a new `Vector3` containing components transformed by Matrix `mat`.
    pub fn transform(&self, mat: Matrix) -> Vector3 {
        Vector3 {
            x: mat.m0 * self.x + mat.m4 * self.y + mat.m8 * self.z + mat.m12,
            y: mat.m1 * self.x + mat.m5 * self.y + mat.m9 * self.z + mat.m13,
            z: mat.m2 * self.x + mat.m6 * self.y + mat.m10 * self.z + mat.m14,
        }
    }

    /// Returns a new `Vector3` with components rotated by Quaternion `q`.
    pub fn rotate_by(&self, q: Quaternion) -> Vector3 {
        Vector3 {
            x: self.x * (q.x * q.x + q.w * q.w - q.y * q.y - q.z * q.z) +
               self.y * (2.0 * q.x * q.y - 2.0 * q.w * q.z) +
               self.z * (2.0 * q.x * q.z + 2.0 * q.w * q.y),
            y: self.x * (2.0 * q.w * q.z + 2.0 * q.x * q.y) +
               self.y * (q.w * q.w - q.x * q.x + q.y * q.y - q.z * q.z) +
               self.z * (-2.0 * q.w * q.x + 2.0 * q.y * q.z),
            z: self.x * (-2.0 * q.w * q.y + 2.0 * q.x * q.z) +
               self.y * (2.0 * q.w * q.x + 2.0 * q.y * q.z) +
               self.z * (q.w * q.w - q.x * q.x - q.y * q.y + q.z * q.z),
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

    /// Returns a new `Vector3` reflected from the current vector using `normal`.
    pub fn reflect(&self, normal: Vector3) -> Vector3 {
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

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Vector4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}
pub type Quaternion = Vector4;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
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
