mod vector;
mod geometry;
mod ray;
mod common;


use crate::vector::*;
use crate::geometry::*;
use crate::ray::*;
use crate::common::*;

//use super::*;

use image::{ImageBuffer, Rgb};
use std::vec;
use std::f32::consts::PI;
use std::path::Path;

extern crate rand; 
use rand::Rng;

// lambertian
#[derive(Debug, Copy, Clone)]
pub struct Material {
    albedo: Vec3f,
    emissive: Vec3f,
}

pub struct PhysicalMaterial {
    albedo: Vec3f,
    emissive: Vec3f,
    roughness: f32,
    metallic: f32,
    ao: f32
}

#[derive(Debug, Copy, Clone)]
pub struct Light {
    position: Vec3f,
    color: Vec3f,
}

pub enum RenderStrategy {
    Phong,
    PathTracing,
}

#[derive(Debug, Copy, Clone)]
pub struct Camera {
    position: Vec3f,
    resolution: Vec2f,
}

impl Camera {

    fn new(position: Vec3f, res: (u32, u32)) -> Self {
        Camera {
            position: position,
            resolution: Vec2f::from(res),
        }
    }
    
    fn ray(&self, pixel: (u32, u32)) -> Ray {
        let uv = (Vec2f::from(pixel) - self.resolution * 0.5) / self.resolution.y;        
        let origin = self.position;
        let target = Vec3f::new(uv.x, uv.y, 1.0);
        Ray::new(origin, Vec3f::normalize(target - origin))        
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Object {
    geometry: Sphere,
    material: Material,
}

pub struct Options {
    background: Vec3f,
    resolution: (u32, u32),
}

impl Hittable for Object {
    fn hit(&self, ray: &Ray, min_t: f32, max_t: f32) -> Option<HitRecord> {
        self.geometry.hit(ray, min_t, max_t)
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
        let brdf_multiplier = (self.eval() * cos_theta) /  self.pdf(); 
        (omega, brdf_multiplier)
    }
}

pub fn DistributionGGX(N: Vec3f, H: Vec3f, a: f32) -> f32 {
    let a2     = a*a;
    let NdotH  = f32::max(Vec3f::dot(N, H), 0.0);
    let NdotH2 = NdotH*NdotH;
    let nom    = a2;
    let mut denom  = (NdotH2 * (a2 - 1.0) + 1.0);
    denom        = PI * denom * denom;
    nom / denom
}

pub fn GeometrySchlickGGX(NdotV: f32, roughness: f32) -> f32 {
    let r = (roughness + 1.0);
    let k = (r*r) / 8.0;
    let num   = NdotV;
    let denom = NdotV * (1.0 - k) + k;
    num / denom
}

pub fn GeometrySmith(N: Vec3f, V: Vec3f, L: Vec3f, roughness: f32) -> f32 {
    let NdotV = f32::max(Vec3f::dot(N, V), 0.0);
    let NdotL = f32::max(Vec3f::dot(N, L), 0.0);
    let ggx2  = GeometrySchlickGGX(NdotV, roughness);
    let ggx1  = GeometrySchlickGGX(NdotL, roughness);
    ggx1 * ggx2
}

pub fn fresnelSchlick(cosTheta: f32, F0: Vec3f) -> Vec3f
{
    F0 + (Vec3f::fill(1.0) - F0) * f32::powf((1.0 - cosTheta).clamp(0.0, 1.0), 5.0)
}

// impl BSDF for PhysicalMaterial {}

pub fn phong(hit: &HitRecord, scene: &Vec<Object>, incoming: &Ray) -> Vec3f {
    let light = Light {
        position: Vec3f::new(5.0, 10.0, 5.0),
        color: Vec3f::fill(1.0),
    };
    let lights = vec![light];

    let object = scene[hit.idx];
    let albedo = object.material.albedo;

    let mut result = Vec3f::fill(0.0);

    for light in lights {
        let light_dir = Vec3f::normalize(hit.point - light.position);

        let ambient = light.color * 0.5;

        let diffuse = light.color * f32::max(Vec3f::dot(hit.normal, light_dir), 0.0);

        let reflected = reflect(light_dir, hit.normal);
        let spec = f32::max(Vec3f::dot(incoming.direction, reflected), 0.0).powf(32.0);
        let specular = light.color * spec * 0.5;

        let ray = Ray::new(hit.point, light_dir);

        // check if this point is in shadow
        let in_shadow = match ray.cast(scene) {
            None => 1.0,
            Some(_) => 0.0,
        };

        result = result + (ambient + (diffuse + specular) * in_shadow) * albedo;
    }

    result
}

pub fn visualize_normal(hit: &HitRecord, _scene: &Vec<Object>, _incoming: &Ray) -> Vec3f {
    (Vec3f::fill(1.0) + hit.normal * Vec3f::new(1.0, -1.0, -1.0)) * 0.5
}

pub fn vector_on_sphere() -> Vec3f {
    let r = 1.0;
    let mut rng = rand::thread_rng();
    Vec3f::normalize(
        Vec3f::new(
            rng.gen_range(-r..r),
            rng.gen_range(-r..r),
            rng.gen_range(-r..r)
        )
    )
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

pub fn path_tracing3(hit: &HitRecord, scene: &Vec<Object>, _incoming: &Ray, depth: u32) -> Vec3f {
    let material = scene[hit.idx].material;
    let (omega, prob) = vector_in_hemisphere(hit.normal);
    let ray = Ray::new(hit.point, omega);
    let brdf = material.albedo / PI;
    let cos_theta = Vec3f::dot(hit.normal, omega);
    material.emissive + cast_ray(&ray, scene, depth - 1) * brdf * cos_theta / prob
}

pub fn path_tracing2(hit: &HitRecord, scene: &Vec<Object>, _incoming: &Ray, depth: u32) -> Vec3f {
    let material = scene[hit.idx].material;
    let (omega, brdf_multiplier) = material.sample(hit.normal);
    let ray = Ray::new(hit.point, omega);
    material.emissive + cast_ray(&ray, scene, depth - 1) * brdf_multiplier
}

pub fn cast_ray(ray: &Ray, scene: &Vec<Object>, depth: u32) -> Vec3f {
    let black = Vec3f::fill(0.0);
    let background = Vec3f::new(0.68, 0.87, 0.96); // light blue

    if depth == 0 {
        return black;
    }
    
    let result = match ray.cast(scene) {
        None => {
            background
        }
        Some(hit) => {
            // choose rendering strategy
            //phong(&hit, scene, ray)
            //visualize_normal(&hit, scene, ray)
            path_tracing2(&hit, scene, ray, depth)
        }
    };

    result
}

pub fn reflect(incoming: Vec3f, normal: Vec3f) -> Vec3f {
    incoming - normal * 2.0 * Vec3f::dot(incoming, normal)
}

// https://www.scratchapixel.com/lessons/3d-basic-rendering/introduction-to-shading/reflection-refraction-fresnel.html
pub fn refract(incoming: Vec3f, normal: Vec3f, ior: f32) -> Vec3f {
    let mut cosi = Vec3f::dot(incoming, normal).clamp(-1.0, 1.0);
    let mut etai = 1.0;
    let mut etat = ior;
    let mut n = normal;

    if cosi < 0.0 { 
        cosi = -cosi; 
    } else { 
        let tmp = etai;
        etai = etat;
        etat = tmp;
        n = -normal; 
    }
    
    let eta = etai / etat;
    let k = 1.0 - eta * eta * (1.0 - cosi * cosi);
    
    if k < 0.0 {
        Vec3f::fill(0.0)
    } else {
        incoming * eta + n * (eta * cosi - k.sqrt())
    }
}

pub fn fresnel(incoming: Vec3f, normal: Vec3f, ior: f32) -> f32 {
    // TODO
    0.0
}

pub fn main() {
    const WIDTH: u32 = 640;
    const HEIGHT: u32 = 480;
    const SAMPLES: u32 = 256;
    const BOUNCES: u32 = 3;

    let camera = Camera::new(Vec3f::new(0.0, 0.0, 0.0), (WIDTH, HEIGHT));

    let mut scene: Vec<Object> = Vec::new();

    // right
    scene.push(Object {
        geometry: Sphere {
            center: Vec3f::new(1.5, 0.0, 3.0),
            radius: 0.5,
        },
        material: Material {
            albedo: Vec3f::new(1.0, 0.0, 0.0),
            emissive: Vec3f::fill(0.0),
        },
    });
    // middle
    scene.push(Object {
        geometry: Sphere {
            center: Vec3f::new(0.0, 0.0, 3.0),
            radius: 0.75,
        },
        material: Material {
            albedo: Vec3f::new(0.0, 1.0, 0.),
            emissive: Vec3f::fill(0.0),
        },
    });
    // left
    scene.push(Object {
        geometry: Sphere {
            center: Vec3f::new(-1.5, 0.0, 3.0),
            radius: 0.5,
        },
        material: Material {
            albedo: Vec3f::new(0.0, 0.0, 1.0),
            emissive: Vec3f::fill(0.0),
        },
    });
    
    let r = 100000.0;
    // ground
    scene.push(Object {
        geometry: Sphere {
            center: Vec3f::new(0.0, r + 1.0, 3.0),
            radius: r,
        },
        material: Material {
            albedo: Vec3f::fill(0.18),
            emissive: Vec3f::fill(0.0),
        },
    });
    

    let pixels = vec![0; 3 * WIDTH as usize * HEIGHT as usize];
    let mut buffer = ImageBuffer::from_raw(WIDTH, HEIGHT, pixels).unwrap();

    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            let mut pixel = Vec3f::fill(0.0);
            let ray = camera.ray((x,y));

            for _ in 0..SAMPLES {
                pixel = pixel + cast_ray(&ray, &scene, BOUNCES);
            }

            pixel = (pixel * (u8::MAX as f32)) / (SAMPLES as f32);
            buffer.put_pixel(x, y, Rgb([pixel.x as u8, pixel.y as u8, pixel.z as u8]));
        }
    }

    let filename = format!("render-{}x{}-{}.png", WIDTH, HEIGHT, SAMPLES);
    let path = Path::new(&filename);
    
    match buffer.save(&path) {
        Err(_) => panic!("Could not save file"),
        Ok(_) => println!("Saved output to {:?}", path),
    };
}


#[test]
fn test_sample() {}
