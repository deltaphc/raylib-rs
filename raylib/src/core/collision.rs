//! Common collision handling code
use crate::core::math::{BoundingBox, Ray, RayHitInfo, Rectangle, Vector3};
use crate::core::models::Model;
use crate::ffi;

impl Rectangle {
    /// Check collision between two rectangles
    #[inline]
    pub fn check_collision_recs(&self, other: &Rectangle) -> bool {
        unsafe { ffi::CheckCollisionRecs(self.into(), other.into()) }
    }

    /// Checks collision between circle and rectangle.
    #[inline]
    pub fn check_collision_circle_rec(&self, center: impl Into<ffi::Vector2>, radius: f32) -> bool {
        unsafe { ffi::CheckCollisionCircleRec(center.into(), radius, self.into()) }
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
    pub fn get_collision_rec(&self, other: &Rectangle) -> Option<Rectangle> {
        if self.check_collision_recs(other) {
            return Some(unsafe { ffi::GetCollisionRec(self.into(), other.into()).into() });
        }
        return None;
    }

    /// Checks if point is inside rectangle.
    #[inline]
    pub fn check_collision_point_rec(&self, point: impl Into<ffi::Vector2>) -> bool {
        unsafe { ffi::CheckCollisionPointRec(point.into(), self.into()) }
    }
}

// Collision Handling
/// Checks collision between two circles.
#[inline]
pub fn check_collision_circles(
    center1: impl Into<ffi::Vector2>,
    radius1: f32,
    center2: impl Into<ffi::Vector2>,
    radius2: f32,
) -> bool {
    unsafe { ffi::CheckCollisionCircles(center1.into(), radius1, center2.into(), radius2) }
}

/// Checks if point is inside circle.
#[inline]
pub fn check_collision_point_circle(
    point: impl Into<ffi::Vector2>,
    center: impl Into<ffi::Vector2>,
    radius: f32,
) -> bool {
    unsafe { ffi::CheckCollisionPointCircle(point.into(), center.into(), radius) }
}

/// Checks if point is inside a triangle.
#[inline]
pub fn check_collision_point_triangle(
    point: impl Into<ffi::Vector2>,
    p1: impl Into<ffi::Vector2>,
    p2: impl Into<ffi::Vector2>,
    p3: impl Into<ffi::Vector2>,
) -> bool {
    unsafe { ffi::CheckCollisionPointTriangle(point.into(), p1.into(), p2.into(), p3.into()) }
}

/// Detects collision between two spheres.
#[inline]
pub fn check_collision_spheres(
    center_a: impl Into<ffi::Vector3>,
    radius_a: f32,
    center_b: impl Into<ffi::Vector3>,
    radius_b: f32,
) -> bool {
    unsafe { ffi::CheckCollisionSpheres(center_a.into(), radius_a, center_b.into(), radius_b) }
}

impl BoundingBox {
    /// Detects collision between two boxes.
    #[inline]
    pub fn check_collision_boxes(&self, box2: BoundingBox) -> bool {
        unsafe { ffi::CheckCollisionBoxes(self.into(), box2.into()) }
    }

    /// Detects collision between box and sphere.
    #[inline]
    pub fn check_collision_box_sphere(
        &self,
        center_sphere: impl Into<ffi::Vector3>,
        radius_sphere: f32,
    ) -> bool {
        unsafe { ffi::CheckCollisionBoxSphere(self.into(), center_sphere.into(), radius_sphere) }
    }

    /// Detects collision between ray and box.
    #[inline]
    pub fn check_collision_ray_box(&self, ray: Ray) -> bool {
        unsafe { ffi::CheckCollisionRayBox(ray.into(), self.into()) }
    }
}

/// Detects collision between ray and sphere.
#[inline]
pub fn check_collision_ray_sphere(
    ray: Ray,
    sphere_position: impl Into<ffi::Vector3>,
    sphere_radius: f32,
) -> bool {
    unsafe { ffi::CheckCollisionRaySphere(ray.into(), sphere_position.into(), sphere_radius) }
}

/// Detects collision between ray and sphere, and returns the collision point.
#[inline]
pub fn check_collision_ray_sphere_ex(
    ray: Ray,
    sphere_position: impl Into<ffi::Vector3>,
    sphere_radius: f32,
) -> Option<Vector3> {
    unsafe {
        let mut col_point = ffi::Vector3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        let collision = ffi::CheckCollisionRaySphereEx(
            ray.into(),
            sphere_position.into(),
            sphere_radius,
            &mut col_point,
        );
        if collision {
            Some(col_point.into())
        } else {
            None
        }
    }
}

/// Gets collision info between ray and model.
#[inline]
pub fn get_collision_ray_model(ray: Ray, model: &Model) -> RayHitInfo {
    unsafe { ffi::GetCollisionRayModel(ray.into(), model.0 ).into() }
}

/// Gets collision info between ray and triangle.
#[inline]
pub fn get_collision_ray_triangle(
    ray: Ray,
    p1: impl Into<ffi::Vector3>,
    p2: impl Into<ffi::Vector3>,
    p3: impl Into<ffi::Vector3>,
) -> RayHitInfo {
    unsafe { ffi::GetCollisionRayTriangle(ray.into(), p1.into(), p2.into(), p3.into()).into() }
}

/// Gets collision info between ray and ground plane (Y-normal plane).
#[inline]
pub fn get_collision_ray_ground(ray: Ray, ground_height: f32) -> RayHitInfo {
    unsafe { ffi::GetCollisionRayGround(ray.into(), ground_height).into() }
}
