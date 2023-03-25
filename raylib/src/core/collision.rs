//! Common collision handling code
use super::models::Mesh;
use crate::ffi::{self, Matrix, Ray, RayCollision, Vector2, Vector3};

// Collision Handling
/// Checks collision between two circles.
#[inline]
pub fn check_collision_circles(
    center1: Vector2,
    radius1: f32,
    center2: Vector2,
    radius2: f32,
) -> bool {
    unsafe { ffi::CheckCollisionCircles(center1, radius1, center2, radius2) }
}

/// Checks if point is inside circle.
#[inline]
pub fn check_collision_point_circle(point: Vector2, center: Vector2, radius: f32) -> bool {
    unsafe { ffi::CheckCollisionPointCircle(point, center, radius) }
}

/// Checks if point is inside a triangle.
#[inline]
pub fn check_collision_point_triangle(
    point: Vector2,
    p1: Vector2,
    p2: Vector2,
    p3: Vector2,
) -> bool {
    unsafe { ffi::CheckCollisionPointTriangle(point, p1, p2, p3) }
}

#[inline]
pub fn check_collision_point_poly(point: Vector2, points: &[Vector2]) -> bool {
    unsafe { ffi::CheckCollisionPointPoly(point, points.as_ptr() as _, points.len() as _) }
}

/// Check the collision between two lines defined by two points each, returns collision point by reference
#[inline]
pub fn check_collision_lines(
    start_pos1: Vector2,
    end_pos1: Vector2,
    start_pos2: Vector2,
    end_pos2: Vector2,
) -> Option<Vector2> {
    let mut out = ffi::Vector2 { x: 0.0, y: 0.0 };

    let collision =
        unsafe { ffi::CheckCollisionLines(start_pos1, end_pos1, start_pos2, end_pos2, &mut out) };

    collision.then_some(out)
}

#[inline]
pub fn check_collision_point_line(
    point: Vector2,
    p1: Vector2,
    p2: Vector2,
    threshold: i32,
) -> bool {
    unsafe { ffi::CheckCollisionPointLine(point, p1, p2, threshold) }
}

/// Detects collision between two spheres.
#[inline]
pub fn check_collision_spheres(
    center_a: Vector3,
    radius_a: f32,
    center_b: Vector3,
    radius_b: f32,
) -> bool {
    unsafe { ffi::CheckCollisionSpheres(center_a, radius_a, center_b, radius_b) }
}

/// Detects collision between ray and sphere.
#[inline]
pub fn get_ray_collision_sphere(
    ray: Ray,
    sphere_position: Vector3,
    sphere_radius: f32,
) -> RayCollision {
    unsafe { ffi::GetRayCollisionSphere(ray, sphere_position, sphere_radius) }
}

/// Gets collision info between ray and model.
#[inline]
pub fn get_ray_collision_mesh(ray: Ray, model: &Mesh, transform: Matrix) -> RayCollision {
    unsafe { ffi::GetRayCollisionMesh(ray, model.0, transform) }
}

/// Gets collision info between ray and triangle.
#[inline]
pub fn get_ray_collision_triangle(ray: Ray, p1: Vector3, p2: Vector3, p3: Vector3) -> RayCollision {
    unsafe { ffi::GetRayCollisionTriangle(ray, p1, p2, p3) }
}

/// Gets collision info between ray and model.
#[inline]
pub fn get_ray_collision_quad(
    ray: Ray,
    p1: Vector3,
    p2: Vector3,
    p3: Vector3,
    p4: Vector3,
) -> RayCollision {
    unsafe { ffi::GetRayCollisionQuad(ray, p1, p2, p3, p4) }
}
