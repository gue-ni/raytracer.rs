extern crate rand;
use rand::Rng;

use std::f32::consts::PI;

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
        let v1v2 = self.2 - self.1;
        let v2v0 = self.0 - self.2;

        let normal = Vec3f::normalize(Vec3f::cross(v0v1, v0v2));
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

        let vp0 = point - self.0;
        c = Vec3f::cross(v0v1, vp0);
        if Vec3f::dot(normal, c) < 0.0 {
            return None;
        }

        let vp1 = point - self.1;
        c = Vec3f::cross(v1v2, vp1);
        if Vec3f::dot(normal, c) < 0.0 {
            return None;
        }

        let vp2 = point - self.2;
        c = Vec3f::cross(v2v0, vp2);
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

// lambertian
#[derive(Debug, Copy, Clone)]
pub struct Material {
    pub albedo: Vec3f,
    pub emissive: Vec3f,
}

pub struct PhysicalMaterial {
    albedo: Vec3f,
    emissive: Vec3f,
    roughness: f32,
    metallic: f32,
    ao: f32,
}

#[derive(Debug, Copy, Clone)]
pub struct Object {
    pub geometry: Sphere,
    pub material: Material,
}

impl Hittable for Object {
    fn hit(&self, ray: &Ray, min_t: f32, max_t: f32) -> Option<HitRecord> {
        self.geometry.hit(ray, min_t, max_t)
    }
}

fn vector_on_sphere() -> Vec3f {
    let r = 1.0;
    let mut rng = rand::thread_rng();
    Vec3f::normalize(Vec3f::new(
        rng.gen_range(-r..r),
        rng.gen_range(-r..r),
        rng.gen_range(-r..r),
    ))
}

/*
pub fn sample_hemisphere() -> Vec3f {
    let mut rng = rand::thread_rng();
    let x1 = rng.get_range(0.0..1.0);
    let x2 = rng.get_range(0.0..1.0);
    let phi = 2.0 * PI * x2;
    let cos_theta = x1;
    let sin_theta = f32::sqrt(1.0 - (cos_theta * cos_theta));
    let cos_phi = f32::cos(phi);
    let sin_phi = f32::sin(phi);
    Vec3f::new(cos_phi * sin_theta, sin_phi * sin_theta, cos_theta)
}
*/

pub fn vector_in_hemisphere(normal: Vec3f) -> (Vec3f, f32) {
    let mut vec: Vec3f;
    loop {
        vec = vector_on_sphere();
        if Vec3f::dot(vec, normal) > 0.0 {
            break;
        }
    }
    let prob = 1.0 / (2.0 * PI);
    (vec, prob)
}

pub fn uniform_sample_hemisphere(normal: Vec3f) -> Vec3f {
    loop {
        let omega = vector_on_sphere();
        if Vec3f::dot(omega, normal) > 0.0 {
            break omega;
        }
    }
}

// Bidirectional Scattering Distribution Function (BSDF)
pub trait BSDF {
    fn pdf(&self) -> f32;
    fn eval(&self) -> Vec3f;
    fn sample(&self, normal: Vec3f) -> (Vec3f, Vec3f);
}

impl BSDF for Material {
    fn pdf(&self) -> f32 {
        1.0 / (2.0 * PI)
    }

    fn eval(&self) -> Vec3f {
        self.albedo / PI
    }

    fn sample(&self, normal: Vec3f) -> (Vec3f, Vec3f) {
        let omega = uniform_sample_hemisphere(normal);
        let cos_theta = Vec3f::dot(normal, omega);
        let brdf_multiplier = (self.eval() * cos_theta) / self.pdf();
        (omega, brdf_multiplier)
    }
}

pub type Scene = Vec<Object>;

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

    #[test]
    fn test_mesh_hit() {
        let s = 0.5;
        let quad = Mesh {
            triangles: vec![
                Triangle(
                    Vec3f::new(-s, -s, -s), // bottom left
                    Vec3f::new(-s, s, -s),  // top left
                    Vec3f::new(s, -s, -s),  // bottom right
                ),
                Triangle(
                    Vec3f::new(-s, s, -s), // top left
                    Vec3f::new(s, s, -s),  // top right
                    Vec3f::new(s, -s, -s), // bottom right
                ),
            ],
        };

        let ray = Ray::new(Vec3f::new(0.0, 0.0, -5.0), Vec3f::new(0.0, 0.0, 1.0));

        let possible_hit = quad.hit(&ray, 0.0, f32::INFINITY);
        assert!(possible_hit.is_some());

        let hit = possible_hit.unwrap();
        assert_eq!(hit.point, Vec3f::new(0.0, 0.0, -s));
        assert_eq!(hit.normal, Vec3f::new(0.0, 0.0, -1.0));
    }
}
