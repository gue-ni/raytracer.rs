mod camera;
mod common;
mod geometry;
mod material;
mod ray;
mod vector;

use crate::camera::*;
use crate::common::*;
use crate::geometry::*;
use crate::material::*;
use crate::ray::*;
use crate::vector::*;

use image::{Rgb, Rgb32FImage, RgbImage};
use std::path::Path;
use std::time::Instant;
use std::vec;

extern crate rand;
use rand::Rng;

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

pub fn visualize_normal(hit: &HitRecord, _scene: &Scene, _incoming: &Ray, _depth: u32) -> Vec3f {
    (Vec3f::fill(1.0) + hit.normal * Vec3f::new(1.0, -1.0, -1.0)) * 0.5
}

pub fn naive_path_tracing_rr(hit: &HitRecord, scene: &Scene, incoming: &Ray, depth: u32) -> Vec3f {
    let material = scene.objects[hit.idx].material;
    let emittance = material.emittance();

    // russian roulette
    let rr_prob = 0.7;
    let mut rng = rand::thread_rng();
    if rng.gen_range(0.0..1.0) >= rr_prob {
        return emittance;
    }

    let wo = -incoming.direction;
    let (wi, pdf) = material.sample(hit.normal, wo);
    let ray = Ray::new(hit.point, wi);
    let bsdf = material.bsdf(hit.normal, wo, wi);
    let cos_theta = Vec3f::dot(hit.normal, wi);
    emittance + cast_ray(&ray, scene, depth - 1) * bsdf * cos_theta / (pdf * rr_prob)
}

pub fn naive_path_tracing(hit: &HitRecord, scene: &Scene, incoming: &Ray, depth: u32) -> Vec3f {
    let material = scene.objects[hit.idx].material;
    let wo = -incoming.direction;
    let (wi, pdf) = material.sample(hit.normal, wo);
    let ray = Ray::new(hit.point, wi);
    let bsdf = material.bsdf(hit.normal, wo, wi);
    let cos_theta = Vec3f::dot(hit.normal, wi);
    material.emittance() + cast_ray(&ray, scene, depth - 1) * bsdf * cos_theta / pdf
}

pub fn cast_ray(ray: &Ray, scene: &Scene, depth: u32) -> Vec3f {
    match ray.cast(scene) {
        Some(hit) if depth > 0 => naive_path_tracing_rr(&hit, scene, ray, depth),
        Some(_) => Vec3f::fill(0.0),
        None => scene.background,
    }
}

pub fn render(camera: &Camera, scene: &Scene, samples: u32, bounces: u32) -> RgbImage {
    let width = camera.resolution.x as u32;
    let height = camera.resolution.y as u32;

    let mut framebuffer = vec![Vec3f::fill(0.0); width as usize * height as usize];

    for s in 0..samples {
        for y in 0..height {
            for x in 0..width {
                let ray = camera.ray((x, y));
                let sample = cast_ray(&ray, &scene, bounces);

                framebuffer[(y * width + x) as usize] += sample;
            }
        }

        if s % 5 == 0 {
            println!("Progress: {:3.1?} %", s as f32 / samples as f32 * 100.0);
        }
    }

    let mut image = RgbImage::new(width, height);
    for y in 0..height {
        for x in 0..width {
            let index = (y * width + x) as usize;
            let pixel = framebuffer[index] * (u8::MAX as f32) / (samples as f32);
            image.put_pixel(x, y, Rgb([pixel.x as u8, pixel.y as u8, pixel.z as u8]));
        }
    }

    image
}

pub fn render1(camera: &Camera, scene: &Scene, samples: u32, bounces: u32) -> RgbImage {
    let width = camera.resolution.x as u32;
    let height = camera.resolution.y as u32;

    let mut image = RgbImage::new(camera.resolution.x as u32, camera.resolution.y as u32);

    for y in 0..height {
        for x in 0..width {
            let mut pixel = Vec3f::fill(0.0);
            let ray = camera.ray((x, y));

            for _ in 0..samples {
                pixel += cast_ray(&ray, &scene, bounces);
            }

            pixel = (pixel * (u8::MAX as f32)) / (samples as f32);
            image.put_pixel(x, y, Rgb([pixel.x as u8, pixel.y as u8, pixel.z as u8]));
        }
    }

    image
}

pub fn main() {
    let wall = Material::diffuse(Vec3f::fill(1.0));
    let light = Material::emissive(Vec3f::fill(1.0), 5.0);
    let light_2 = Material::emissive(Vec3f::fill(1.0), 1.0);

    let mut scene: Scene = Scene::new(Vec3f::new(0.68, 0.87, 0.96));

    // right
    scene.add(Object {
        geometry: Sphere::new(Vec3f::new(1.75, 0.0, 4.0), 0.5),
        material: Material::diffuse(Vec3f::new(0.0, 1.0, 0.0)),
    });
    // middle
    scene.add(Object {
        geometry: Sphere::new(Vec3f::new(0.0, 0.0, 4.0), 1.0),
        material: Material::diffuse(Vec3f::new(1.0, 0.0, 0.0)),
    });
    // left
    scene.add(Object {
        geometry: Sphere::new(Vec3f::new(-1.75, 0.0, 4.0), 0.5),
        material: Material::diffuse(Vec3f::new(0.0, 0.0, 1.0)),
    });

    // light
    /*
    scene.add(Object {
        geometry: Sphere::new(Vec3f::new(0.0, -1.5, 4.0), 0.3),
        material: light,
    });
    */

    let r = 100000.0;
    let s = 1.0;
    let w = 4.0;

    let room_center = Vec3f::new(0.0, 0.0, 5.0);

    scene.add(Object {
        geometry: Sphere::new(Vec3f::new(0.0, -(r + w), 5.0), r),
        material: light_2,
    });
    scene.add(Object {
        geometry: Sphere::new(Vec3f::new(0.0, r + s, 5.0), r),
        material: wall,
    });
    scene.add(Object {
        geometry: Sphere::new(Vec3f::new(-(r + w), 0.0, 5.0), r),
        material: wall,
    });
    scene.add(Object {
        geometry: Sphere::new(Vec3f::new(r + w, 0.0, 5.0), r),
        material: wall,
    });
    scene.add(Object {
        geometry: Sphere::new(Vec3f::new(0.0, 0.0, 5.0 + (r + w)), r),
        material: wall,
    });

    const WIDTH: u32 = 640;
    const HEIGHT: u32 = 480;
    const SAMPLES: u32 = 32;
    const BOUNCES: u32 = 3;

    let camera = Camera::new(Vec3f::new(0.0, 0.0, 0.0), (WIDTH, HEIGHT));

    let now = Instant::now();
    let buffer = render(&camera, &scene, SAMPLES, BOUNCES);
    let elapsed = now.elapsed();

    println!(
        "{}x{}, samples: {}, bounces: {}",
        WIDTH, HEIGHT, SAMPLES, BOUNCES
    );
    println!("Elapsed time: {:.2?}", elapsed);

    //let filename = format!("img/render/render-{}x{}-s{}-b{}.png", WIDTH, HEIGHT, SAMPLES, BOUNCES);
    let filename = format!("img/render/render.png");
    let path = Path::new(&filename);

    match buffer.save(&path) {
        Err(_) => panic!("Could not save file"),
        Ok(_) => println!("Saved output to {:?}", path),
    };
}

#[test]
fn test_sample() {}
