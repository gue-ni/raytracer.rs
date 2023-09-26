use crate::common::*;
use crate::onb::Onb;
use crate::vector::*;

use serde::{Deserialize, Serialize};
use std::f32::consts::PI;
use rand::Rng;

/// Bidirectional Scattering Distribution Function (BSDF)
pub trait BSDF {
    /// Returns outgoing vector and brdf multiplier
    /// 'normal' - Normal vector at hit point
    /// 'wo' - Direction vector toward camera
    fn sample(&self, normal: Vec3f, wo: Vec3f) -> (Vec3f, Vec3f);
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum MaterialType {
    /// Mirror (perfectly specular)
    Mirror,
    /// Uniform Hemisphere Sampling (perfectly diffuse)
    Uniform,
    /// Cosine-weighted Hemisphere Sampling (perfectly diffuse)
    CosineWeighted,
    /// Cook-Torrance Reflection Model
    CookTorrance,
    ///
    Transparent,
}

/// Material Properties
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct Material {
    pub albedo: Vec3f,
    pub emittance: f32,
    pub roughness: f32,
    pub ior: f32,
    pub metallic: f32,
    pub material: MaterialType,
}

impl Material {
    pub fn diffuse(color: Vec3f) -> Self {
        Material {
            albedo: color,
            emittance: 0.0,
            roughness: 0.0,
            ior: 0.0,
            metallic: 0.0,
            material: MaterialType::CosineWeighted,
        }
    }

    pub fn emissive(color: Vec3f, intensity: f32) -> Self {
        Material {
            albedo: color,
            emittance: intensity,
            roughness: 0.0,
            ior: 0.0,
            metallic: 0.0,
            material: MaterialType::CosineWeighted,
        }
    }

    pub fn physical(color: Vec3f, roughness: f32, metallic: f32) -> Self {
        Material {
            albedo: color,
            roughness,
            metallic,
            emittance: 0.0,
            ior: 0.0,
            material: MaterialType::CookTorrance,
        }
    }

    pub fn mirror(color: Vec3f) -> Self {
        Material {
            albedo: color,
            emittance: 0.0,
            roughness: 0.0,
            ior: 0.0,
            metallic: 0.0,
            material: MaterialType::Mirror,
        }
    }

    pub fn transparent(color: Vec3f) -> Self {
        Material {
            albedo: color,
            emittance: 0.0,
            roughness: 0.0,
            ior: 1.52, //glass
            metallic: 0.0,
            material: MaterialType::Transparent,
        }
    }
}

/// Schlick's Fresnel Approximation
fn fresnel_schlick(f0: Vec3f, cos_theta: f32) -> Vec3f {
    f0 + (Vec3f::from(1.0) - f0) * f32::powf((1.0 - cos_theta).clamp(0.0, 1.0), 5.0)
}

fn distribution_ggx(normal: Vec3f, halfway: Vec3f, roughness: f32) -> f32 {
    let a2 = roughness * roughness;
    let ndoth = f32::max(Vec3f::dot(normal, halfway), 0.0);
    let ndoth2 = ndoth * ndoth;
    let nom = a2;
    let denom = ndoth2 * (a2 - 1.0) + 1.0;
    nom / (PI * denom * denom)
}

fn geometry_schlick_ggx(ndotv: f32, roughness: f32) -> f32 {
    let r = roughness + 1.0;
    let k = (r * r) / 8.0;
    let num = ndotv;
    let denom = ndotv * (1.0 - k) + k;
    num / denom
}

fn geometry_smith(normal: Vec3f, wo: Vec3f, wi: Vec3f, roughness: f32) -> f32 {
    let ndotv = f32::max(Vec3f::dot(normal, wo), 0.0);
    let ndotl = f32::max(Vec3f::dot(normal, wi), 0.0);
    let ggx2 = geometry_schlick_ggx(ndotv, roughness);
    let ggx1 = geometry_schlick_ggx(ndotl, roughness);
    ggx1 * ggx2
}

impl BSDF for Material {
    fn sample(&self, normal: Vec3f, wo: Vec3f) -> (Vec3f, Vec3f) {
        match self.material {
            MaterialType::Mirror => (reflect(-wo, normal), self.albedo),
            MaterialType::Transparent => {
                
                let mut rng = rand::thread_rng();
                let r1 = rng.gen_range(0.0..1.0);

                let fr = fresnel(-wo, normal, self.ior).clamp(0.0, 1.0);
                //let fr = 1.0;
                
                if r1 <= fr {
                    let wi = refract(-wo, normal, self.ior);
                    (wi, self.albedo * (fr))
                } else {
                    let wi = reflect(-wo, normal);
                    (wi, self.albedo * (1.0 - fr))
                }
            }
            MaterialType::CosineWeighted => {
                // Cosine-weighted hemisphere sampling
                let wi = Onb::local_to_world(normal, cosine_weighted_hemisphere());
                let cos_theta = Vec3f::dot(normal, wi);
                let pdf = cos_theta / PI;
                let brdf = self.albedo / PI;
                (wi, brdf * cos_theta / pdf)
            }
            MaterialType::Uniform => {
                // Uniform hemisphere sampling
                let pdf = 1.0 / (2.0 * PI);
                let wi = Onb::local_to_world(normal, uniform_hemisphere());
                let cos_theta = Vec3f::dot(normal, wi);
                let brdf = self.albedo / PI;
                (wi, brdf * cos_theta / pdf)
            }
            MaterialType::CookTorrance => {
                // Cook-Torrance Reflection Model

                // let wi = Onb::local_to_world(normal, uniform_hemisphere());
                // let cos_theta = Vec3f::dot(normal, wi);
                // let pdf = 1.0 / (2.0 * PI);

                let wi = Onb::local_to_world(normal, cosine_weighted_hemisphere());
                let cos_theta = Vec3f::dot(normal, wi);
                let pdf = cos_theta / PI;

                let f0 = Vec3::lerp(Vec3::from(0.04), self.albedo, self.metallic);

                // Schlick's fresnel approximation
                let fresnel = fresnel_schlick(f0, cos_theta);

                // Halfway vector between wo and wi
                let halfway = Vec3::normalize(wo + wi);

                // Normal Distribution Function
                let distribution = distribution_ggx(normal, halfway, self.roughness);

                // Geometry Function
                let geometry = geometry_smith(normal, wo, wi, self.roughness);

                // DFG / (4 dot(wo, n) dot(wi, n))
                let brdf =
                    (fresnel * distribution * geometry) / (4.0 * cos_theta * Vec3::dot(normal, wo));

                (wi, brdf * cos_theta / pdf)
            }
        }
    }
}
