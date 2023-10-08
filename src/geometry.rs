use crate::material::*;
use crate::ray::Ray;
use crate::vector::*;

use serde::{Deserialize, Deserializer, Serialize};
use std::f64::consts::PI;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
pub struct Hit {
    pub t: f64,
    pub normal: Vec3f,
    pub point: Vec3f,
    pub idx: usize,
}

impl Default for Hit {
    fn default() -> Self {
        Hit {
            t: f64::INFINITY,
            normal: Vec3f::from(0.0),
            point: Vec3f::from(0.0),
            idx: 0,
        }
    }
}

impl Hit {
    pub fn new(t: f64, normal: Vec3f, point: Vec3f, idx: usize) -> Self {
        Self {
            t,
            normal,
            point,
            idx,
        }
    }

    pub fn get_point(&self) -> Vec3f {
        self.point + self.normal * 0.001
    }
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct Sphere {
    pub center: Vec3f,
    pub radius: f64,
}

impl Sphere {
    #[allow(dead_code)]
    pub fn new(center: Vec3f, radius: f64) -> Self {
        Sphere { center, radius }
    }

    pub fn surface_area(&self) -> f64 {
        4.0 * PI * self.radius * self.radius
    }
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct Triangle(pub Vec3f, pub Vec3f, pub Vec3f);

/// Convex polygon mesh
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mesh {
    triangles: Vec<Triangle>,
}

impl Mesh {
    pub fn new() -> Self {
        Self { triangles: vec![] }
    }

    fn parse_indices(tokens: &[&str]) -> (usize, usize, usize) {
        assert!(tokens.len() == 3);
        let mut tmp: Vec<usize> = Vec::new();
        for token in tokens {
            let indices: Vec<&str> = token.split('/').collect();
            assert!(indices.len() == 3);

            let vertex_index = indices[0].parse::<usize>().unwrap();
            tmp.push(vertex_index - 1);
        }

        (tmp[0], tmp[1], tmp[2])
    }

    fn parse_vertex(tokens: &[&str]) -> Vec3f {
        assert!(tokens.len() == 3);
        let x = tokens[0].parse::<f64>().unwrap();
        let y = tokens[1].parse::<f64>().unwrap();
        let z = tokens[2].parse::<f64>().unwrap();
        Vec3::new(x, y, z)
    }

    pub fn from_obj(path: &str) -> io::Result<Mesh> {
        let mut mesh = Mesh::new();

        let mut vertices: Vec<Vec3f> = Vec::new();

        let file = File::open(path)?;
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let line = line?;

            if line.starts_with('#') || line.is_empty() {
                continue;
            }

            let tokens: Vec<&str> = line.split_whitespace().collect();

            match tokens[0] {
                "v" => {
                    let vertex_tokens = &tokens[1..];
                    vertices.push(Self::parse_vertex(vertex_tokens));
                }
                "f" => {
                    let index_tokens = &tokens[1..];
                    let (v0, v1, v2) = Self::parse_indices(index_tokens);
                    mesh.triangles
                        .push(Triangle(vertices[v0], vertices[v1], vertices[v2]));
                }
                _ => {
                    println!("Skip line {:?}", tokens[0]);
                }
            }
        }

