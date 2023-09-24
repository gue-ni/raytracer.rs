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
    pub metalness: f32,
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
            metalness: 0.0,
            material: MaterialType::CosineWeighted,
        }
    }

    pub fn emissive(color: Vec3f, intensity: f32) -> Self {
        Material {
            albedo: color,
            emittance: intensity,
            roughness: 1.0,
            ior: 0.0,
            metalness: 0.0,
            material: MaterialType::CosineWeighted,
        }
    }

    pub fn physical(color: Vec3f, roughness: f32, metalness: f32) -> Self {
        Material {
            albedo: color,
            roughness,
            metalness,
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
            metalness: 0.0,
            material: MaterialType::Mirror,
        }
    }

    pub fn transparent(color: Vec3f) -> Self {
        Material {
            albedo: color,
            emittance: 0.0,
            roughness: 0.0,
            ior: 1.0,
            metalness: 0.0,
            material: MaterialType::Transparent,
        }
    }
}

/// Schlick's Fresnel Approximation
fn fresnel_schlick(cos_theta: f32, f0: Vec3f) -> Vec3f {
    f0 + (Vec3f::fill(1.0) - f0) * f32::powf((1.0 - cos_theta).clamp(0.0, 1.0), 5.0)
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
                // Cook-Torrance Reflection Model
                // brdf = (D F G) / (4  wo • n wi • n))
                let wi = Onb::local_to_world(normal, uniform_hemisphere());
                let cos_theta = Vec3f::dot(normal, wi);
                let pdf = 1.0 / (2.0 * PI);

                let f0_dielectics = Vec3::from(0.04);
                let specular_color = Vec3::lerp(f0_dielectics, self.albedo, self.metalness);

                // Schlick's fresnel approximation
                let fresnel = fresnel_schlick(cos_theta, specular_color);

                // Halfway vector between wo and wi
                let halfway = Vec3::normalize(wo + wi);

                // Normal Distribution Function
                let distribution = distribution_ggx(normal, halfway, self.roughness);

                // Geometry Function
                let geometry = geometry_smith(normal, wo, wi, self.roughness);

                let brdf =
                    (fresnel * distribution * geometry) / (4.0 * cos_theta * Vec3::dot(normal, wo));
                (wi, brdf * cos_theta / pdf)
            }
        }
    }
}
