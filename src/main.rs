mod vector;
use crate::vector::{dot, normalize, reflect, Vec3};

mod geometry;
use crate::geometry::*;

mod ray;
use crate::ray::*;

mod common;
use crate::common::*;

mod test;

use image::{ImageBuffer, Rgb};
use std::vec;

#[derive(Debug, Copy, Clone)]
pub struct Material {
    albedo: Vec3,
    emissive: Vec3,
}

#[derive(Debug, Copy, Clone)]
pub struct Light {
    position: Vec3,
    color: Vec3,
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

impl Hittable for Object {
    fn hit(&self, ray: &Ray, min_t: f32, max_t: f32) -> Option<HitRecord> {
        self.geometry.hit(ray, min_t, max_t)
    }
}

pub fn camera_ray(x: u32, y: u32, x_res: u32, y_res: u32) -> Ray {
    let mut ray_target = Vec3::new(
        ((x as f32) / (x_res as f32)) * 2.0 - 1.0,
        ((y as f32) / (y_res as f32)) * 2.0 - 1.0,
        1.0,
    );

    let aspect_ratio = (x_res as f32) / (y_res as f32);
    ray_target.y /= aspect_ratio;

    let origin = Vec3::new(0.0, 0.0, 0.0);
    let direction = normalize(ray_target - origin);

    Ray::new(origin, direction)
}

pub fn phong(hit: &HitRecord, scene: &Vec<Object>, incoming: &Ray) -> Vec3 {
    let light = Light {
        position: Vec3::new(5.0, 10.0, 5.0),
        color: Vec3::one(),
    };
    let lights = vec![light];

    //let albedo = hit.normal * 0.5 + 0.5;
    let object = scene[hit.idx];
    let albedo = object.material.albedo;

    let mut result = Vec3::zero();

    for light in lights {
        let light_dir = normalize(hit.point - light.position);

        let ambient = light.color * 0.5;

        let diffuse = light.color * f32::max(dot(hit.normal, light_dir), 0.0);

        let reflected = reflect(light_dir, hit.normal);
        let spec = f32::max(dot(incoming.direction, reflected), 0.0).powf(32.0);
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

pub fn visualize_normal(hit: &HitRecord, _scene: &Vec<Object>, _incoming: &Ray) -> Vec3 {
    (Vec3::one() + hit.normal * Vec3::new(1.0, -1.0, -1.0)) * 0.5
}

pub fn path_tracing(hit: &HitRecord, scene: &Vec<Object>, incoming: &Ray) -> Vec3 {
    //let reflected = reflect(incoming.direction, hit.normal);
    //let ray = Ray::new(hit.point, reflected);

    let mut result = Vec3::zero();

    // direct light
    for object in scene {
        let light_dir = normalize(hit.point - object.geometry.center);

        let ray = Ray::new(hit.point, light_dir);
        let in_shadow = match ray.cast(scene) {
            None => 1.0,
            Some(_) => 0.0,
        };
        let l = f32::max(dot(hit.normal, light_dir), 0.0);

        result = result + (object.material.emissive * l * in_shadow);
    }

    result
}

pub fn cast_ray(ray: &Ray, scene: &Vec<Object>) -> Vec3 {
    let result = match ray.cast(scene) {
        None => {
            let background = Vec3::new(0.6, 0.6, 0.6);
            background
        }
        Some(hit) => {
            // choose rendering strategy
            //phong(&hit, scene, ray)
            visualize_normal(&hit, scene, ray)
            //path_tracing(&hit, scene, ray)
        }
    };

    result
}

pub fn main() {
    const WIDTH: u32 = 640;
    const HEIGHT: u32 = 480;
    const SAMPLES: u32 = 1;

    let material = Material {
        albedo: Vec3::new(0.0, 1.0, 0.0),
        emissive: Vec3::zero(),
    };

    let mut scene: Vec<Object> = Vec::new();
    scene.push(Object {
        geometry: Sphere {
            center: Vec3::new(0.0, 0.0, 3.0),
            radius: 1.0,
        },
        material: material,
    });

    let r = 10000.0;
    scene.push(Object {
        geometry: Sphere {
            center: Vec3::new(0.0, r + 1.0, 3.0),
            radius: r,
        },
        material: Material {
            albedo: Vec3::new(0.0, 0.6, 1.0),
            emissive: Vec3::zero(),
        },
    });

    let pixels = vec![0; 3 * WIDTH as usize * HEIGHT as usize];
    let mut buffer = ImageBuffer::from_raw(WIDTH, HEIGHT, pixels).unwrap();

    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            let mut pixel = Vec3::new(0.0, 0.0, 0.0);
            let ray = camera_ray(x, y, WIDTH, HEIGHT);

            for _ in 0..SAMPLES {
                pixel = pixel + cast_ray(&ray, &scene);
            }

            pixel = (pixel * (u8::MAX as f32)) / (SAMPLES as f32);
            buffer.put_pixel(x, y, Rgb([pixel.x as u8, pixel.y as u8, pixel.z as u8]));
        }
    }

    match buffer.save("output.png") {
        Err(_) => panic!("Could not save file"),
        Ok(_) => println!("Saved file"),
    };
}
