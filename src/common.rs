use image::RgbImage;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::f64::consts::PI;

use crate::camera::*;
use crate::geometry::*;
use crate::vector::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigFile {
    pub scene: Scene,
    pub camera: Camera,
}

#[allow(dead_code)]
pub fn reflect(incident: Vec3f, normal: Vec3f) -> Vec3f {
    incident - normal * 2.0 * Vec3f::dot(incident, normal)
}

// https://www.scratchapixel.com/lessons/3d-basic-rendering/introduction-to-shading/reflection-refraction-fresnel.html
#[allow(dead_code)]
pub fn refract(incident: Vec3f, normal: Vec3f, ior: f64) -> Vec3f {
    let mut cosi = Vec3f::dot(incident, normal);
    let mut etai = 1.0;
    let mut etat = ior;
    let mut n = normal;

    if cosi < 0.0 {
        cosi = -cosi;
    } else {
        (etai, etat) = (etat, etai);
        n = -normal;
    }

    let eta = etai / etat;
    let k = 1.0 - eta * eta * (1.0 - cosi * cosi);

    if k < 0.0 {
        // total internal reflection, no refraction
        Vec3f::from(0.0)
    } else {
        incident * eta + n * (eta * cosi - k.sqrt())
    }
}

// https://registry.khronos.org/OpenGL-Refpages/gl4/html/refract.xhtml
#[allow(dead_code)]
pub fn refract_glsl(incident: Vec3f, normal: Vec3f, eta: f64) -> Vec3f {
    let cos_incident = Vec3::dot(normal, incident);
    let k = 1.0 - eta * eta * (1.0 - cos_incident * cos_incident);
    if k < 0.0 {
        // total internal reflection, no refraction
        Vec3::from(0.0)
    } else {
        incident * eta - normal * (eta * cos_incident + k.sqrt())
    }
}

///
pub fn fresnel(incident: Vec3f, normal: Vec3f, ior: f64) -> f64 {
    let mut cosi = Vec3f::dot(incident, normal);
    let etai = 1.0;
    let etat = ior;

    let sint = etai / etat * f64::sqrt(f64::max(0.0, 1.0 - cosi * cosi));

    let kr = if sint >= 1.0 {
        // Total internal reflection
        1.0
    } else {
        let cost = f64::sqrt(f64::max(0.0, 1.0 - sint * sint));
        cosi = cosi.abs();
        let rs = ((etat * cosi) - (etai * cost)) / ((etat * cosi) + (etai * cost));
        let rp = ((etai * cosi) - (etat * cost)) / ((etai * cosi) + (etat * cost));
        (rs * rs + rp * rp) / 2.0
    };

    1.0 - kr
}

/// Returns vector based on spherical coordinates
/// But: in our coordinate system, y is up
pub fn from_spherical(theta: f64, phi: f64) -> Vec3f {
    let sin_phi = phi.sin();
    let cos_phi = phi.cos();
    let sin_theta = theta.sin();
    let cos_theta = theta.cos();
    Vec3f::new(cos_phi * sin_theta, sin_phi * sin_theta, cos_theta)
}

/// Uniform sample from hemisphere
pub fn uniform_hemisphere() -> Vec3f {
    let mut rng = rand::thread_rng();
    let r1 = rng.gen_range(0.0..1.0);
    let r2 = rng.gen_range(0.0..1.0);
    let phi = 2.0 * PI * r1;
    let theta = f64::acos(r2);
    Vec3f::normalize(from_spherical(theta, phi))
}

/// Cosine weighted sample from hemisphere
pub fn cosine_weighted_hemisphere() -> Vec3f {
    let mut rng = rand::thread_rng();
    let r1 = rng.gen_range(0.0..1.0);
    let r2 = rng.gen_range(0.0..1.0);
    let phi = 2.0 * PI * r1;
    let theta = f64::acos(f64::sqrt(r2));
    Vec3f::normalize(from_spherical(theta, phi))
}

// https://agraphicsguy.wordpress.com/2015/11/01/sampling-microfacet-brdf/
// https://computergraphics.stackexchange.com/questions/4979/what-is-importance-sampling
// https://schuttejoe.github.io/post/ggximportancesamplingpart1/
pub fn ggx_hemisphere(roughness: f64) -> Vec3f {
    let mut rng = rand::thread_rng();
    let r1 = rng.gen_range(0.0..1.0);
    let r2 = rng.gen_range(0.0..1.0);
    let phi = 2.0 * PI * r1;
    let a2 = roughness * roughness;
    let theta = f64::acos(f64::sqrt(a2 / (r2 * (a2 - 1.0) + 1.0)));
    Vec3f::normalize(from_spherical(theta, phi))
}

pub fn vector_on_sphere() -> Vec3f {
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
    Vec3f::new(r as f64, g as f64, b as f64) / (u8::MAX as f64)
}

