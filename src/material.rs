use crate::common::*;
use crate::onb::Onb;
use crate::vector::*;
use std::f32::consts::PI;

/// Bidirectional Scattering Distribution Function (BSDF)
pub trait BSDF {
    /// Returns outgoing vector and bsdf multiplier
    fn sample(&self, normal: Vec3f, wo: Vec3f) -> (Vec3f, Vec3f);
}

#[derive(Debug, Copy, Clone)]
pub enum MaterialType {
    /// Mirror (perfectly specular)
    Mirror,
    /// Uniform Hemisphere Sampling (perfectly diffuse)
    Uniform,
    /// Cosine-weighted Hemisphere Sampling (perfectly diffuse)
    CosineWeighted,
    /// Physically based model
    CookTorrance,
    ///
    Transparent,
}

/// Material Properties
#[derive(Debug, Copy, Clone)]
pub struct Material {
    pub albedo: Vec3f,
    pub emittance: f32,
    pub roughness: f32,
    pub material: MaterialType,
}

impl Material {
    pub fn diffuse(color: Vec3f) -> Self {
        Material {
            albedo: color,
            emittance: 0.0,
            roughness: 1.0,
            material: MaterialType::CosineWeighted,
        }
    }

    pub fn emissive(color: Vec3f, intensity: f32) -> Self {
        Material {
            albedo: color,
            emittance: intensity,
            roughness: 1.0,
            material: MaterialType::CosineWeighted,
        }
    }

    pub fn physical(color: Vec3f, roughness: f32) -> Self {
        Material {
            albedo: color,
            emittance: 0.0,
            roughness,
            material: MaterialType::CosineWeighted,
        }
    }

    pub fn specular(color: Vec3f) -> Self {
        Material {
            albedo: color,
            emittance: 0.0,
            roughness: 0.0,
            material: MaterialType::Mirror,
        }
    }

    pub fn transparent(color: Vec3f) -> Self {
        Material {
            albedo: color,
            emittance: 0.0,
            roughness: 0.0,
            material: MaterialType::Transparent,
        }
    }
}

impl BSDF for Material {
    fn sample(&self, normal: Vec3f, wo: Vec3f) -> (Vec3f, Vec3f) {
        match self.material {
            MaterialType::Mirror => {
                let wi = reflect(-wo, normal);
                let bsdf = self.albedo;
                (wi, bsdf)
            }
            MaterialType::Transparent => {
                // TODO
                let wi = reflect(-wo, normal);
                let bsdf = self.albedo;
                (wi, bsdf)
            }
            MaterialType::CosineWeighted => {
                // Cosine-weighted hemisphere sampling
                // brdf = albedo / 𝜋
                // pdf = cos(θ) / 𝜋
                let wi = Onb::local_to_world(normal, cosine_weighted_hemisphere());
                let cos_theta = Vec3f::dot(normal, wi);
                let bsdf = self.albedo / PI;
                let pdf = cos_theta / PI;
                (wi, bsdf * cos_theta / pdf)
            }
            MaterialType::Uniform => {
                // Uniform hemisphere sampling
                // brdf = albedo / 𝜋
                // pdf = 1 / 2 * 𝜋
                let pdf = 1.0 / (2.0 * PI);
                let wi = Onb::local_to_world(normal, uniform_hemisphere());
                let cos_theta = Vec3f::dot(normal, wi);
                let bsdf = self.albedo / PI;
                (wi, bsdf * cos_theta / pdf)
            }
            MaterialType::CookTorrance => {
                // TODO:
                let wi = Onb::local_to_world(normal, cosine_weighted_hemisphere());
                let cos_theta = Vec3f::dot(normal, wi);
                let bsdf = self.albedo / PI;
                let pdf = cos_theta / PI;
                (wi, bsdf * cos_theta / pdf)
            }
        }
    }
}
