use crate::vector::*;
use crate::common::*;
use crate::ray::*;
use std::vec;

#[derive(Debug, Copy, Clone)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
}

#[derive(Debug, Copy, Clone)]
pub struct Triangle(Vec3, Vec3, Vec3);

// convex polygon
#[derive(Debug, Clone)]
pub struct Mesh {
    triangles: Vec<Triangle>
}

// signed distance function
pub trait SDF {
    fn sdf(&self, point: Vec3) -> f32;
}

impl SDF for Sphere {
    fn sdf(&self, point: Vec3) -> f32 {
        (point - self.center).length() - self.radius
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, min_t: f32, max_t: f32) -> Option<HitRecord>;
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, min_t: f32, max_t: f32) -> Option<HitRecord> {
        let m = ray.origin - self.center;
        let b = dot(m, ray.direction);
        let c = dot(m, m) - self.radius * self.radius;

        if c > 0.0 && b > 0.0 {
            return None;
        }

        let discr = b * b - c;
        if discr < 0.0 {
            return None;
        }

        let t = -b - discr.sqrt();

        if min_t < t && t < max_t {
            let point = ray.point_at(t);
            let normal = normalize(point - self.center);
            let idx = 0;
            return Some(HitRecord { t, normal, point, idx });
        } else {
            return None;
        }
    }
}

impl Hittable for Triangle {
    fn hit(&self, ray: &Ray, min_t: f32, max_t: f32) -> Option<HitRecord> {

        let v0v1 = self.1 - self.0;
        let v0v2 = self.2 - self.0;
        let n = cross(v0v1, v0v2);

        let ndot = dot(n, ray.direction);
        if f32::abs(ndot) < f32::EPSILON {
            return None;
        }

        let d = -dot(n, self.0);

        let t = -(dot(n, ray.origin) + d) / ndot;
        if t < 0.0 {
            return None;
        }

        let point = ray.point_at(t);

        let mut c = Vec3::zero();

        let edge0 = self.1 - self.0;
        let vp0 = point - self.0;
        c = cross(edge0, vp0);
        if dot(n,c) < 0.0 {
            return None;
        }

        let edge1 = self.2 - self.1;
        let vp1 = point - self.1;
        c = cross(edge1, vp1);
        if dot(n,c) < 0.0 {
            return None;
        }

        let edge2 = self.0 - self.2;
        let vp2 = point - self.2;
        c = cross(edge2, vp2);
        if dot(n,c) < 0.0 {
            return None;
        }
        
        Some(HitRecord { t: t, normal: n, point: point, idx: 0 })
    }
}

impl Hittable for Mesh {
    fn hit(&self, ray: &Ray, min_t: f32, max_t: f32) -> Option<HitRecord> {
        for triangle in &self.triangles {
            let hit_record = triangle.hit(ray, min_t, max_t);
            if hit_record.is_some() {
                return hit_record;
            }
        }
        None
    }
}

impl Sphere {
    #[allow(dead_code)]
    pub fn new(center: Vec3, radius: f32) -> Self {
        Sphere { center, radius }
    }
}
