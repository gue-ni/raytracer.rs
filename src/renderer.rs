use crate::camera::*;
use crate::common::*;
use crate::geometry::*;
use crate::material::*;
use crate::onb::*;
use crate::ray::*;
use crate::vector::*;

use image::RgbImage;
use std::f64::consts::PI;

//
//use rand::Rng;

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

fn power_heuristic(numf: f64, f_pdf: f64, numg: f64, g_pdf: f64) -> f64 {
    let f = numf * f_pdf;
    let g = numg * g_pdf;
    (f * f) / (f * f + g * g)
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

        let light_pos = Vec3f::new(0.0, -1.5, 4.0);
        let light_intensity = 1.0;
        let light_color = Vec3f::from(1.0) * light_intensity;
        let light_dir = Vec3f::normalize(light_pos - hit.point);

        let ray = Ray::new(hit.point, light_dir);
        let _shadow = match scene.hit(&ray, 0.0, f64::INFINITY) {
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

                let cos_theta = f64::max(Vec3f::dot(hit.normal, light_dir), 0.0);
                let diffuse = light_color * cos_theta * kd;

                let view_dir = -incoming.direction;
                let halfway_dir = Vec3f::normalize(light_dir + view_dir);
                let specular = light_color
                    * f64::max(Vec3f::dot(hit.normal, halfway_dir), 0.0).powf(alpha)
                    * ks;

                (ambient + (diffuse + specular) * _shadow) * material.albedo
            }
        }
    }

    /// Direct Lighting Integrator
    #[allow(dead_code)]
    fn direct_lighting(_hit: &HitRecord, _scene: &Scene, _incoming: &Ray, _depth: u32) -> Vec3f {
        Vec3::from(0.0)
    }

    /// Naive, unbiased monte-carlo path tracing
    /// This function implements the rendering equation using monte-carlo integration
    ///
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
        // Get outgoing ray direction and (brdf * cos_theta / pdf)
        let (wi, brdf_multiplier) = material.sample(hit.normal, wo);
        // Reflected ray
        let bias = Vec3::dot(wi, hit.normal).signum() * 0.001;
        let ray = Ray::new(hit.point + hit.normal * bias, wi);
        emittance + Self::trace(&ray, scene, depth + 1) * brdf_multiplier
    }

    /// Path Tracing with Explicit/Direct Light Sampling
    /// https://computergraphics.stackexchange.com/questions/5152/progressive-path-tracing-with-explicit-light-sampling?noredirect=1&lq=1
    /// https://computergraphics.stackexchange.com/questions/4288/path-weight-for-direct-light-sampling
    /// For some reason this only works if the entire light sphere is visible
    #[allow(dead_code)]
    fn trace_loop_2(incident: &Ray, scene: &Scene, max_bounce: u32) -> Vec3f {
        let mut radiance = Vec3::from(0.0);
        let mut throughput = Vec3::from(1.0);

        let mut ray = incident.clone();

        for bounce in 0..max_bounce {
            if let Some(hit) = scene.hit(&ray, 0.001, f64::INFINITY) {
                let material = scene.objects[hit.idx].material;

                let point = hit.point + hit.normal * 0.001;
                let wi = Onb::local_to_world(hit.normal, cosine_weighted_hemisphere());

                ray = Ray::new(point, wi);

                if bounce == 0 {
                    radiance += throughput * (material.albedo * material.emittance);
                }

                throughput *= material.albedo;

                assert_eq!(scene.lights.len(), 1);

                for &i in &scene.lights {
                    if hit.idx != i {
                        let light = scene.objects[i];
                        let emission = light.material.albedo * light.material.emittance;

                        // sample point on light
                        let (point_on_light, light_normal) = {
                            let normal = vector_on_sphere().normalize();
                            let point = light.geometry.center + normal * light.geometry.radius;
                            (point, normal)
                        };

                        let shadow_ray = Ray::new(point, Vec3::normalize(point_on_light - point));

                        if let Some(light_hit) = scene.hit(&shadow_ray, 0.001, f64::INFINITY) {
                            if light_hit.idx == i
                                && 0.0 < Vec3::dot(light_normal, -shadow_ray.direction)
                                && bounce < max_bounce - 1
                            {
                                if true {
                                    let cos_theta = Vec3::dot(hit.normal, shadow_ray.direction);

                                    let pdf = {
                                        let radius2 = light.geometry.radius * light.geometry.radius;
                                        let area = 4.0 * PI * radius2;
                                        let distance2 = light_hit.t * light_hit.t;
                                        let cos_theta_light =
                                            Vec3::dot(light_normal, -shadow_ray.direction)
                                                .clamp(0.0, 1.0);
                                        distance2 / (cos_theta_light * area)
                                    };

                                    let brdf = material.albedo / PI;

                                    radiance += throughput * emission * cos_theta * brdf / pdf;
                                } else {
                                    let weight = {
                                        let radius2 = light.geometry.radius * light.geometry.radius;
                                        let distance2 = light_hit.t * light_hit.t;

                                        let cos_a_max =
                                            f64::sqrt(1.0 - (radius2 / distance2).clamp(0.0, 1.0));

                                        2.0 * (1.0 - cos_a_max)
                                    };

                                    radiance += (throughput * emission)
                                        * (weight
                                            * Vec3::dot(hit.normal, shadow_ray.direction)
                                                .clamp(0.0, 1.0));
                                }
                            }
                        } else {
                            //assert!(false);
                        }
                    }
                }
            } else {
                radiance += scene.background * throughput;
                break;
            }
        }

        radiance
    }

    #[allow(dead_code)]
    fn trace_loop_1(incident: &Ray, scene: &Scene, max_bounce: u32) -> Vec3f {
        let mut radiance = Vec3::from(0.0);
        let mut throughput = Vec3::from(1.0);

        assert!(0 < max_bounce);

        let mut ray = incident.clone();

        for _ in 0..max_bounce {
            if let Some(hit) = scene.hit(&ray, 0.001, f64::INFINITY) {
                let material = scene.objects[hit.idx].material;
                let emittance = material.albedo * material.emittance;

                //let (wi, brdf_multiplier) = material.sample(hit.normal, -ray.direction);

                let wi = Onb::local_to_world(hit.normal, cosine_weighted_hemisphere());
                //let brdf = material.albedo / PI;
                //let cos_theta = Vec3::dot(hit.normal, -wi);
                //let pdf = cos_theta / PI;

                throughput *= material.albedo;
                radiance += emittance * throughput;

                ray = Ray::new(hit.point + hit.normal * 0.001, wi);
            } else {
                radiance += scene.background * throughput;
                break;
            }
        }

        radiance
    }

    /// Trace ray into scene, returns radiance
    #[allow(dead_code)]
    fn trace(ray: &Ray, scene: &Scene, depth: u32) -> Vec3f {
        if depth > 0 {
            if let Some(hit) = scene.hit(ray, 0.001, f64::INFINITY) {
                Self::naive_path_tracing(&hit, scene, ray, depth)
            } else {
                scene.background
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
                    let color = Self::trace(&ray, scene, bounces) / (samples as f64);
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
        //let worker_count = usize::max(available_parallelism().unwrap().get() - 1, 2);
        let worker_count = available_parallelism().unwrap().get();
        let chunk_size = framebuffer.len() / worker_count;

        println!("workers = {}", worker_count);

        thread::scope(|scope| {
            for (worker, chunk) in framebuffer.chunks_mut(chunk_size).enumerate() {
                scope.spawn(move || {
                    for sample in 0..samples {
                        for i in 0..chunk.len() {
                            let xy = get_xy((worker * chunk_size + i) as u32, width);
                            let ray = camera.ray(xy);
                            let color = Self::trace_loop_2(&ray, scene, bounces);

                            // assert!(0.0 <= f64::min(color.x, f64::min(color.y, color.z)));

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
        let multithreading = true;
        if multithreading {
            Self::render_multithreaded(camera, scene, samples, bounces)
        } else {
            Self::render_singlethread(camera, scene, samples, bounces)
        }
    }
}
