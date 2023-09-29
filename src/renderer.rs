use crate::camera::*;
use crate::common::*;
use crate::geometry::*;
use crate::material::*;
use crate::ray::*;
use crate::vector::*;

use image::RgbImage;
use std::f64::consts::PI;

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
    fn direct_lighting(hit: &HitRecord, scene: &Scene, incoming: &Ray, depth: u32) -> Vec3f {
        Vec3::from(0.0)
    }

    /// Path Tracing with Explicit/Direct Light Sampling
    #[allow(dead_code)]
    fn path_tracing_dls(hit: &HitRecord, scene: &Scene, incoming: &Ray, depth: u32) -> Vec3f {
        let object_id = hit.idx;
        let material = scene.objects[object_id].material;

        let mut _direct = Vec3::from(0.0);
        let mut _num_lights = 0;

        // why do i need a reference here?
        for light_id in &scene.lights {
            let light = scene.objects[*light_id];

            if *light_id != object_id {
                let light_vec = light.geometry.center - hit.point;
                let light_dir = light_vec.normalize();

                // what is the difference here?
                let shadow_ray = Ray::new(hit.point + hit.normal * 0.001, light_dir);
                // why does this look kinda shaded?
                //let shadow_ray = Ray::new(hit.point, light_dir);

                if let Some(light_hit) = scene.hit(&shadow_ray, 0.001, f64::INFINITY) {
                    // we hit the light -> not in shadow
                    if light_hit.idx == *light_id {
                        let li = light.material.albedo * light.material.emittance;

                        let cos_theta_x = Vec3::dot(hit.normal, -light_dir);
                        let cos_theta_y = Vec3::dot(light_hit.normal, -light_dir); 
                        
                        let distance = light_vec.length();
                        let area = 4.0 * PI * light.geometry.radius * light.geometry.radius;
                        let p = 1.0
                            / Vec3::dot(light_hit.point - hit.point, hit.point - light_hit.point);

                        //_direct += li * cos_theta_x * (cos_theta_y / (distance * distance)) * area
                        _direct += li * cos_theta_x * p * area
                        //_direct += li *  cos_theta_x * cos_theta_y * (area / (distance * distance))
                    }
                }
            }
        }

        let wo = -incoming.direction;
        let (wi, brdf_multiplier) = material.sample(hit.normal, wo);
        let bias = Vec3::dot(wi, hit.normal).signum() * 0.001;
        let ray = Ray::new(hit.point + hit.normal * bias, wi);

        let emittance = if depth == 0 {
            material.albedo * material.emittance
        } else {
            Vec3::from(0.0)
        };
        let _diffuse = emittance + Self::trace(&ray, scene, depth + 1) * brdf_multiplier;

        let weight = 1.0;
        (_diffuse * (1.0 - weight)) + (_direct * weight)
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

    fn trace_loop(incident: &Ray, scene: &Scene, max_depth: u32) -> Vec3f {

        let radiance = Vec3::from(0.0);
        let throughput = Vec3::from(1.0);

        let mut ray = incident.clone();
        
        for depth in 0..max_depth {
            if let Some(hit) = scene.hit(&ray, 0.001, f64::INFINITY)  {
                let material = scene.objects[hit.idx].material;
                let albedo = material.albedo;
                let emittance = albedo * material.emittance;
                
                let (wi, brdf_multiplier) = material.sample(hit.normal, -ray.direction);

                radiance += emittance * throughput;
                throughput *= brdf_multiplier;
                
                ray = Ray::new(hit.point + hit.normal * 0.001, wi);
            
            } else {
                radiance += scene.background * throughput;
                break;
            }
        }

        radiance
    }


    /// Trace ray into scene, returns radiance
    fn trace(ray: &Ray, scene: &Scene, depth: u32) -> Vec3f {
        if depth < 5 {
            if let Some(hit) = scene.hit(ray, 0.001, f64::INFINITY) {
                Self::path_tracing_dls(&hit, scene, ray, depth)
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
                    framebuffer[(y * width + x) as usize] +=
                        Self::trace(&ray, scene, bounces) / (samples as f64);
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
                            chunk[i] += Self::trace_loop(&ray, scene, bounces) / (samples as f64);
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
            Self::render_multithreaded(camera, scene, samples, 0)
        } else {
            Self::render_singlethread(camera, scene, samples, 0)
        }
    }
}
