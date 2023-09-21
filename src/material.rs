use crate::common::*;
use crate::vector::*;
use std::f32::consts::PI;

// Bidirectional Scattering Distribution Function (BSDF)
pub trait BSDF {
    fn pdf(&self) -> f32;
    fn eval(&self) -> Vec3f;
    fn emissive(&self) -> Vec3f;
    fn sample(&self, normal: Vec3f) -> (Vec3f, Vec3f);
}

pub trait BxDF {
    // return outgoing vector and pdf
    fn sample_f(&self, normal: Vec3f, wo: Vec3f) -> (Vec3f, f32);
    // 
    fn bsdf(&self, normal: Vec3f, wo: Vec3f, wi: Vec3f) -> Vec3f;
    // return emittance * albedo
    fn emittance(&self) -> Vec3f;
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

    fn emissive(&self) -> Vec3f {
        self.emissive 
    }

    fn sample(&self, normal: Vec3f) -> (Vec3f, Vec3f) {
        let omega = uniform_sample_hemisphere(normal);
        let cos_theta = Vec3f::dot(normal, omega);
        let brdf_multiplier = (self.eval() * cos_theta) / self.pdf();
        (omega, brdf_multiplier)
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Material {
    Diffuse { albedo: Vec3f, emissive: Vec3f },
    Physical { albedo: Vec3f, emittance: f32, roughness: f32 },
}

impl BxDF for Material {
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
            Material::Diffuse { emissive, .. } => *emissive,
        }
    }
}

impl BSDF for Material {
    fn pdf(&self) -> f32 {
        match self {
           _ => 1.0 / (2.0 * PI)
        }
    }

    fn eval(&self) -> Vec3f {
        match self {
            Material::Diffuse { albedo, .. } => *albedo / PI,
            _ => Vec3f::fill(0.0)
        }
    }

    fn emissive(&self) -> Vec3f {
       match self {
            Material::Diffuse { emissive, .. } => *emissive,
            _ => Vec3f::fill(0.0),
        }
    } 

    fn sample(&self, normal: Vec3f) -> (Vec3f, Vec3f) {
        match self {
            Material::Diffuse { .. } => {
                let omega = uniform_sample_hemisphere(normal);
                let cos_theta = Vec3f::dot(normal, omega);
                let brdf_multiplier = (self.eval() * cos_theta) / self.pdf();
                (omega, brdf_multiplier)
            },
            _ => (Vec3f::fill(0.0), Vec3f::fill(0.0))
        }
        
    }
}

