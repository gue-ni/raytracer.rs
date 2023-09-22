extern crate rand;
use rand::Rng;

#[allow(unused_imports)]
use std::f32::consts::PI;

use crate::vector::*;

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
        std::mem::swap(&mut etai, &mut etat);
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

pub fn distribution_ggx(N: Vec3f, H: Vec3f, a: f32) -> f32 {
    let a2 = a * a;
    let NdotH = f32::max(Vec3f::dot(N, H), 0.0);
    let NdotH2 = NdotH * NdotH;
    let nom = a2;
    let mut denom = NdotH2 * (a2 - 1.0) + 1.0;
    denom = PI * denom * denom;
    nom / denom
}
pub fn geometry_schlick_ggx(NdotV: f32, roughness: f32) -> f32 {
    let r = roughness + 1.0;
    let k = (r * r) / 8.0;
    let num = NdotV;
    let denom = NdotV * (1.0 - k) + k;
    num / denom
}

pub fn geometry_smith(N: Vec3f, V: Vec3f, L: Vec3f, roughness: f32) -> f32 {
    let NdotV = f32::max(Vec3f::dot(N, V), 0.0);
    let NdotL = f32::max(Vec3f::dot(N, L), 0.0);
    let ggx2 = geometry_schlick_ggx(NdotV, roughness);
    let ggx1 = geometry_schlick_ggx(NdotL, roughness);
    ggx1 * ggx2
}

pub fn fresnel_schlick(cosTheta: f32, F0: Vec3f) -> Vec3f {
    F0 + (Vec3f::fill(1.0) - F0) * f32::powf((1.0 - cosTheta).clamp(0.0, 1.0), 5.0)
}

pub fn uniform_hemisphere() -> (Vec3f, f32) {
    let mut rng = rand::thread_rng();
    let x1 = rng.gen_range(0.0..1.0);
    let x2 = rng.gen_range(0.0..1.0);
    let phi = 2.0 * PI * x2;
    let cos_theta = x1;
    let sin_theta = f32::sqrt(1.0 - (cos_theta * cos_theta));
    let cos_phi = f32::cos(phi);
    let sin_phi = f32::sin(phi);
    let omega = Vec3f::new(cos_phi * sin_theta, sin_phi * sin_theta, cos_theta);
    let pdf = 1.0 / (2.0 * PI);
    (omega, pdf)
}

pub fn cosine_weighted_hemisphere() -> (Vec3f, f32) {
    let mut rng = rand::thread_rng();
    let r1 = rng.gen_range(0.0..1.0);
    let r2 = rng.gen_range(0.0..1.0);

    let _phi = 2.0 * PI * r1;
    let sin_theta = f32::sqrt(r2);
    let cos_theta = f32::sqrt(1.0 - r2);

    let omega = Vec3f::new(0.0, 0.0, 0.0);
    let pdf = cos_theta * sin_theta / PI;
    (omega, pdf)
}

fn vector_on_sphere() -> Vec3f {
    let r = 1.0;
    let mut rng = rand::thread_rng();
    Vec3f::normalize(Vec3f::new(
        rng.gen_range(-r..r),
        rng.gen_range(-r..r),
        rng.gen_range(-r..r),
    ))
}

pub fn uniform_sample_hemisphere(normal: Vec3f) -> Vec3f {
    loop {
        let omega = vector_on_sphere();
        if Vec3f::dot(omega, normal) > 0.0 {
            break omega;
        }
    }
}

pub fn from_hex(color: u32) -> Vec3f {
    assert!(color <= 0xffffff);
    let r = (color & 0xff0000) >> 16;
    let g = (color & 0x00ff00) >> 8;
    let b = color & 0x0000ff;
    Vec3f::new(r as f32, g as f32, b as f32) / (u8::MAX as f32)
}

#[cfg(test)]
mod test {
    use crate::common::*;

    #[test]
    fn test_reflect() {
        let normal = Vec3f::new(0.0, 1.0, 0.0);
        let incoming = Vec3::normalize(Vec3f::new(1.0, -1.0, 0.0));
        let outgoing = reflect(incoming, normal);
        assert_eq!(Vec3f::dot(incoming, outgoing), 0.0); // right angle
        assert_eq!(outgoing, Vec3::normalize(Vec3f::new(1.0, 1.0, 0.0)));
    }
}
