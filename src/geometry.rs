use crate::common::*;
use crate::ray::*;
use crate::vector::*;

#[derive(Debug, Copy, Clone)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
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
            return Some(HitRecord {
                t,
                normal,
                point,
                idx,
            });
        } else {
            return None;
        }
    }
}

impl Sphere {
    #[allow(dead_code)]
    pub fn new(center: Vec3, radius: f32) -> Self {
        Sphere { center, radius }
    }
}
