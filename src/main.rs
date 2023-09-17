mod vector;
use crate::vector::{ Vec3, normalize, dot, reflect };

mod geometry;
use crate::geometry::*;

mod ray;
use crate::ray::*;

mod common;
use crate::common::*;

use std::vec;
use image::{ Rgb, ImageBuffer };

#[allow(dead_code)]
#[derive(Debug)]
pub struct Material {
    albedo: Vec3,
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

pub fn camera_ray(x: u32, y: u32, x_res: u32, y_res: u32) -> Ray {
    let mut ray_target = Vec3::new(
        ((x as f32) / (x_res as f32)) * 2.0 - 1.0,
        ((y as f32) / (y_res as f32)) * 2.0 - 1.0,
        1.0
    );

    let aspect_ratio = (x_res as f32) / (y_res as f32);
    ray_target.y /= aspect_ratio;

    let origin = Vec3::new(0.0, 0.0, 0.0);
    let direction = normalize(ray_target - origin);

    Ray::new(origin, direction)
}

pub fn phong(hit: &HitRecord, scene: &Vec<Sphere>, incoming: &Ray) -> Vec3 {
    let light = Light { position: Vec3::new(1.0, 10.0, 5.0), color: Vec3::one() };
    let lights = vec![light];

    //let albedo = hit.normal * 0.5 + 0.5;
    let albedo = Vec3::new(1.0, 0.0, 0.0);

    let mut result = Vec3::zero();

    for light in lights {
        let _sphere = scene[hit.idx];

        let light_dir = normalize(hit.point - light.position);

        let ambient = light.color * 0.5;

        let diffuse = light.color * f32::max(dot(hit.normal, light_dir), 0.0);

        let reflected = reflect(light_dir, hit.normal);
        let spec = f32::max(dot(incoming.direction, reflected), 0.0).powf(32.0);
        let specular = light.color * spec * 0.5;

        let ray = Ray::new(hit.point, light_dir);

        let in_shadow = match ray.cast(scene) {
            None => 1.0,
            Some(_) => 0.0,
        };

        result = result + (ambient + (diffuse + specular) * in_shadow) * albedo;
    }

    result
}

pub fn render(
    strategy: RenderStrategy,
    hit: &HitRecord,
    scene: &Vec<Sphere>,
    incoming: &Ray
) -> Vec3 {
    match strategy {
        RenderStrategy::Phong => { phong(&hit, scene, incoming) }
        RenderStrategy::PathTracing => { Vec3::zero() }
    }
}

pub fn cast_ray(ray: &Ray, scene: &Vec<Sphere>) -> Vec3 {
    let result = match ray.cast(scene) {
        None => {
            let background = Vec3::new(0.6, 0.6, 0.6);
            background
        }
        Some(hit) => { phong(&hit, scene, ray) }
    };

    result
}

#[test]
fn test_sphere_hit() {
    let sphere = Sphere::new(Vec3::new(0.0, 0.0, 5.0), 1.0);
    let ray = Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0));
    let hit = sphere.test(&ray, 0.0, f32::INFINITY).unwrap();
    assert_eq!(hit.t, 4.0);
    assert_eq!(hit.point, Vec3::new(0.0, 0.0, 4.0));
    assert_eq!(hit.normal, Vec3::new(0.0, 0.0, -1.0));
}

pub fn main() {
    const WIDTH: u32 = 640;
    const HEIGHT: u32 = 480;
    const SAMPLES: u32 = 1;

    let mut scene: Vec<Sphere> = Vec::new();
    scene.push(Sphere::new(Vec3::new(0.0, 0.0, 3.0), 1.0));

    let r = 10000.0;
    scene.push(Sphere::new(Vec3::new(0.0, r + 1.0, 3.0), r));

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
