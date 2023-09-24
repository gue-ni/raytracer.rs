extern crate rand;
use rand::Rng;

#[allow(unused_imports)]
use std::f32::consts::PI;

use image::RgbImage;

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

pub fn distribution_ggx(n: Vec3f, h: Vec3f, a: f32) -> f32 {
    let a2 = a * a;
    let ndoth = f32::max(Vec3f::dot(n, h), 0.0);
    let ndoth2 = ndoth * ndoth;
    let nom = a2;
    let mut denom = ndoth2 * (a2 - 1.0) + 1.0;
    denom = PI * denom * denom;
    nom / denom
}
pub fn geometry_schlick_ggx(ndotv: f32, roughness: f32) -> f32 {
    let r = roughness + 1.0;
    let k = (r * r) / 8.0;
    let num = ndotv;
    let denom = ndotv * (1.0 - k) + k;
    num / denom
}

pub fn geometry_smith(n: Vec3f, v: Vec3f, l: Vec3f, roughness: f32) -> f32 {
    let ndotv = f32::max(Vec3f::dot(n, v), 0.0);
    let ndotl = f32::max(Vec3f::dot(n, l), 0.0);
    let ggx2 = geometry_schlick_ggx(ndotv, roughness);
    let ggx1 = geometry_schlick_ggx(ndotl, roughness);
    ggx1 * ggx2
}

pub fn fresnel_schlick(cos_theta: f32, f0: Vec3f) -> Vec3f {
    f0 + (Vec3f::fill(1.0) - f0) * f32::powf((1.0 - cos_theta).clamp(0.0, 1.0), 5.0)
}

/// Returns vector based on spherical coordinates
pub fn from_spherical(theta: f32, phi: f32) -> Vec3f {
    let sin_phi = phi.sin();
    let cos_phi = phi.cos();
    let sin_theta = theta.sin();
    let cos_theta = theta.cos();
    Vec3f::new(cos_phi * sin_theta, cos_theta, sin_phi * sin_theta)
}

/// Uniform sample from hemisphere
pub fn uniform_hemisphere() -> Vec3f {
    let mut rng = rand::thread_rng();
    let r1 = rng.gen_range(0.0..1.0);
    let r2 = rng.gen_range(0.0..1.0);

    let phi = 2.0 * PI * r1;
    let theta = f32::acos(r2);

    Vec3f::normalize(from_spherical(theta, phi))
}

pub fn cosine_weighted_hemisphere() -> Vec3f {
    let mut rng = rand::thread_rng();

    let r1 = rng.gen_range(0.0..1.0);
    let r2 = rng.gen_range(0.0..1.0);

    let phi = 2.0 * PI * r1;
    let theta = f32::acos(f32::sqrt(r2));
    Vec3f::normalize(from_spherical(theta, phi))
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

pub fn to_image(framebuffer: Vec<Vec3f>, width: u32, height: u32) -> RgbImage {
    let scale = u8::MAX as f32;

    let buffer: Vec<u8> = framebuffer
        .iter()
        .flat_map(|&pixel| [pixel.x, pixel.y, pixel.z])
        .map(|value| {
            // gamma correction
            // (value * scale) as u8
            (value.sqrt() * scale) as u8
        })
        .collect();

    RgbImage::from_vec(width, height, buffer).unwrap()
}

#[cfg(test)]
mod test {
    use crate::common::*;
    use crate::onb::*;

    #[test]
    fn test_reflect() {
        let normal = Vec3f::new(0.0, 1.0, 0.0);
        let incoming = Vec3::normalize(Vec3f::new(1.0, -1.0, 0.0));
        let outgoing = reflect(incoming, normal);
        assert_eq!(Vec3f::dot(incoming, outgoing), 0.0); // right angle
        assert_eq!(outgoing, Vec3::normalize(Vec3f::new(1.0, 1.0, 0.0)));
    }

    fn create_image_from_distribution(
        width: usize,
        height: usize,
        sample_hemisphere: impl Fn() -> Vec3f,
    ) -> RgbImage {
        let mut buffer = vec![Vec3f::fill(0.0); (width * height) as usize];

        let samples = 5000;
        for _ in 0..samples {
            let vec = sample_hemisphere();
            let sample = (vec + 1.0) / 2.0;
            //println!("{:?}, {:?}", vec, sample);

            let w = width as f32;
            let h = height as f32;

            let x = (sample.x * w) as usize;
            let y = (sample.z * h) as usize;
            //println!("{}, {}", x, y);
            assert!(x < width && y < height);

            let index = (y * width + x) as usize;
            buffer[index] = Vec3::lerp(Vec3::new(0.0, 0.0, 1.0), Vec3::new(1.0, 0.0, 0.0), vec.y);
        }

        to_image(buffer, width as u32, height as u32)
    }

    #[test]
    fn test_cosine() {
        let image = create_image_from_distribution(200, 200, || cosine_weighted_hemisphere());
        let _ = image.save("cosine.png");
    }

    #[test]
    fn test_uniform_hemisphere() {
        let image = create_image_from_distribution(200, 200, || uniform_hemisphere());
        let _ = image.save("uniform.png");
    }

    #[test]
    fn test_onb() {
        let image = create_image_from_distribution(200, 200, || {
            let normal = Vec3::new(1.0, 0.0, 0.0);
            let sample = uniform_sample_hemisphere(normal);

            //println!(" {:?}", sample);

            let w = Vec3::new(0.0, 1.0, 0.0);

            let onb = Onb::new(w);
            onb.transform(sample)
            //sample
        });
        let _ = image.save("onb.png");
    }

    #[test]
    fn test_onb_basic() {
        let w = Vec3::new(0.0, 1.0, 0.0);
        let v = Vec3::new(1.0, 0.0, 0.0);
        assert_eq!(Onb::local_to_world(w, v), Vec3::new(0.0, 1.0, 0.0));
    }
}
