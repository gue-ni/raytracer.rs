use crate::material::*;
use crate::ray::Ray;
use crate::vector::*;

pub struct HitRecord {
    pub t: f32,
    pub normal: Vec3f,
    pub point: Vec3f,
    pub idx: usize,
}

impl Default for HitRecord {
    fn default() -> Self {
        HitRecord {
            t: f32::INFINITY,
            normal: Vec3f::fill(0.0),
            point: Vec3f::fill(0.0),
            idx: 0,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Sphere {
    pub center: Vec3f,
    pub radius: f32,
}

impl Sphere {
    #[allow(dead_code)]
    pub fn new(center: Vec3f, radius: f32) -> Self {
        Sphere { center, radius }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Triangle(pub Vec3f, pub Vec3f, pub Vec3f);

/// Convex polygon mesh
#[derive(Debug, Clone)]
pub struct Mesh {
    triangles: Vec<Triangle>,
}

#[derive(Debug, Clone)]
pub enum Geometry {
    MESH(Mesh),
    SPHERE(Sphere),
}

#[derive(Debug, Clone)]
pub struct Object {
    pub geometry: Sphere,
    pub material: Material,
}

pub struct Scene {
    pub objects: Vec<Object>,
    pub background: Vec3f,
}

impl Scene {
    pub fn new(background: Vec3f) -> Self {
        Self {
            objects: Vec::new(),
            background,
        }
    }

    pub fn add(&mut self, object: Object) {
        self.objects.push(object);
    }
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
            let normal = (point - self.center) / self.radius;
            let idx = 0;
            Some(HitRecord {
                t,
                normal,
                point,
                idx,
            })
        } else {
            None
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
            t,
            normal,
            point,
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

impl Hittable for Geometry {
    fn hit(&self, ray: &Ray, min_t: f32, max_t: f32) -> Option<HitRecord> {
        match self {
            Geometry::MESH(g) => g.hit(ray, min_t, max_t),
            Geometry::SPHERE(g) => g.hit(ray, min_t, max_t),
        }
    }
}

impl Hittable for Scene {
    fn hit(&self, ray: &Ray, min_t: f32, max_t: f32) -> Option<HitRecord> {
        let mut closest = HitRecord::default();
        closest.t = max_t;

        for (i, object) in self.objects.iter().enumerate() {
            match object.geometry.hit(ray, min_t, closest.t) {
                None => {}
                Some(hit) => {
                    closest = hit;
                    closest.idx = i;
                }
            }
        }

        if closest.t < max_t {
            Some(closest)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use crate::geometry::*;
    use crate::ray::*;

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
