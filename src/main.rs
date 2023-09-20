mod vector;
use crate::vector::*;

mod geometry;
use crate::geometry::*;

mod ray;
use crate::ray::*;

mod common;
use crate::common::*;

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
pub struct Object {
    geometry: Sphere,
    material: Material,
}

pub struct Options {
    background: Vec3f,
    width: u32,
    height: u32
}

impl Hittable for Object {
    fn hit(&self, ray: &Ray, min_t: f32, max_t: f32) -> Option<HitRecord> {
        self.geometry.hit(ray, min_t, max_t)
    }
}

pub fn camera_ray(x: u32, y: u32, x_res: u32, y_res: u32) -> Ray {
    let mut ray_target = Vec3f::new(
        ((x as f32) / (x_res as f32)) * 2.0 - 1.0,
        ((y as f32) / (y_res as f32)) * 2.0 - 1.0,
        1.0,
    );

    let aspect_ratio = (x_res as f32) / (y_res as f32);
    ray_target.y /= aspect_ratio;

    let origin = Vec3f::new(0.0, 0.0, 0.0);
    let direction = Vec3f::normalize(ray_target - origin);

    Ray::new(origin, direction)
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

pub fn path_tracing3(hit: &HitRecord, scene: &Vec<Object>, _incoming: &Ray, depth: u32) -> Vec3f {
    let material = scene[hit.idx].material;
    let albedo = material.albedo;
    let emissive = material.emissive;

    let (omega, prob) = vector_in_hemisphere(hit.normal);
    let ray = Ray::new(hit.point, omega);
    let brdf = albedo / PI;
    let cos_theta = Vec3f::dot(hit.normal, omega);
    emissive + brdf * cast_ray(&ray, scene, depth - 1) * cos_theta / prob
}

pub fn path_tracing2(hit: &HitRecord, scene: &Vec<Object>, _incoming: &Ray, depth: u32) -> Vec3f {
    let material = scene[hit.idx].material;
    let albedo = material.albedo;
    let emissive = material.emissive;

    let (random_vector, p) = vector_in_hemisphere(hit.normal);
    let new_ray = Ray::new(hit.point, random_vector);
    
    let cos_theta = Vec3f::dot(new_ray.direction, hit.normal);
    let brdf = material.albedo / PI;

    let light = cast_ray(&new_ray, scene, depth - 1);

    emissive + (brdf * light * cos_theta / p)
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
    let background = Vec3f::fill(0.0);

    if depth == 0 {
        return background;
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
    const SAMPLES: u32 = 32;
    const BOUNCES: u32 = 3;

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
            radius: 0.5,
        },
        material: Material {
            albedo: Vec3f::new(1.0, 0.0, 0.),
            emissive: Vec3f::fill(1.0),
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

    /*
    let r = 100000.0;
    scene.push(Object {
        geometry: Sphere {
            center: Vec3f::new(0.0, r + 1.0, 3.0),
            radius: r,
        },
        material: Material {
            albedo: Vec3f::new(0.0, 0.6, 1.0),
            emissive: Vec3f::fill(0.0),
        },
    });
    */

    let pixels = vec![0; 3 * WIDTH as usize * HEIGHT as usize];
    let mut buffer = ImageBuffer::from_raw(WIDTH, HEIGHT, pixels).unwrap();

    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            let mut pixel = Vec3f::new(0.0, 0.0, 0.0);
            let ray = camera_ray(x, y, WIDTH, HEIGHT);

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
