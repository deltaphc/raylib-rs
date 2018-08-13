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

    /// Returns a new `Vector2` with components scaled by `scale`.
    pub fn scale_by(&self, scale: f32) -> Vector2 {
        *self * scale
    }

    /// Normalizes the vector.
    pub fn normalize(&mut self) {
        *self /= self.length();
    }

    /// Returns a new `Vector2` with normalized components from the current vector.
    pub fn normalized(&self) -> Vector2 {
        *self / self.length()
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

impl Matrix {
    /// Returns the identity matrix.
    pub fn identity() -> Matrix {
        Matrix {
            m0: 1.0, m4: 0.0, m8: 0.0, m12: 0.0,
            m1: 0.0, m5: 1.0, m9: 0.0, m13: 0.0,
            m2: 0.0, m6: 0.0, m10: 1.0, m14: 0.0,
            m3: 0.0, m7: 0.0, m11: 0.0, m15: 1.0,
        }
    }

    /// Returns a translation matrix.
    pub fn translate(x: f32, y: f32, z: f32) -> Matrix {
        Matrix {
            m0: 1.0, m4: 0.0, m8: 0.0, m12: x,
            m1: 0.0, m5: 1.0, m9: 0.0, m13: y,
            m2: 0.0, m6: 0.0, m10: 1.0, m14: z,
            m3: 0.0, m7: 0.0, m11: 0.0, m15: 1.0,
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
        result.m6 = -sinres;
        result.m9 = sinres;
        result.m10 = cosres;
        result
    }

    /// Returns a translation matrix around the Y axis.
    pub fn rotate_y(angle: f32) -> Matrix {
        let mut result = Matrix::identity();

        let cosres = angle.cos();
        let sinres = angle.sin();

        result.m0 = cosres;
        result.m2 = sinres;
        result.m8 = -sinres;
        result.m10 = cosres;
        result
    }

    /// Returns a translation matrix around the Z axis.
    pub fn rotate_z(angle: f32) -> Matrix {
        let mut result = Matrix::identity();

        let cosres = angle.cos();
        let sinres = angle.sin();

        result.m0 = cosres;
        result.m1 = -sinres;
        result.m4 = sinres;
        result.m5 = cosres;
        result
    }

    /// Returns a scaling matrix.
    pub fn scale(x: f32, y: f32, z: f32) -> Matrix {
        Matrix {
            m0: x, m4: 0.0, m8: 0.0, m12: 0.0,
            m1: 0.0, m5: y, m9: 0.0, m13: 0.0,
            m2: 0.0, m6: 0.0, m10: z, m14: 0.0,
            m3: 0.0, m7: 0.0, m11: 0.0, m15: 1.0,
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
            m14: -(far * near * 2.0)/fne,
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
        }.inverted()
    }

    /// Calculates the determinant of the current matrix.
    pub fn determinant(&self) -> f32 {
        let a00 = self.m0; let a01 = self.m1; let a02 = self.m2; let a03 = self.m3;
        let a10 = self.m4; let a11 = self.m5; let a12 = self.m6; let a13 = self.m7;
        let a20 = self.m8; let a21 = self.m9; let a22 = self.m10; let a23 = self.m11;
        let a30 = self.m12; let a31 = self.m13; let a32 = self.m14; let a33 = self.m15;

        a30*a21*a12*a03 - a20*a31*a12*a03 - a30*a11*a22*a03 + a10*a31*a22*a03 +
        a20*a11*a32*a03 - a10*a21*a32*a03 - a30*a21*a02*a13 + a20*a31*a02*a13 +
        a30*a01*a22*a13 - a00*a31*a22*a13 - a20*a01*a32*a13 + a00*a21*a32*a13 +
        a30*a11*a02*a23 - a10*a31*a02*a23 - a30*a01*a12*a23 + a00*a31*a12*a23 +
        a10*a01*a32*a23 - a00*a11*a32*a23 - a20*a11*a02*a33 + a10*a21*a02*a33 +
        a20*a01*a12*a33 - a00*a21*a12*a33 - a10*a01*a22*a33 + a00*a11*a22*a33
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
        let a00 = self.m0; let a01 = self.m1; let a02 = self.m2; let a03 = self.m3;
        let a10 = self.m4; let a11 = self.m5; let a12 = self.m6; let a13 = self.m7;
        let a20 = self.m8; let a21 = self.m9; let a22 = self.m10; let a23 = self.m11;
        let a30 = self.m12; let a31 = self.m13; let a32 = self.m14; let a33 = self.m15;

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

        let inv_det = 1.0 / ((b00 * b11) - (b01 * b10) + (b02 * b09) + (b03 * b08) - (b04 * b07) + (b05 * b06));

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
        [self.m0, self.m1, self.m2, self.m3,
        self.m4, self.m5, self.m6, self.m7,
        self.m8, self.m9, self.m10, self.m11,
        self.m12, self.m13, self.m14, self.m15]
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