        Ok(mesh)
    }
}

fn load_from_obj<'de, D>(deserializer: D) -> Result<Mesh, D::Error>
where
    D: Deserializer<'de>,
{
    let obj_path: &str = Deserialize::deserialize(deserializer)?;
    Ok(Mesh::from_obj(obj_path))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Geometry {
    #[serde(deserialize_with = "load_from_obj")]
    MESH(Mesh),
    SPHERE(Sphere),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Object {
    pub geometry: Geometry,
    pub material: Material,
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct Light {
    pub geometry: Sphere,
    pub emission: Vec3f,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Scene {
    pub background: Vec3f,
    pub lights: Vec<Light>,
    pub objects: Vec<Object>,
}

impl Scene {
    pub fn new(background: Vec3f) -> Self {
        Self {
            background,
            objects: Vec::new(),
            lights: Vec::new(),
        }
    }

    pub fn add(&mut self, object: Object) {
        self.objects.push(object);
    }
}

pub trait Hittable {
    /// Returns Hit if ray intersects geometry
    fn hit(&self, ray: &Ray, min_t: f64, max_t: f64) -> Option<Hit>;
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, min_t: f64, max_t: f64) -> Option<Hit> {
        let m = ray.origin - self.center;
        let b = Vec3::dot(m, ray.direction);
        let c = Vec3::dot(m, m) - self.radius * self.radius;

        if 0.0 < c && 0.0 < b {
            return None;
        }

        let discr = b * b - c;
        if discr < 0.0 {
            return None;
        }

        let mut t = -b - f64::sqrt(discr);

        if t < 0.0 {
            t = -b + f64::sqrt(discr);
        }

        if min_t < t && t < max_t {
            let point = ray.point_at(t);
            Some(Hit::new(t, (point - self.center) / self.radius, point, 0))
        } else {
            None
        }
    }
}

impl Hittable for Triangle {
    fn hit(&self, ray: &Ray, min_t: f64, max_t: f64) -> Option<Hit> {
        let v0v1 = self.1 - self.0;
        let v0v2 = self.2 - self.0;
        let v1v2 = self.2 - self.1;
        let v2v0 = self.0 - self.2;

        let normal = Vec3f::normalize(Vec3f::cross(v0v1, v0v2));
        let ndot = Vec3f::dot(normal, ray.direction);

        if f64::abs(ndot) < f64::EPSILON {
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

        Some(Hit::new(t, normal, point, 0))
    }
}

impl Hittable for Mesh {
    fn hit(&self, ray: &Ray, min_t: f64, max_t: f64) -> Option<Hit> {
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
    fn hit(&self, ray: &Ray, min_t: f64, max_t: f64) -> Option<Hit> {
        match self {
            Geometry::MESH(g) => g.hit(ray, min_t, max_t),
            Geometry::SPHERE(g) => g.hit(ray, min_t, max_t),
        }
    }
}

impl Hittable for Scene {
    fn hit(&self, ray: &Ray, min_t: f64, max_t: f64) -> Option<Hit> {
        let mut closest = Hit::default();
        closest.t = max_t;

        for (i, object) in self.objects.iter().enumerate() {
            if let Some(hit) = object.geometry.hit(ray, min_t, closest.t) {
                closest = hit;
                closest.idx = i;
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
    use crate::common::*;
    use crate::geometry::*;
    use crate::ray::*;

    #[test]
    fn test_sphere_hit() {
        let sphere = Sphere::new(Vec3f::new(0.0, 0.0, 5.0), 1.0);
        let ray = Ray::new(Vec3f::new(0.0, 0.0, 0.0), Vec3f::new(0.0, 0.0, 1.0));
        if let Some(hit) = sphere.hit(&ray, 0.0, f64::INFINITY) {
            assert_eq!(hit.t, 4.0);
            assert_eq!(hit.point, Vec3f::new(0.0, 0.0, 4.0));
            assert_eq!(hit.normal, Vec3f::new(0.0, 0.0, -1.0));
        } else {
            assert!(false);
        }
    }

    #[test]
    fn test_sphere_hit_inside() {
        let sphere = Sphere::new(Vec3f::from(0.0), 3.0);
        let ray = Ray::towards(sphere.center, Vec3::new(0.0, 0.0, 1.0));
        if let Some(hit) = sphere.hit(&ray, 0.0, f64::INFINITY) {
            assert_eq!(hit.t, 3.0);
            assert_eq!(hit.point, Vec3f::new(0.0, 0.0, 3.0));
            // what should the normal be in this case?
            assert_eq!(hit.normal, Vec3f::new(0.0, 0.0, 1.0));
        } else {
            assert!(false);
        }
    }

    #[test]
    fn test_triangle_hit() {
        let triangle = Triangle(
            Vec3f::new(-0.5, 0.0, 5.0),
            Vec3f::new(0.0, 1.0, 5.0),
            Vec3f::new(0.5, 0.0, 5.0),
        );
        let ray = Ray::new(Vec3f::new(0.0, 0.5, 0.0), Vec3f::new(0.0, 0.0, 1.0));
        let hit = triangle.hit(&ray, 0.0, f64::INFINITY).unwrap();
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

        let possible_hit = quad.hit(&ray, 0.0, f64::INFINITY);
        assert!(possible_hit.is_some());

        let hit = possible_hit.unwrap();
        assert_eq!(hit.point, Vec3f::new(0.0, 0.0, -s));
        assert_eq!(hit.normal, Vec3f::new(0.0, 0.0, -1.0));
    }

    #[test]
    fn test_misc() {
        let sphere = Sphere::new(Vec3f::new(0.0, 0.0, 5.0), 1.0);
        let ray_1 = Ray::towards(Vec3::new(0.0, 0.0, 0.0), sphere.center);

        let hit_1 = sphere.hit(&ray_1, 0.0, f64::INFINITY).unwrap();

        println!("{:?}", sphere);
        println!("{:?}", ray_1);
        println!("{:?}", hit_1);

        let incoming = ray_1.direction;

        let ior = 1.2;
        let reflected = reflect(incoming, hit_1.normal);
        let refracted = refract(incoming, hit_1.normal, ior);

        println!("reflected = {:?}", reflected);
        println!("refracted = {:?}", refracted);

        let ray_2 = Ray::new(hit_1.point + hit_1.normal * -0.001, refracted);
        println!("{:?}", ray_2);

        let hit_2 = sphere.hit(&ray_2, 0.0, f64::INFINITY).unwrap();

        println!("hit_2 = {:?}", hit_2);
    }

    #[test]
    fn test_deserialize() {
        {
            let json = r#"{ "radius": 1.0, "center": [0.5, -1.0, 0.0] }"#;
            let sphere: Sphere = serde_json::from_str(json).unwrap();
            assert_eq!(sphere.radius, 1.0);
            assert_eq!(sphere.center, Vec3::new(0.5, -1.0, 0.0));
        }
        {
            let json = r#"{
                "geometry": {
                    "radius": 1.0,
                    "center": [0.0, 0.0, 0.0]
                },
                "material": {
                    "albedo": [1.0, 0.0, 0.0],
                    "emittance": 1.0,
                    "roughness": 1.0,
                    "ior": 1.0,
                    "metallic": 1.0,
                    "material": "Lambert"
                }
            }"#;
            let _object: Object = serde_json::from_str(json).unwrap();
            //println!("{:?}", object);
        }
        {
            let json = r#"{
                "background": [0.5, 0.0, 1.0],
                "objects": [
                    {
                        "geometry": {
                            "radius": 1.0,
                            "center": [0.0, 0.0, 7.0]
                        },
                        "material": {
                            "albedo": [1.0, 0.0, 0.0],
                            "emittance": 1.0,
                            "roughness": 1.0,
                            "ior": 1.0,
                            "metallic": 1.0,
                            "material": "Lambert"
                        }
                    }
                ],
                "lights": [
                    {
                        "geometry": {
                            "radius": 0.5,
                            "center": [0.0, 4.0, 0.0]
                        },
                        "emission": [10,10,5]
                    }
                ]
            }"#;
            let _scene: Scene = serde_json::from_str(json).unwrap();
        }
    }

    #[test]
    fn test_deserialize_2() {
        let json = r#"{
                "background": [0.5, 0.0, 1.0],
                "objects": [
                    {
                        "geometry": "scenes/cube.obj",
                        "material": {
                            "albedo": [1.0, 0.0, 0.0],
                            "emittance": 1.0,
                            "roughness": 1.0,
                            "ior": 1.0,
                            "metallic": 1.0,
                            "material": "Lambert"
                        }
                    }
                ],
                "lights": [
                    {
                        "geometry": {
                            "radius": 0.5,
                            "center": [0.0, 4.0, 0.0]
                        },
                        "emission": [10,10,5]
                    }
                ]
            }"#;
        let _scene: Scene = serde_json::from_str(json).unwrap();
        println!("{:?}", _scene);
    }

    #[test]
    fn test_load_obj() {
        if let Ok(mesh) = Mesh::from_obj("scenes/cube.obj") {
            println!("{:?}", mesh);
            assert_eq!(mesh.triangles.len(), 6 * 2);
        }
    }
}
