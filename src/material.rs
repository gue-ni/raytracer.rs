use crate::common::*;
use crate::vector::*;
use std::f32::consts::PI;

// Bidirectional Scattering Distribution Function (BSDF)
pub trait BSDF {
    fn pdf(&self) -> f32;
    fn eval(&self) -> Vec3f;
    fn sample(&self, normal: Vec3f) -> (Vec3f, Vec3f);
}

// lambertian
#[derive(Debug, Copy, Clone)]
pub struct DiffuseMaterial {
    pub albedo: Vec3f,
    pub emissive: Vec3f,
}

impl BSDF for DiffuseMaterial {
    fn pdf(&self) -> f32 {
        1.0 / (2.0 * PI)
    }

    fn eval(&self) -> Vec3f {
        self.albedo / PI
    }

    fn sample(&self, normal: Vec3f) -> (Vec3f, Vec3f) {
        let omega = uniform_sample_hemisphere(normal);
        let cos_theta = Vec3f::dot(normal, omega);
        let brdf_multiplier = (self.eval() * cos_theta) / self.pdf();
        (omega, brdf_multiplier)
    }
}

#[allow(dead_code)]
#[derive(Debug, Copy, Clone)]
pub struct PhysicalDiffuseMaterial {
    albedo: Vec3f,
    emissive: Vec3f,
    roughness: f32,
    metallic: f32,
    ao: f32,
}
