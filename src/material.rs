use crate::common::*;
use crate::vector::*;
use std::f32::consts::PI;

// Bidirectional Scattering Distribution Function (BSDF)
pub trait BSDF {
    // return outgoing vector and pdf
    fn sample_f(&self, normal: Vec3f, wo: Vec3f) -> (Vec3f, f32);
    // 
    fn bsdf(&self, normal: Vec3f, wo: Vec3f, wi: Vec3f) -> Vec3f;
    // return emittance * albedo
    fn emittance(&self) -> Vec3f;
}

#[derive(Debug, Copy, Clone)]
pub enum Material {
    Diffuse { albedo: Vec3f, emittance: f32 },
    Physical { albedo: Vec3f, emittance: f32, roughness: f32 },
    Specular
}

impl Material {
    pub fn diffuse(color: Vec3f) -> Self {
        Material::Diffuse { albedo: color, emittance: 0.0 }
    }

    pub fn emissive(color: Vec3f, intensity: f32) -> Self {
        Material::Diffuse { albedo: color, emittance: intensity }
    }
}

impl BSDF for Material {
    fn sample_f(&self, normal: Vec3f, wo: Vec3f) -> (Vec3f, f32) {
        match self {
            _ => {
                let omega = uniform_sample_hemisphere(normal);
                let pdf = 1.0 / (2.0 * PI);
                (omega, pdf)
            }
        }
    }
     
    fn bsdf(&self, normal: Vec3f, wo: Vec3f, wi: Vec3f) -> Vec3f {
        match self {
            Material::Diffuse { albedo, .. } => *albedo / PI,
            _ => Vec3f::fill(0.0)
        }
    }
    
    fn emittance(&self) -> Vec3f {
        match self {
            Material::Physical { emittance, albedo, .. } => *albedo * *emittance,
            Material::Diffuse  { emittance, albedo }     => *albedo * *emittance,
            _ => Vec3f::fill(0.0)
        }
    }
}

