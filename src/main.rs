mod common;
mod geometry;
mod material;
mod ray;
mod vector;
mod camera;

use crate::common::*;
use crate::geometry::*;
use crate::material::*;
use crate::ray::*;
use crate::vector::*;
use crate::camera::*;

use image::{ImageBuffer, Rgb};
use std::f32::consts::PI;
use std::path::Path;
use std::vec;
use std::time::Instant;

/*

pub struct Light {
    position: Vec3f,
    color: Vec3f,
}

pub fn phong(hit: &HitRecord, scene: &Scene, incoming: &Ray) -> Vec3f {
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
*/

pub fn visualize_normal(hit: &HitRecord, _scene: &Scene, _incoming: &Ray) -> Vec3f {
    (Vec3f::fill(1.0) + hit.normal * Vec3f::new(1.0, -1.0, -1.0)) * 0.5
}

/*
// cook torrance
pub fn path_tracing4(hit: &HitRecord, scene: &Scene, _incoming: &Ray, depth: u32) -> Vec3f {
    let material = scene[hit.idx].material;
    let omega = uniform_sample_hemisphere(hit.normal);
    let ray = Ray::new(hit.point, omega);
    material.emissive() + cast_ray(&ray, scene, depth - 1)
}

pub fn path_tracing3(hit: &HitRecord, scene: &Scene, _incoming: &Ray, depth: u32) -> Vec3f {
    let material = scene[hit.idx].material;
    let omega = uniform_sample_hemisphere(hit.normal);
    let prob = 1.0 / (2.0 * PI);
    let ray = Ray::new(hit.point, omega);
    let brdf = material.albedo / PI;
    let cos_theta = Vec3f::dot(hit.normal, omega);
    material.emissive() + cast_ray(&ray, scene, depth - 1) * brdf * cos_theta / prob
}
*/
pub fn path_tracing(hit: &HitRecord, scene: &Scene, _incoming: &Ray, depth: u32) -> Vec3f {
    let material = scene[hit.idx].material;
    let (omega, brdf_multiplier) = material.sample(hit.normal);
    let ray = Ray::new(hit.point, omega);
    material.emissive() + cast_ray(&ray, scene, depth - 1) * brdf_multiplier
}

pub fn cast_ray(ray: &Ray, scene: &Scene, depth: u32) -> Vec3f {
    let background = Vec3f::new(0.68, 0.87, 0.96) * 0.1; // light blue

    if depth == 0 {
        return Vec3f::fill(0.0);
    }

    match ray.cast(scene) {
        None => background,
        Some(hit) => path_tracing(&hit, scene, ray, depth),
    }
}

pub fn main() {
    const WIDTH: u32 = 640;
    const HEIGHT: u32 = 480;
    const SAMPLES: u32 = 64;
    const BOUNCES: u32 = 3;

    let camera = Camera::new(Vec3f::new(0.0, 0.0, 0.0), (WIDTH, HEIGHT));

    let mut scene: Scene = Vec::new();

    // right
    scene.push(Object {
        geometry: Sphere {
            center: Vec3f::new(1.5, 0.0, 4.0),
            radius: 0.5,
        },
        material: Material::Diffuse {
            albedo: Vec3f::new(1.0, 0.0, 0.0),
            emissive: Vec3f::fill(0.0),
        },
    });
    // middle
    scene.push(Object {
        geometry: Sphere {
            center: Vec3f::new(0.0, 0.0, 4.0),
            radius: 0.75,
        },
        material: Material::Diffuse {
            albedo: Vec3f::new(0.0, 1.0, 0.),
            emissive: Vec3f::fill(0.0),
        },
    });
    // left
    scene.push(Object {
        geometry: Sphere {
            center: Vec3f::new(-1.5, 0.0, 5.0),
            radius: 0.5,
        },
        material: Material::Diffuse {
            albedo: Vec3f::new(0.0, 0.0, 1.0),
            emissive: Vec3f::fill(0.0),
        },
    });

    let r = 100000.0;
    let s = 1.0;
    let w = 4.0;
    
    let wall = Material::Diffuse {
        albedo: Vec3f::fill(0.75),
        emissive: Vec3f::fill(0.0),
    };
    
    // ground
    scene.push(Object {
        geometry: Sphere {
            center: Vec3f::new(0.0, r + s, 5.0),
            radius: r,
        },
        material: wall
    });
    scene.push(Object {
        geometry: Sphere {
            center: Vec3f::new(-(r + w), 0.0, 5.0),
            radius: r,
        },
        material: wall
    });
    scene.push(Object {
        geometry: Sphere {
            center: Vec3f::new((r + w), 0.0, 5.0),
            radius: r,
        },
        material: wall
    });
    scene.push(Object {
        geometry: Sphere {
            center: Vec3f::new(0.0, 0.0, 5.0 + (r + w)),
            radius: r,
        },
        material: wall
    });
    scene.push(Object {
        geometry: Sphere {
            center: Vec3f::new(0.0, -(r+w), 5.0),
            radius: r,
        },
        material: Material::Diffuse {
            albedo: Vec3f::fill(0.18),
            emissive: Vec3f::fill(1.0) * 1.0,
        }
    });
    

    let pixels = vec![0; 3 * WIDTH as usize * HEIGHT as usize];
    let mut buffer = ImageBuffer::from_raw(WIDTH, HEIGHT, pixels).unwrap();

    let now = Instant::now();
    
    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            let mut pixel = Vec3f::fill(0.0);
            let ray = camera.ray((x, y));

            for _ in 0..SAMPLES {
                pixel = pixel + cast_ray(&ray, &scene, BOUNCES);
            }

            pixel = (pixel * (u8::MAX as f32)) / (SAMPLES as f32);
            buffer.put_pixel(x, y, Rgb([pixel.x as u8, pixel.y as u8, pixel.z as u8]));
        }
    }

    let elapsed = now.elapsed();
    println!("Elapsed time: {:.2?}", elapsed);

    let filename = format!("render-{}x{}-{}.png", WIDTH, HEIGHT, SAMPLES);
    let path = Path::new(&filename);

    match buffer.save(&path) {
        Err(_) => panic!("Could not save file"),
        Ok(_) => println!("Saved output to {:?}", path),
    };
}

#[test]
fn test_sample() {}
