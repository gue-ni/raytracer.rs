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

extern crate rand; 
use rand::Rng;

#[derive(Debug, Copy, Clone)]
pub struct Material {
    albedo: Vec3f,
    emissive: Vec3f,
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
            resolution: Vec2f::new(res.0 as f32, res.1 as f32),
        }
    }
    
    fn ray(&self, pixel: (u32, u32)) -> Ray {
        // vec2 uv = (fragCoord.xy - 0.5 * iResolution.xy) / iResolution.y;

        let coord = Vec2f::new(pixel.0 as f32, pixel.1 as f32);
        let uv = (coord - self.resolution * 0.5) / self.resolution.y;
        
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
    let cos_theta = x1;
    let sin_theta = f32::sqrt(1.0 - (cos_theta * cos_theta));
    let cos_phi = f32::cos(2.0 * PI * x2);
    let sin_phi = f32::sin(2.0 * PI * x2);
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
    let albedo = material.albedo;
    let emissive = material.emissive;

    let (omega, prob) = vector_in_hemisphere(hit.normal);
    let ray = Ray::new(hit.point, omega);
    let brdf = albedo / PI;
    let cos_theta = Vec3f::dot(hit.normal, omega);
    emissive + cast_ray(&ray, scene, depth - 1) * brdf * cos_theta / prob
}

pub fn path_tracing1(hit: &HitRecord, scene: &Vec<Object>, _incoming: &Ray) -> Vec3f {   
    let mut direct_light = Vec3f::fill(0.0);
    let mut indirect_light = Vec3f::fill(0.0);

    // direct light contribution
    for (idx, object) in scene.iter().enumerate() {
        
        // object is not illuminated by itself
        if idx == hit.idx {
            continue;
        }
        
        let light_dir = Vec3f::normalize(hit.point - object.geometry.center);

        let shadow_ray = Ray::new(hit.point, light_dir);
        
        let visible = match shadow_ray.cast(scene) {
            None => 1.0,
            Some(_) => 0.0
        };

        let emissive = object.material.emissive;
        let diffuse = f32::max(Vec3f::dot(hit.normal, light_dir), 0.0);
        
        direct_light = direct_light + (emissive);
    }

    let albedo = scene[hit.idx].material.albedo;
    
    (direct_light + indirect_light) * albedo / 3.14
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
            path_tracing3(&hit, scene, ray, depth)
        }
    };

    result
}

pub fn reflect(incoming: Vec3f, normal: Vec3f) -> Vec3f {
    incoming - normal * 2.0 * Vec3f::dot(incoming, normal)
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
            albedo: Vec3f::new(0.0, 1.0, 0.0),
            emissive: Vec3f::fill(0.0),
        },
    });
    // middle
    scene.push(Object {
        geometry: Sphere {
            center: Vec3f::new(0.0, 0.0, 3.0),
            radius: 1.0,
        },
        material: Material {
            albedo: Vec3f::new(1.0, 0.0, 0.),
            emissive: Vec3f::fill(0.0),
        },
    });
    // left
    scene.push(Object {
        geometry: Sphere {
            center: Vec3f::new(-1.5, 0.0, 3.0),
            radius: 0.75,
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
            let mut pixel = Vec3f::new(0.0, 0.0, 0.0);
            let ray = camera.ray((x,y));

            for _ in 0..SAMPLES {
                pixel = pixel + cast_ray(&ray, &scene, BOUNCES);
            }

            pixel = (pixel * (u8::MAX as f32)) / (SAMPLES as f32);
            buffer.put_pixel(x, y, Rgb([pixel.x as u8, pixel.y as u8, pixel.z as u8]));
        }
    }

    let filename = "output.png";
    match buffer.save(filename) {
        Err(_) => panic!("Could not save file"),
        Ok(_) => println!("Saved output to {}", filename),
    };
}


#[test]
fn test_sample() {}
