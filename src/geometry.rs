use crate::common::*;
use crate::ray::Ray;
use crate::vector::*;

#[derive(Debug, Copy, Clone)]
pub struct Sphere {
    pub center: Vec3f,
    pub radius: f32,
}

#[derive(Debug, Copy, Clone)]
pub struct Triangle(pub Vec3f, pub Vec3f, pub Vec3f);

// convex polygon
#[derive(Debug, Clone)]
pub struct Mesh {
    triangles: Vec<Triangle>,
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, min_t: f32, max_t: f32) -> Option<HitRecord>;
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, min_t: f32, max_t: f32) -> Option<HitRecord> {
        let m = ray.origin - self.center;
        let b = Vec3f::dot(m, ray.direction);
        let c = Vec3f::dot(m, m) - self.radius * self.radius;

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
            let normal = Vec3f::normalize(point - self.center);
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

impl Hittable for Triangle {
    fn hit(&self, ray: &Ray, min_t: f32, max_t: f32) -> Option<HitRecord> {
        let v0v1 = self.1 - self.0;
        let v0v2 = self.2 - self.0;
        let normal = Vec3f::cross(v0v1, v0v2);

        let ndot = Vec3f::dot(normal, ray.direction);
        if f32::abs(ndot) < f32::EPSILON {
            return None;
        }

        let d = -Vec3f::dot(normal, self.0);

        let t = -(Vec3f::dot(normal, ray.origin) + d) / ndot;
        if t < min_t && t > max_t {
            return None;
        }

        let point = ray.point_at(t);

        let mut c: Vec3f;

        let edge0 = self.1 - self.0;
        let vp0 = point - self.0;
        c = Vec3f::cross(edge0, vp0);
        if Vec3f::dot(normal, c) < 0.0 {
            return None;
        }

        let edge1 = self.2 - self.1;
        let vp1 = point - self.1;
        c = Vec3f::cross(edge1, vp1);
        if Vec3f::dot(normal, c) < 0.0 {
            return None;
        }

        let edge2 = self.0 - self.2;
        let vp2 = point - self.2;
        c = Vec3f::cross(edge2, vp2);
        if Vec3f::dot(normal, c) < 0.0 {
            return None;
        }

        Some(HitRecord {
            t: t,
            normal: normal,
            point: point,
            idx: 0,
        })
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
    pub fn new(center: Vec3f, radius: f32) -> Self {
        Sphere { center, radius }
    }
}

#[cfg(test)]
mod test {
    use crate::geometry::*;
    use crate::ray::*;
    use crate::vector::*;

    #[test]
    fn test_sphere_hit() {
        let sphere = Sphere::new(Vec3f::new(0.0, 0.0, 5.0), 1.0);
        let ray = Ray::new(Vec3f::new(0.0, 0.0, 0.0), Vec3f::new(0.0, 0.0, 1.0));
        let hit = sphere.hit(&ray, 0.0, f32::INFINITY).unwrap();
        assert_eq!(hit.t, 4.0);
        assert_eq!(hit.point, Vec3f::new(0.0, 0.0, 4.0));
        assert_eq!(hit.normal, Vec3f::new(0.0, 0.0, -1.0));
    }

    #[test]
    fn test_triangle_hit() {
        let triangle = Triangle(
            Vec3f::new(-0.5, 0.0, 5.0),
            Vec3f::new(0.0, 1.0, 5.0),
            Vec3f::new(0.5, 0.0, 5.0),
        );
        let ray = Ray::new(Vec3f::new(0.0, 0.5, 0.0), Vec3f::new(0.0, 0.0, 1.0));
        let hit = triangle.hit(&ray, 0.0, f32::INFINITY).unwrap();
        assert_eq!(hit.t, 5.0);
        assert_eq!(hit.point, Vec3f::new(0.0, 0.5, 5.0));
        assert_eq!(hit.normal, Vec3f::new(0.0, 0.0, -1.0));
    }
}
