extern crate rand;
use rand::Rng;
use std::f32::consts::PI;

use crate::vector::*;

pub struct HitRecord {
    pub t: f32,
    pub normal: Vec3f,
    pub point: Vec3f,
    pub idx: usize,
}

impl HitRecord {
    pub fn new() -> Self {
        HitRecord {
            t: f32::INFINITY,
            normal: Vec3f::fill(0.0),
            point: Vec3f::fill(0.0),
            idx: 0,
        }
    }
}

#[allow(dead_code)]
pub fn reflect(incoming: Vec3f, normal: Vec3f) -> Vec3f {
    incoming - normal * 2.0 * Vec3f::dot(incoming, normal)
}

// https://www.scratchapixel.com/lessons/3d-basic-rendering/introduction-to-shading/reflection-refraction-fresnel.html
#[allow(dead_code)]
pub fn refract(incoming: Vec3f, normal: Vec3f, ior: f32) -> Vec3f {
    let mut cosi = Vec3f::dot(incoming, normal).clamp(-1.0, 1.0);
    let mut etai = 1.0;
    let mut etat = ior;
    let mut n = normal;

    if cosi < 0.0 {
        cosi = -cosi;
    } else {
        let tmp = etai;
        etai = etat;
        etat = tmp;
        n = -normal;
    }

    let eta = etai / etat;
    let k = 1.0 - eta * eta * (1.0 - cosi * cosi);

    if k < 0.0 {
        Vec3f::fill(0.0)
    } else {
        incoming * eta + n * (eta * cosi - k.sqrt())
    }
}

/*
pub fn DistributionGGX(N: Vec3f, H: Vec3f, a: f32) -> f32 {
    let a2     = a*a;
    let NdotH  = f32::max(Vec3f::dot(N, H), 0.0);
    let NdotH2 = NdotH*NdotH;
    let nom    = a2;
    let mut denom  = (NdotH2 * (a2 - 1.0) + 1.0);
    denom        = PI * denom * denom;
    nom / denom
}

pub fn GeometrySchlickGGX(NdotV: f32, roughness: f32) -> f32 {
    let r = (roughness + 1.0);
    let k = (r*r) / 8.0;
    let num   = NdotV;
    let denom = NdotV * (1.0 - k) + k;
    num / denom
}

pub fn GeometrySmith(N: Vec3f, V: Vec3f, L: Vec3f, roughness: f32) -> f32 {
    let NdotV = f32::max(Vec3f::dot(N, V), 0.0);
    let NdotL = f32::max(Vec3f::dot(N, L), 0.0);
    let ggx2  = GeometrySchlickGGX(NdotV, roughness);
    let ggx1  = GeometrySchlickGGX(NdotL, roughness);
    ggx1 * ggx2
}

pub fn fresnelSchlick(cosTheta: f32, F0: Vec3f) -> Vec3f
{
    F0 + (Vec3f::fill(1.0) - F0) * f32::powf((1.0 - cosTheta).clamp(0.0, 1.0), 5.0)
}
*/

fn vector_on_sphere() -> Vec3f {
    let r = 1.0;
    let mut rng = rand::thread_rng();
    Vec3f::normalize(Vec3f::new(
        rng.gen_range(-r..r),
        rng.gen_range(-r..r),
        rng.gen_range(-r..r),
    ))
}

/*
pub fn sample_hemisphere() -> Vec3f {
    let mut rng = rand::thread_rng();
    let x1 = rng.get_range(0.0..1.0);
    let x2 = rng.get_range(0.0..1.0);
    let phi = 2.0 * PI * x2;
    let cos_theta = x1;
    let sin_theta = f32::sqrt(1.0 - (cos_theta * cos_theta));
    let cos_phi = f32::cos(phi);
    let sin_phi = f32::sin(phi);
    Vec3f::new(cos_phi * sin_theta, sin_phi * sin_theta, cos_theta)
}
*/

pub fn vector_in_hemisphere(normal: Vec3f) -> (Vec3f, f32) {
    let mut vec: Vec3f;
    loop {
        vec = vector_on_sphere();
        if Vec3f::dot(vec, normal) > 0.0 {
            break;
        }
    }
    let prob = 1.0 / (2.0 * PI);
    (vec, prob)
}

pub fn uniform_sample_hemisphere(normal: Vec3f) -> Vec3f {
    loop {
        let omega = vector_on_sphere();
        if Vec3f::dot(omega, normal) > 0.0 {
            break omega;
        }
    }
}
