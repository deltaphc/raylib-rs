use crate::{BoundingBox, Ray, RayCollision, Rectangle};
use mint::{Vector2, Vector3};
use std::mem;

macro_rules! mint_transmutable {
    ($ffit:ty, $mt:ty) => {
        impl From<$mt> for $ffit {
            fn from(value: $mt) -> Self {
                unsafe { mem::transmute(value) }
            }
        }

        impl From<$ffit> for $mt {
            fn from(value: $ffit) -> Self {
                unsafe { mem::transmute(value) }
            }
        }
    };
}

mint_transmutable!(crate::Vector2, mint::Vector2<f32>);
mint_transmutable!(crate::Vector3, mint::Vector3<f32>);
mint_transmutable!(crate::Vector4, mint::Vector4<f32>);

mint_transmutable!(crate::Matrix, mint::ColumnMatrix4<f32>);
mint_transmutable!(crate::Quaternion, mint::Quaternion<f32>);

impl Rectangle {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    /// Check collision between two rectangles
    #[inline]
    pub fn check_collision_recs(&self, other: Rectangle) -> bool {
        unsafe { crate::CheckCollisionRecs(*self, other) }
    }

    /// Checks collision between circle and rectangle.
    #[inline]
    pub fn check_collision_circle_rec(&self, center: Vector2<f32>, radius: f32) -> bool {
        unsafe { crate::CheckCollisionCircleRec(center.into(), radius, *self) }
    }

    /// Gets the overlap between two colliding rectangles.
    /// ```rust
    /// use raylib::prelude::*;
    /// fn main() {
    ///    let r1 = Rectangle::new(0.0, 0.0, 10.0, 10.0);
    ///    let r2 = Rectangle::new(20.0, 20.0, 10.0, 10.0);
    ///    assert_eq!(None, r1.get_collision_rec(&r2));
    ///    assert_eq!(Some(r1), r1.get_collision_rec(&r1));
    /// }
    /// ```
    #[inline]
    pub fn get_collision_rec(&self, other: Rectangle) -> Option<Rectangle> {
        self.check_collision_recs(other)
            .then(|| unsafe { crate::GetCollisionRec(*self, other) })
    }

    /// Checks if point is inside rectangle.
    #[inline]
    pub fn check_collision_point_rec(&self, point: Vector2<f32>) -> bool {
        unsafe { crate::CheckCollisionPointRec(point.into(), *self) }
    }
}

impl BoundingBox {
    /// Detects collision between two boxes.
    #[inline]
    pub fn check_collision_boxes(&self, box2: BoundingBox) -> bool {
        unsafe { crate::CheckCollisionBoxes(*self, box2) }
    }

    /// Detects collision between box and sphere.
    #[inline]
    pub fn check_collision_box_sphere(
        &self,
        center_sphere: Vector3<f32>,
        radius_sphere: f32,
    ) -> bool {
        unsafe { crate::CheckCollisionBoxSphere(*self, center_sphere.into(), radius_sphere) }
    }

    /// Detects collision between ray and box.
    #[inline]
    pub fn get_ray_collision_box(&self, ray: Ray) -> RayCollision {
        unsafe { crate::GetRayCollisionBox(ray, *self) }
    }
}

#[test]
fn math_test() {
    // TODO: Do some transmutation tests.
}
