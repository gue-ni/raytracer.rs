use crate::common::*;
use crate::vector::*;
use std::f32::consts::PI;

// Bidirectional Scattering Distribution Function (BSDF)
pub trait BSDF {
    // return outgoing vector and pdf
    fn sample(&self, normal: Vec3f, wo: Vec3f) -> (Vec3f, f32);
    //
    fn bsdf(&self, normal: Vec3f, wo: Vec3f, wi: Vec3f) -> Vec3f;
    // return emittance * albedo
    fn emittance(&self) -> Vec3f;
}

#[derive(Debug, Copy, Clone)]
pub enum Material {
    Diffuse {
        albedo: Vec3f,
        emittance: f32,
    },
    Physical {
        albedo: Vec3f,
        emittance: f32,
        roughness: f32,
    },
    Specular,
}

impl Material {
    pub fn diffuse(color: Vec3f) -> Self {
        Material::Diffuse {
            albedo: color,
            emittance: 0.0,
        }
    }

    pub fn emissive(color: Vec3f, intensity: f32) -> Self {
        Material::Diffuse {
            albedo: color,
            emittance: intensity,
        }
    }

    pub fn physical(color: Vec3f, roughness: f32) -> Self {
        Material::Physical {
            albedo: color,
            emittance: 0.0,
            roughness,
        }
    }
}

impl BSDF for Material {
    fn sample(&self, normal: Vec3f, _wo: Vec3f) -> (Vec3f, f32) {
        match self {
            Material::Specular => {
                let pdf = 1.0 / (2.0 * PI);
                let wi = reflect(_wo, normal);
                (wi, pdf)
            }
            Material::Physical { roughness, .. } => {
                let pdf = 1.0 / (2.0 * PI);
                let reflected = reflect(_wo, normal);
                let random = uniform_sample_hemisphere(normal);
                let wi = Vec3f::lerp(reflected, random, *roughness);
                (wi, pdf)
            }
            _ => {
                let pdf = 1.0 / (2.0 * PI);
                let wi = uniform_sample_hemisphere(normal);
                (wi, pdf)
            }
        }
    }

    fn bsdf(&self, _normal: Vec3f, _wo: Vec3f, _wi: Vec3f) -> Vec3f {
        match self {
            Material::Diffuse { albedo, .. } => *albedo / PI,
            Material::Physical { albedo, .. } => *albedo / PI,
            _ => Vec3f::new(0.0, 1.0, 0.0),
        }
    }

    fn emittance(&self) -> Vec3f {
        match self {
            Material::Physical {
                emittance, albedo, ..
            } => *albedo * *emittance,
            Material::Diffuse {
                emittance, albedo, ..
            } => *albedo * *emittance,
            _ => Vec3f::fill(0.0),
        }
    }
}
