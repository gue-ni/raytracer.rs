use crate::ray::*;
use crate::vector::*;
use serde::{Deserialize, Serialize};

use rand::Rng;

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct Camera {
    pub position: Vec3f,
    pub target: Vec3f,
    pub fov: f64,

    #[serde(skip)]
    aspect_ratio: f64,

    #[serde(skip)]
    pub focal_length: f64,

    #[serde(skip)]
    pub aperture: f64,

    #[serde(skip)]
    pub resolution: Vec2f,
}

impl Camera {
    pub fn new(position: Vec3f, target: Vec3f, fov: f64, res: (u32, u32)) -> Self {
        Camera {
            position,
            target,
            fov,
            resolution: Vec2f::from(res),
            aspect_ratio: (res.1 as f64) / (res.0 as f64),
            focal_length: 3.0,
            aperture: 0.001,
        }
    }

    pub fn get_ray(&self, pixel: (u32, u32)) -> Ray {
        let coord = Vec2f::from(pixel) / self.resolution;

        let forward = (self.target - self.position).normalize();
        let right = Vec3::cross(forward, Vec3::new(0.0, 1.0, 0.0)).normalize();
        let up = Vec3::cross(right, forward).normalize();

        let half_width = f64::tan(self.fov / 2.0);
        let half_height = half_width * self.aspect_ratio;

        let height = 2.0 * half_height;
        let width = 2.0 * half_width;

        //xIncVector = (U*2*halfWidth)/xResolution;
        //yIncVector = (V*2*halfHeight)/yResolution;

        let bottom_left = self.target - (right * half_width) - (up * half_height);

        let view_point = bottom_left + (right * width * coord.x) + (up * height * coord.y);

        Ray::new(self.position, (view_point - self.position).normalize())
    }

    pub fn ray_without_dof(&self, pixel: (u32, u32)) -> Ray {
        let mut rng = rand::thread_rng();
        let r1 = rng.gen_range(-1.0..1.0);
        let r2 = rng.gen_range(-1.0..1.0);

        let uv = (Vec2f::from(pixel) - self.resolution * 0.5) / self.resolution.y;
        let origin = self.position + Vec3f::new(r1, 0.0, r2) * 0.001;

        let target = Vec3::new(uv.x, uv.y, 1.0);
        let direction = Vec3::normalize(target - origin);

        Ray::new(origin, direction)
    }

    pub fn ray(&self, pixel: (u32, u32)) -> Ray {
        let ray = self.ray_without_dof(pixel);
        let focal_point = ray.point_at(self.focal_length);

        let mut rng = rand::thread_rng();

        let jitter =
            Vec3::new(rng.gen_range(-0.5..0.5), rng.gen_range(-0.5..0.5), 0.0) * self.aperture;

        let origin = self.position + jitter;
        let direction = Vec3::normalize(focal_point - origin);

        Ray::new(origin, direction)
    }
}

#[cfg(test)]
mod test {
    use crate::camera::*;

    #[ignore]
    #[test]
    fn test_deserialize() {
        let json = r#"{ "position": [0.0, 1.0, 0.0] }"#;
        let camera: Camera = serde_json::from_str(json).unwrap();
        assert_eq!(camera.position, Vec3::new(0.0, 1.0, 0.0));
    }

    #[test]
    fn test_get_ray() {
        let eye = Vec3::new(0.0, 1.0, 0.0);
        let target = Vec3::new(0.0, 0.0, 5.0);
        let resolution: (u32, u32) = (512, 512);
        let fov = 45.0;

        let camera = Camera::new(eye, target, fov, resolution);

        let pixel = (256, 256);

        let ray = camera.get_ray(pixel);

        println!("{:?}", ray);
    }
}
