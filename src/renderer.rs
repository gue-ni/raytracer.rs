use crate::camera::*;
use crate::common::*;
use crate::geometry::*;
use crate::material::*;
use crate::ray::*;
use crate::vector::*;

use image::{Rgb, RgbImage};
extern crate rand;
use rand::Rng;

#[allow(dead_code)]
fn visualize_normal(hit: &HitRecord, _scene: &Scene, _incoming: &Ray, _depth: u32) -> Vec3f {
    (Vec3f::fill(1.0) + hit.normal * Vec3f::new(1.0, -1.0, -1.0)) * 0.5
}

#[allow(dead_code)]
fn naive_path_tracing_rr(hit: &HitRecord, scene: &Scene, incoming: &Ray, depth: u32) -> Vec3f {
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
    emittance + trace(&ray, scene, depth - 1) * bsdf * cos_theta / (pdf * rr_prob)
}

#[allow(dead_code)]
fn naive_path_tracing(hit: &HitRecord, scene: &Scene, incoming: &Ray, depth: u32) -> Vec3f {
    let material = scene.objects[hit.idx].material;
    let wo = -incoming.direction;
    let (wi, pdf) = material.sample(hit.normal, wo);
    let ray = Ray::new(hit.point, wi);
    let bsdf = material.bsdf(hit.normal, wo, wi);
    let cos_theta = Vec3f::dot(hit.normal, wi);
    material.emittance() + trace(&ray, scene, depth - 1) * bsdf * cos_theta / pdf
}

fn trace(ray: &Ray, scene: &Scene, depth: u32) -> Vec3f {
    match scene.hit(ray, 0.0, f32::INFINITY) {
        Some(hit) if depth > 0 => naive_path_tracing(&hit, scene, ray, depth),
        Some(_) => Vec3f::fill(0.0),
        None => scene.background,
    }
}

fn render_v1(camera: &Camera, scene: &Scene, samples: u32, bounces: u32) -> RgbImage {
    let width = camera.resolution.x as u32;
    let height = camera.resolution.y as u32;

    let mut framebuffer = vec![Vec3f::fill(0.0); width as usize * height as usize];

    for s in 0..samples {
        for y in 0..height {
            for x in 0..width {
                let ray = camera.ray((x, y));
                framebuffer[(y * width + x) as usize] += trace(&ray, &scene, bounces);
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

#[allow(dead_code)]
fn render_v2(camera: &Camera, scene: &Scene, samples: u32, bounces: u32) -> RgbImage {
    let width = camera.resolution.x as u32;
    let height = camera.resolution.y as u32;

    let mut image = RgbImage::new(camera.resolution.x as u32, camera.resolution.y as u32);

    for y in 0..height {
        for x in 0..width {
            let mut pixel = Vec3f::fill(0.0);
            let ray = camera.ray((x, y));

            for _ in 0..samples {
                pixel += trace(&ray, &scene, bounces);
            }

            pixel = (pixel * (u8::MAX as f32)) / (samples as f32);
            image.put_pixel(x, y, Rgb([pixel.x as u8, pixel.y as u8, pixel.z as u8]));
        }
    }

    image
}

pub fn render(camera: &Camera, scene: &Scene, samples: u32, bounces: u32) -> RgbImage {
    render_v1(camera, scene, samples, bounces)
}
