//! Common collision handling code
use mint::{ColumnMatrix4, Vector2, Vector3};
use raylib_sys::{Ray, RayCollision};

use super::models::Mesh;
use crate::ffi;

// Collision Handling
/// Checks collision between two circles.
#[inline]
pub fn check_collision_circles(
    center1: Vector2<f32>,
    radius1: f32,
    center2: Vector2<f32>,
    radius2: f32,
) -> bool {
    unsafe { ffi::CheckCollisionCircles(center1.into(), radius1, center2.into(), radius2) }
}

/// Checks if point is inside circle.
#[inline]
pub fn check_collision_point_circle(
    point: Vector2<f32>,
    center: Vector2<f32>,
    radius: f32,
) -> bool {
    unsafe { ffi::CheckCollisionPointCircle(point.into(), center.into(), radius) }
}

/// Checks if point is inside a triangle.
#[inline]
pub fn check_collision_point_triangle(
    point: Vector2<f32>,
    p1: Vector2<f32>,
    p2: Vector2<f32>,
    p3: Vector2<f32>,
) -> bool {
    unsafe { ffi::CheckCollisionPointTriangle(point.into(), p1.into(), p2.into(), p3.into()) }
}

#[inline]
pub fn check_collision_point_poly(point: Vector2<f32>, points: &[Vector2<f32>]) -> bool {
    unsafe { ffi::CheckCollisionPointPoly(point.into(), points.as_ptr() as _, points.len() as _) }
}

/// Check the collision between two lines defined by two points each, returns collision point by reference
#[inline]
pub fn check_collision_lines(
    start_pos1: Vector2<f32>,
    end_pos1: Vector2<f32>,
    start_pos2: Vector2<f32>,
    end_pos2: Vector2<f32>,
) -> Option<Vector2<f32>> {
    let mut out = ffi::Vector2 { x: 0.0, y: 0.0 };

    let collision = unsafe {
        ffi::CheckCollisionLines(
            start_pos1.into(),
            end_pos1.into(),
            start_pos2.into(),
            end_pos2.into(),
            &mut out,
        )
    };

    collision.then_some(out.into())
}

#[inline]
pub fn check_collision_point_line(
    point: Vector2<f32>,
    p1: Vector2<f32>,
    p2: Vector2<f32>,
    threshold: i32,
) -> bool {
    unsafe { ffi::CheckCollisionPointLine(point.into(), p1.into(), p2.into(), threshold) }
}

/// Detects collision between two spheres.
#[inline]
pub fn check_collision_spheres(
    center_a: Vector3<f32>,
    radius_a: f32,
    center_b: Vector3<f32>,
    radius_b: f32,
) -> bool {
    unsafe { ffi::CheckCollisionSpheres(center_a.into(), radius_a, center_b.into(), radius_b) }
}

/// Detects collision between ray and sphere.
#[inline]
pub fn get_ray_collision_sphere(
    ray: Ray,
    sphere_position: Vector3<f32>,
    sphere_radius: f32,
) -> RayCollision {
    unsafe { ffi::GetRayCollisionSphere(ray, sphere_position.into(), sphere_radius) }
}

/// Gets collision info between ray and model.
#[inline]
pub fn get_ray_collision_mesh(
    ray: Ray,
    model: &Mesh,
    transform: ColumnMatrix4<f32>,
) -> RayCollision {
    unsafe { ffi::GetRayCollisionMesh(ray, model.0, transform.into()) }
}

/// Gets collision info between ray and triangle.
#[inline]
pub fn get_ray_collision_triangle(
    ray: Ray,
    p1: Vector3<f32>,
    p2: Vector3<f32>,
    p3: Vector3<f32>,
) -> RayCollision {
    unsafe { ffi::GetRayCollisionTriangle(ray, p1.into(), p2.into(), p3.into()) }
}

/// Gets collision info between ray and model.
#[inline]
pub fn get_ray_collision_quad(
    ray: Ray,
    p1: Vector3<f32>,
    p2: Vector3<f32>,
    p3: Vector3<f32>,
    p4: Vector3<f32>,
) -> RayCollision {
    unsafe { ffi::GetRayCollisionQuad(ray, p1.into(), p2.into(), p3.into(), p4.into()) }
}
