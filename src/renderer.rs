use crate::camera::*;
use crate::common::*;
use crate::geometry::*;
use crate::material::*;
use crate::onb::*;
use crate::ray::*;
use crate::vector::*;

use image::RgbImage;
use std::f64::consts::PI;

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
    fn visualize_normal(hit: &Hit, _scene: &Scene, _incoming: &Ray, _depth: u32) -> Vec3f {
        (Vec3f::from(1.0) + hit.normal * Vec3f::new(1.0, -1.0, -1.0)) * 0.5
        //(hit.normal + 0.5) * 0.5
    }

    /// Returns direction to light and distance
    fn sample_light(object: &Object, point: Vec3f) -> (Vec3f, f64) {
        let sphere = object.geometry;
        let normal = vector_on_sphere();
        let point_on_light = sphere.center + normal * sphere.radius;
        let light_dir = point_on_light - point;
        let distance = light_dir.length();
        (light_dir / distance, distance)
    }

    fn sample_lights(scene: &Scene, hit: &Hit, wo: Vec3f) -> Vec3f {
        let mut direct_light = Vec3::from(0.0);
        
        let material = scene.objects[hit.idx].material;
        
        for &i in &scene.lights {
            if i == hit.idx {
                continue;
            }
            
            let light = scene.objects[i];
            let (direction, distance) = Self::sample_light(&light, hit.point);
            let shadow_ray = Ray::new(hit.point, direction);

            if let Some(lhit) = scene.hit(&shadow_ray, 0.001, f64::INFINITY) {
                if hit.idx != lhit.idx && distance <= lhit.t {
                    //direct_light += ();
                }
            }
        }

        direct_light
    }

    fn path_tracing_dsa(ray: &Ray, scene: &Scene, bounce: u32) -> Vec3f {
        if let Some(hit) = scene.hit(ray, 0.001, f64::INFINITY) {
            let material = scene.objects[hit.idx].material;
            let point = hit.point + hit.normal * 0.001;
            let wo = -ray.direction;

            let mut color = material.albedo * material.emittance;
            
            color += Self::sample_lights(scene, &hit, wo);
            
            if 0 < bounce {
                let (wi, pdf) = material.sample_f(hit.normal, wo);
                let bsdf = material.bsdf(hit.normal, wo, wi); 
                let cos_theta = Vec3::dot(hit.normal, wi).abs();
                let ray = Ray::new(point, wi);
                color += Self::path_tracing_dsa(&ray, scene, bounce - 1) * bsdf * cos_theta / pdf;
            }

            color
        } else {
            scene.background
        }
    }

    

    /// Path Tracing with Explicit/Direct Light Sampling
    /// https://computergraphics.stackexchange.com/questions/5152/progressive-path-tracing-with-explicit-light-sampling?noredirect=1&lq=1
    /// https://computergraphics.stackexchange.com/questions/4288/path-weight-for-direct-light-sampling
    /// For some reason this only works if the entire light sphere is visible
    #[allow(dead_code)]
    fn path_tracing_nee(incident: &Ray, scene: &Scene, max_bounce: u32) -> Vec3f {
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
                                    let cos_theta =
                                        Vec3::dot(hit.normal, shadow_ray.direction).clamp(0.0, 1.0);

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
    fn path_tracing(incident: &Ray, scene: &Scene, max_bounce: u32) -> Vec3f {
        let mut radiance = Vec3::from(0.0);
        let mut throughput = Vec3::from(1.0);

        assert!(0 < max_bounce);

        let mut ray = incident.clone();

        for _ in 0..max_bounce {
            if let Some(hit) = scene.hit(&ray, 0.001, f64::INFINITY) {
                let material = scene.objects[hit.idx].material;
                let emittance = material.albedo * material.emittance;

                let (wi, brdf_multiplier) = material.sample(hit.normal, -ray.direction);

                throughput *= brdf_multiplier;
                radiance += emittance * throughput;

                ray = Ray::new(hit.point + hit.normal * 0.001, wi);
            } else {
                radiance += scene.background * throughput;
                break;
            }
        }

        radiance
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
    fn path_tracing_recursive(incident: &Ray, scene: &Scene, depth: u32) -> Vec3f {
        if depth == 0 {
            return Vec3::from(0.0);
        }

        if let Some(hit) = scene.hit(incident, 0.001, f64::INFINITY) {
            let material = scene.objects[hit.idx].material;
            let emittance = material.albedo * material.emittance;
            let (reflected, brdf_multiplier) = material.sample(hit.normal, -incident.direction);
            let bias = Vec3::dot(reflected, hit.normal).signum() * 0.001;
            let ray = Ray::new(hit.point + hit.normal * bias, reflected);
            emittance + Self::path_tracing_recursive(&ray, scene, depth - 1) * brdf_multiplier
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
                    let ray = camera.ray((x, y));
                    let color = Self::path_tracing(&ray, scene, bounces) / (samples as f64);
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
                            let ray = camera.ray(xy);
                            let color = Self::path_tracing_nee(&ray, scene, bounces);
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
        let multithreading = true;
        if multithreading {
            Self::render_multithreaded(camera, scene, samples, bounces)
        } else {
            Self::render_singlethread(camera, scene, samples, bounces)
        }
    }
}
