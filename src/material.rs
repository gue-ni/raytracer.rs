use crate::common::*;
use crate::onb::Onb;
use crate::vector::*;
use std::f32::consts::PI;

/// Bidirectional Scattering Distribution Function (BSDF)
pub trait BRDF {
    /// Returns outgoing vector and brdf multiplier
    /// 'normal' - Normal vector at hit point
    /// 'wo' - Direction vector toward camera
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

    /// Cook-Torrance Reflection Model
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
    pub ior: f32,
    pub metalic: f32,
    /// Index of Refraction
    pub material: MaterialType,
}

impl Material {
    pub fn diffuse(color: Vec3f) -> Self {
        Material {
            albedo: color,
            emittance: 0.0,
            roughness: 0.0,
            ior: 0.0,
            metalic: 0.0,
            material: MaterialType::CosineWeighted,
        }
    }

    pub fn emissive(color: Vec3f, intensity: f32) -> Self {
        Material {
            albedo: color,
            emittance: intensity,
            roughness: 1.0,
            ior: 0.0,
            metalic: 0.0,
            material: MaterialType::CosineWeighted,
        }
    }

    pub fn physical(color: Vec3f, roughness: f32, metalic: f32) -> Self {
        Material {
            albedo: color,
            roughness,
            metalic,
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
            metalic: 0.0,
            material: MaterialType::Mirror,
        }
    }

    pub fn transparent(color: Vec3f) -> Self {
        Material {
            albedo: color,
            emittance: 0.0,
            roughness: 0.0,
            ior: 1.0,
            metalic: 0.0,
            material: MaterialType::Transparent,
        }
    }
}

/// Schlick's Fresnel Approximation
fn fresnel_schlick(cos_theta: f32, f0: Vec3f) -> Vec3f {
    f0 + (Vec3f::fill(1.0) - f0) * f32::powf((1.0 - cos_theta).clamp(0.0, 1.0), 5.0)
}

fn distribution_ggx(normal: Vec3f, halfway: Vec3f, roughness: f32) -> f32 {
    let a2 = roughness * roughness;
    let ndoth = f32::max(Vec3f::dot(normal, halfway), 0.0);
    let ndoth2 = ndoth * ndoth;
    let nom = a2;
    let denom = ndoth2 * (a2 - 1.0) + 1.0;
    //denom = PI * denom * denom;
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

impl BRDF for Material {
    fn sample(&self, normal: Vec3f, wo: Vec3f) -> (Vec3f, Vec3f) {
        match self.material {
            MaterialType::Mirror => {
                let wi = reflect(-wo, normal);
                let bsdf = self.albedo;
                (wi, bsdf)
            }
            MaterialType::Transparent => {
                // TODO
                let wi = refract(-wo, normal, self.ior);
                let bsdf = self.albedo;
                (wi, bsdf)
            }
            MaterialType::CosineWeighted => {
                // Cosine-weighted hemisphere sampling
                // brdf = albedo / 𝜋
                // pdf = cos(θ) / 𝜋
                let wi = Onb::local_to_world(normal, cosine_weighted_hemisphere());
                let cos_theta = Vec3f::dot(normal, wi);
                let pdf = cos_theta / PI;
                let bsdf = self.albedo / PI;
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
                // Cook-Torrance Reflection Model

                //let wi = Onb::local_to_world(normal, uniform_hemisphere());
                //let cos_theta = Vec3f::dot(normal, wi);
                //let pdf = 1.0 / (2.0 * PI);

                let wi = Onb::local_to_world(normal, cosine_weighted_hemisphere());
                let cos_theta = Vec3f::dot(normal, wi);
                let pdf = cos_theta / PI;

                //
                let specular_color = Vec3::lerp(Vec3::from(0.04), self.albedo, self.metalic);

                // Schlick's fresnel approximation
                let fresnel = fresnel_schlick(cos_theta, specular_color);

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
