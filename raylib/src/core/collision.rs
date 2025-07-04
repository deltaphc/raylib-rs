//! Common collision handling code
use crate::core::math::Vector2;

use crate::ffi;
use crate::math::{Matrix, RayCollision};
use crate::models::Mesh;

/// Check if circle collides with a line created betweeen two points [p1] and [p2]
#[inline]
#[must_use]
pub fn check_collision_circle_line(
    center: impl Into<ffi::Vector2>,
    radius: f32,
    p1: impl Into<ffi::Vector2>,
    p2: impl Into<ffi::Vector2>,
) -> bool {
    unsafe { ffi::CheckCollisionCircleLine(center.into(), radius, p1.into(), p2.into()) }
}

// Collision Handling
/// Checks collision between two circles.
#[inline]
#[must_use]
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
#[must_use]
pub fn check_collision_point_circle(
    point: impl Into<ffi::Vector2>,
    center: impl Into<ffi::Vector2>,
    radius: f32,
) -> bool {
    unsafe { ffi::CheckCollisionPointCircle(point.into(), center.into(), radius) }
}

/// Check if point is within a polygon described by array of vertices
#[inline]
#[must_use]
pub fn check_collision_point_poly(point: impl Into<ffi::Vector2>, points: &[Vector2]) -> bool {
    unsafe {
        ffi::CheckCollisionPointPoly(
            point.into(),
            std::mem::transmute(points.as_ptr()),
            points.len() as i32,
        )
    }
}

/// Check if point belongs to line created between two points [p1] and [p2] with defined margin in pixels [threshold]
#[inline]
#[must_use]
pub fn check_collision_point_line(
    point: impl Into<ffi::Vector2>,
    p1: impl Into<ffi::Vector2>,
    p2: impl Into<ffi::Vector2>,
    threshold: i32,
) -> bool {
    unsafe { ffi::CheckCollisionPointLine(point.into(), p1.into(), p2.into(), threshold) }
}

/// Checks if point is inside a triangle.
#[inline]
#[must_use]
pub fn check_collision_point_triangle(
    point: impl Into<ffi::Vector2>,
    p1: impl Into<ffi::Vector2>,
    p2: impl Into<ffi::Vector2>,
    p3: impl Into<ffi::Vector2>,
) -> bool {
    unsafe { ffi::CheckCollisionPointTriangle(point.into(), p1.into(), p2.into(), p3.into()) }
}

/// Check the collision between two lines defined by two points each, returns collision point by reference
#[inline]
#[must_use]
pub fn check_collision_lines(
    start_pos1: impl Into<ffi::Vector2>,
    end_pos1: impl Into<ffi::Vector2>,
    start_pos2: impl Into<ffi::Vector2>,
    end_pos2: impl Into<ffi::Vector2>,
) -> Option<Vector2> {
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
    if collision {
        return Some(out.into());
    } else {
        return None;
    }
}

/// Detects collision between two spheres.
#[inline]
#[must_use]
pub fn check_collision_spheres(
    center_a: impl Into<ffi::Vector3>,
    radius_a: f32,
    center_b: impl Into<ffi::Vector3>,
    radius_b: f32,
) -> bool {
    unsafe { ffi::CheckCollisionSpheres(center_a.into(), radius_a, center_b.into(), radius_b) }
}

/// Detects collision between ray and sphere.
#[inline]
#[must_use]
pub fn get_ray_collision_sphere(
    ray: impl Into<ffi::Ray>,
    sphere_position: impl Into<ffi::Vector3>,
    sphere_radius: f32,
) -> RayCollision {
    unsafe { ffi::GetRayCollisionSphere(ray.into(), sphere_position.into(), sphere_radius).into() }
}

/// Gets collision info between ray and model.
#[inline]
#[must_use]
pub fn get_ray_collision_model(
    ray: impl Into<ffi::Ray>,
    model: &Mesh,
    transform: &Matrix,
) -> RayCollision {
    unsafe { ffi::GetRayCollisionMesh(ray.into(), model.0, transform.into()).into() }
}

/// Gets collision info between ray and triangle.
#[inline]
#[must_use]
pub fn get_ray_collision_triangle(
    ray: impl Into<ffi::Ray>,
    p1: impl Into<ffi::Vector3>,
    p2: impl Into<ffi::Vector3>,
    p3: impl Into<ffi::Vector3>,
) -> RayCollision {
    unsafe { ffi::GetRayCollisionTriangle(ray.into(), p1.into(), p2.into(), p3.into()).into() }
}

/// Gets collision info between ray and model.
#[inline]
#[must_use]
pub fn get_ray_collision_quad(
    ray: impl Into<ffi::Ray>,
    p1: impl Into<ffi::Vector3>,
    p2: impl Into<ffi::Vector3>,
    p3: impl Into<ffi::Vector3>,
    p4: impl Into<ffi::Vector3>,
) -> RayCollision {
    unsafe {
        ffi::GetRayCollisionQuad(ray.into(), p1.into(), p2.into(), p3.into(), p4.into()).into()
    }
}
