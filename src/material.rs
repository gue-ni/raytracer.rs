use crate::common::*;
use crate::vector::*;
use crate::onb::Onb;
use std::f32::consts::PI;

/// Bidirectional Scattering Distribution Function (BSDF)
pub trait BSDF {
    /// Returns outgoing vector and pdf
    fn sample(&self, normal: Vec3f, wo: Vec3f) -> (Vec3f, f32);
    /// Returns color of hit
    fn bsdf(&self, normal: Vec3f, wo: Vec3f, wi: Vec3f) -> Vec3f;
}

#[derive(Debug, Copy, Clone)]
pub enum MaterialType {
    /// Perfectly Diffuse
    Diffuse,
    /// Perfectly Specular
    Specular,
    /// Physically-based Material
    Physical,
    /// Refractive
    Transparent,
}

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
            material: MaterialType::Physical,
        }
    }

    pub fn emissive(color: Vec3f, intensity: f32) -> Self {
        Material {
            albedo: color,
            emittance: intensity,
            roughness: 1.0,
            material: MaterialType::Diffuse,
        }
    }

    pub fn physical(color: Vec3f, roughness: f32) -> Self {
        Material {
            albedo: color,
            emittance: 0.0,
            roughness,
            material: MaterialType::Physical,
        }
    }

    pub fn specular(color: Vec3f) -> Self {
        Material {
            albedo: color,
            emittance: 0.0,
            roughness: 0.0,
            material: MaterialType::Specular,
        }
    }
}

impl BSDF for Material {
    fn sample(&self, normal: Vec3f, wo: Vec3f) -> (Vec3f, f32) {
        match self.material {
            MaterialType::Specular => {
                let wi = reflect(-wo, normal);
                let cos_theta = Vec3f::dot(normal, wi);
                (wi, cos_theta)
            }
            MaterialType::Physical => {
                let onb = Onb::new(normal);
                let wi = onb.local_to_world(cosine_weighted_hemisphere())
                let cos_theta = Vec3f::dot(normal, wi);
                let pdf = cos_theta / PI;
                (wi, pdf)
            },
            _ => {
                let pdf = 1.0 / (2.0 * PI);
                let wi = uniform_sample_hemisphere(normal);
                (wi, pdf)
            }
        }
    }

    fn bsdf(&self, _normal: Vec3f, _wo: Vec3f, _wi: Vec3f) -> Vec3f {
        match self.material {
            MaterialType::Specular => self.albedo,
            _ => self.albedo / PI,
        }
    }
}