pub fn to_image(framebuffer: Vec<Vec3f>, width: u32, height: u32) -> RgbImage {
    let scale = u8::MAX as f64;

    let buffer: Vec<u8> = framebuffer
        .iter()
        .flat_map(|&pixel| [pixel.x, pixel.y, pixel.z])
        .map(|value| (value.sqrt() * scale) as u8)
        .collect();

    RgbImage::from_vec(width, height, buffer).unwrap()
}

#[cfg(test)]
mod test {
    use crate::common::*;
    use crate::onb::*;
    use std::fs;

    fn create_image_from_distribution(
        width: usize,
        height: usize,
        sample_hemisphere: impl Fn() -> Vec3f,
    ) -> RgbImage {
        let mut buffer = vec![Vec3f::from(0.0); (width * height) as usize];

        let samples = 10000;
        for _ in 0..samples {
            let vec = sample_hemisphere();
            let sample = (vec + 1.0) / 2.0;

            let w = width as f64;
            let h = height as f64;

            let x = (sample.x.clamp(0.0, 0.999) * w) as usize;
            let y = (sample.y.clamp(0.0, 0.999) * h) as usize;
            assert!(x < width && y < height);

            let index = (y * width + x) as usize;
            let blue = Vec3::new(0.0, 0.0, 1.0);
            let red = Vec3::new(1.0, 0.0, 0.0);
            buffer[index] = Vec3::lerp(blue, red, vec.z.clamp(0.0, 1.0));
        }

        to_image(buffer, width as u32, height as u32)
    }

    #[test]
    fn test_reflect() {
        {
            let normal = Vec3f::new(0.0, 1.0, 0.0);
            let incident = Vec3::normalize(Vec3f::new(1.0, -1.0, 0.0));
            let outgoing = reflect(incident, normal);
            assert_eq!(Vec3::dot(incident, normal), Vec3::dot(outgoing, normal));
            assert_eq!(Vec3f::dot(incident, outgoing), 0.0);
            assert_eq!(outgoing, Vec3::normalize(Vec3f::new(1.0, 1.0, 0.0)));
        }
    }

    #[test]
    fn test_refract() {
        {
            let ior = 1.5; // glass
            let etai = 1.0; // air
            let etat = ior;
            let eta = etai / etat; // going from air into glass

            let normal = Vec3f::new(0.0, 1.0, 0.0);
            let incident = Vec3::normalize(Vec3f::new(1.0, -1.0, 0.0));

            let r1 = refract_glsl(incident, normal, eta);
            let r2 = refract(incident, normal, ior);

            // Compare with value from glm implementation
            assert_eq!(r1, Vec3f::new(0.47140452, -0.8819171, 0.0));
            assert_eq!(r1, r2);
        }
    }

    #[test]
    fn test_fresnel() {}

    #[test]
    fn test_cosine() {
        let image = create_image_from_distribution(200, 200, || cosine_weighted_hemisphere());
        let _ = image.save("renders/cosine.png");
    }

    #[test]
    fn test_uniform_hemisphere() {
        let image = create_image_from_distribution(200, 200, || uniform_hemisphere());
        let _ = image.save("renders/uniform_v1.png");
    }

    #[test]
    fn test_ggx_hemisphere() {
        {
            let alpha = 0.0;
            let image = create_image_from_distribution(200, 200, || ggx_hemisphere(alpha));
            let _ = image.save("renders/ggx_00.png");
        }
        {
            let alpha = 0.2;
            let image = create_image_from_distribution(200, 200, || ggx_hemisphere(alpha));
            let _ = image.save("renders/ggx_02.png");
        }

        {
            let alpha = 0.5;
            let image = create_image_from_distribution(200, 200, || ggx_hemisphere(alpha));
            let _ = image.save("renders/ggx_05.png");
        }
        {
            let alpha = 1.0;
            let image = create_image_from_distribution(200, 200, || ggx_hemisphere(alpha));
            let _ = image.save("renders/ggx_10.png");
        }
    }

    #[test]
    #[ignore]
    fn test_uniform_hemisphere_2() {
        let normal = Vec3::new(0.0, 1.0, 0.0);
        let image = create_image_from_distribution(200, 200, || uniform_sample_hemisphere(normal));
        let _ = image.save("uniform_v2.png");
    }

    #[test]
    #[ignore]
    fn test_onb() {
        let image = create_image_from_distribution(200, 200, || {
            //let sample = uniform_sample_hemisphere(normal);
            let sample = cosine_weighted_hemisphere();
            let normal = Vec3::new(0.0, 1.0, 0.0);
            Onb::local_to_world(normal, sample)
        });
        let _ = image.save("onb.png");
    }

    #[test]
    fn test_serialize() {
        let json = fs::read_to_string("scenes/sphere.json").unwrap();
        let _config: ConfigFile = serde_json::from_str(&json).unwrap();
        println!("{:?}", _config);
    }
}
