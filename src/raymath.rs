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
