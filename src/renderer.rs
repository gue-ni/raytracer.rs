use crate::camera::*;
use crate::common::*;
use crate::geometry::*;
use crate::material::*;
use crate::ray::*;
use crate::vector::*;

use image::RgbImage;

//
//use rand::Rng;

use std::thread;
use std::thread::available_parallelism;

fn get_xy(index: u32, width: u32) -> (u32, u32) {
    let x = index % width;
    let y = index / width;
    (x, y)
}

fn print_progress(current_sample: u32, total_samples: u32) {
    let percentage = current_sample as f32 / total_samples as f32 * 100.0;
    println!(
        "Progress: {:3.1?} % ({}/{})",
        percentage, current_sample, total_samples
    );
}

pub struct Renderer;

impl Renderer {
    /// Visualize Normal Vector
    #[allow(dead_code)]
    fn visualize_normal(hit: &HitRecord, _scene: &Scene, _incoming: &Ray, _depth: u32) -> Vec3f {
        (Vec3f::from(1.0) + hit.normal * Vec3f::new(1.0, -1.0, -1.0)) * 0.5
    }

    /// Whitted Ray-Tracing
    #[allow(dead_code)]
    fn ray_tracing(hit: &HitRecord, scene: &Scene, incoming: &Ray, depth: u32) -> Vec3f {
        let material = scene.objects[hit.idx].material;

        let light_pos = Vec3f::new(0.0, -3.5, 4.0);
        let light_intensity = 1.0;
        let light_color = Vec3f::from(1.0) * light_intensity;
        let light_dir = Vec3f::normalize(light_pos - hit.point);

        let ray = Ray::new(hit.point, light_dir);
        let _shadow = match scene.hit(&ray, 0.0, f32::INFINITY) {
            Some(_) => 1.0,
            None => 0.0,
        };

        match material.material {
            MaterialType::Mirror => {
                let reflected = reflect(incoming.direction, hit.normal);
                let ray = Ray::new(hit.point, reflected);
                material.albedo * Self::trace(&ray, scene, depth - 1) * 0.9
            }
            _ => {
                let ka = 0.25;
                let kd = 1.0;
                let ks = 1.0;
                let alpha = 16.0;

                let ambient = light_color * ka;

                let cos_theta = f32::max(Vec3f::dot(hit.normal, light_dir), 0.0);
                let diffuse = light_color * cos_theta * kd;

                let view_dir = -incoming.direction;
                let halfway_dir = Vec3f::normalize(light_dir + view_dir);
                let specular = light_color
                    * f32::max(Vec3f::dot(hit.normal, halfway_dir), 0.0).powf(alpha)
                    * ks;

                (ambient + (diffuse + specular) * _shadow) * material.albedo
            }
        }
    }

    /// Naive, unbiased monte-carlo path tracing
    /// This function implements the rendering equation using monte-carlo integration
    /// Rendering Equation:
    /// L(p, wo) = Le + ∫ Li(p, wi) fr(wo, wi) cos(theta) dw
    ///
    /// Monte-Carlo:
    /// L(p, wo) = Le + 1/N * Σ (fr(wo, wi) * cos(theta) / pdf(wi))
    ///
    #[allow(dead_code)]
    fn naive_path_tracing(hit: &HitRecord, scene: &Scene, incoming: &Ray, depth: u32) -> Vec3f {
        // Material properties
        let material = scene.objects[hit.idx].material;
        let emittance = material.albedo * material.emittance;

        // Direction toward camera
        let wo = -incoming.direction;

        // Orient normal correctly
        //let normal = hit.normal * Vec3::dot(hit.normal, wo).signum();
        let normal = hit.normal;

        // Get outgoing ray direction and (brdf * cos_theta / pdf)
        let (wi, brdf_multiplier) = material.sample(normal, wo);

        // Reflected ray
        let ray = Ray::new(hit.point + normal * 0.001, wi);

        // Integral is of the form 'emittance + trace() * brdf * cos_theta / pdf'
        emittance + Self::trace(&ray, scene, depth - 1) * brdf_multiplier
    }

    /// Trace ray into scene, returns color
    fn trace(ray: &Ray, scene: &Scene, depth: u32) -> Vec3f {
        if depth > 0 {
            match scene.hit(ray, 0.0, f32::INFINITY) {
                None => scene.background,
                Some(hit) => Self::naive_path_tracing(&hit, scene, ray, depth),
            }
        } else {
            Vec3f::from(0.0)
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
                    let ray = camera.ray((x, y));
                    framebuffer[(y * width + x) as usize] +=
                        Self::trace(&ray, scene, bounces) / (samples as f32);
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

                            let ray = camera.ray(xy);

                            chunk[i] += Self::trace(&ray, scene, bounces) / (samples as f32);
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
        let multithreading = true;
        if multithreading {
            Self::render_multithreaded(camera, scene, samples, bounces)
        } else {
            Self::render_singlethread(camera, scene, samples, bounces)
        }
    }
}
