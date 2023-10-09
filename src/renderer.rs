use crate::camera::*;
use crate::common::*;
use crate::geometry::*;
use crate::material::*;
use crate::ray::*;
use crate::vector::*;

use image::RgbImage;
use std::thread;
use std::thread::available_parallelism;
use std::vec;

fn get_xy(index: u32, width: u32) -> (u32, u32) {
    let x = index % width;
    let y = index / width;
    (x, y)
}

fn print_progress(current_sample: u32, total_samples: u32) {
    let percentage = current_sample as f64 / total_samples as f64 * 100.0;
    println!(
        "Progress: {:3.1?} % ({}/{})",
        percentage, current_sample, total_samples
    );
}

pub struct Renderer;

impl Renderer {
    /// Visualize Normal Vector
    #[allow(dead_code)]
    fn visualize_normal(ray: &Ray, scene: &Scene, _bounce: u32) -> Vec3f {
        if let Some((hit, _)) = scene.closest_hit(ray, 0.001, f64::INFINITY) {
            (Vec3f::from(1.0) + hit.normal * Vec3f::new(1.0, -1.0, -1.0)) * 0.5
        } else {
            scene.background
        }
    }

    fn sample_lights(scene: &Scene, hit: &Hit, material: &Material, wo: Vec3f) -> Vec3f {
        if material.material == MaterialType::Mirror
            || material.material == MaterialType::Transparent
        {
            return Vec3::from(0.0);
        }

        let mut direct_light = Vec3::from(0.0);
        let point = hit.get_point();

        for &light in &scene.lights {
            let (direction, distance, normal) = light.sample(hit.point);
            let shadow_ray = Ray::new(point, direction);

            let cos_theta = Vec3::dot(normal, -direction);

            let closest = scene.closest_hit(&shadow_ray, 0.001, f64::INFINITY);

            if (closest.is_none() || distance < closest.unwrap().0.t) && 0.0 < cos_theta {
                let emission = light.emission;

                let pdf = {
                    let distance2 = distance * distance;
                    let area = light.geometry.surface_area();
                    distance2 / (area * cos_theta)
                };

                let bsdf = material.bsdf(hit.normal, wo, direction);

                direct_light += bsdf * Vec3::dot(hit.normal, direction).abs() * emission / pdf;
            }
        }

        direct_light / (scene.lights.len() as f64)
    }

    #[allow(dead_code)]
    fn path_tracing(ray: &Ray, scene: &Scene, bounce: u32) -> Vec3f {
        if let Some((hit, idx)) = scene.closest_hit(ray, 0.001, f64::INFINITY) {
            let material = scene.objects[idx].material;
            let point = hit.get_point();
            let wo = -ray.direction;

            let mut color = material.albedo * material.emittance;

            color += Self::sample_lights(scene, &hit, &material, wo);

            if 0 < bounce {
                let (wi, pdf) = material.sample_f(hit.normal, wo);
                let bsdf = material.bsdf(hit.normal, wo, wi);
                let cos_theta = Vec3::dot(hit.normal, wi).abs();
                let ray = Ray::new(point, wi);
                color += Self::path_tracing(&ray, scene, bounce - 1) * bsdf * cos_theta / pdf;
            }

            color
        } else {
            scene.background
        }
    }

    #[allow(dead_code)]
    fn render_singlethread(camera: &Camera, scene: &Scene, samples: u32, bounces: u32) -> RgbImage {
        let width = camera.resolution.x as u32;
        let height = camera.resolution.y as u32;

        let mut framebuffer = vec![Vec3f::from(0.0); width as usize * height as usize];

        for sample in 0..samples {
            for y in 0..height {
                for x in 0..width {
                    let ray = camera.get_ray((x, y));
                    let color = Self::path_tracing(&ray, scene, bounces) / (samples as f64);
                    assert!(0.0 <= f64::min(color.x, f64::min(color.y, color.z)));
                    framebuffer[(y * width + x) as usize] += color;
                }
            }
            if sample % 5 == 0 {
                print_progress(sample, samples);
            }
        }

        to_image(framebuffer, width as u32, height as u32)
    }

    #[allow(dead_code)]
    fn render_multithreaded(
        camera: &Camera,
        scene: &Scene,
        samples: u32,
        bounces: u32,
    ) -> RgbImage {
        let width = camera.resolution.x as u32;
        let height = camera.resolution.y as u32;

        let mut framebuffer = vec![Vec3f::from(0.0); (width * height) as usize];

        // leave one thread for operating the computer : )
        let worker_count = usize::max(available_parallelism().unwrap().get() - 1, 2);
        let chunk_size = framebuffer.len() / worker_count;

        println!("workers = {}", worker_count);

        thread::scope(|scope| {
            for (worker, chunk) in framebuffer.chunks_mut(chunk_size).enumerate() {
                scope.spawn(move || {
                    for sample in 0..samples {
                        for i in 0..chunk.len() {
                            let xy = get_xy((worker * chunk_size + i) as u32, width);
                            let ray = camera.get_ray(xy);
                            let color = Self::path_tracing(&ray, scene, bounces);
                            assert!(0.0 <= f64::min(color.x, f64::min(color.y, color.z)));
                            chunk[i] += color / (samples as f64);
                        }
                        if worker == 0 && sample % 5 == 0 {
                            print_progress(sample, samples);
                        }
                    }
                });
            }
        });

        to_image(framebuffer, width, height)
    }

    /// Render Scene to RgbImage
    pub fn render(camera: &Camera, scene: &Scene, samples: u32, bounces: u32) -> RgbImage {
        if true {
            Self::render_multithreaded(camera, scene, samples, bounces)
        } else {
            Self::render_singlethread(camera, scene, samples, bounces)
        }
    }
}
