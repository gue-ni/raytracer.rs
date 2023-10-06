use crate::ray::*;
use crate::vector::*;
use serde::{Deserialize, Serialize};

use rand::Rng;

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct Camera {
    pub position: Vec3f,

    #[serde(skip)]
    pub resolution: Vec2f,

    #[serde(skip)]
    pub focal_length: f64,

    #[serde(skip)]
    pub aperture: f64
}

impl Camera {
    pub fn new(position: Vec3f, res: (u32, u32)) -> Self {
        Camera {
            position,
            resolution: Vec2f::from(res),
            focal_length: 1.0,
            aperture: 0.005
        }
    }

    pub fn ray_without_dof(&self, pixel: (u32, u32)) -> Ray {
        let mut rng = rand::thread_rng();
        let r1 = rng.gen_range(-1.0..1.0);
        let r2 = rng.gen_range(-1.0..1.0);

        let uv = (Vec2f::from(pixel) - self.resolution * 0.5) / self.resolution.y;
        let origin = self.position + Vec3f::new(r1, 0.0, r2) * 0.001;
        let target = Vec3f::new(uv.x, uv.y, 1.0);
        Ray::new(origin, Vec3f::normalize(target - origin))
    }

    pub fn ray(&self, pixel: (u32, u32)) -> Ray {
        let ray = self.ray_without_dof(pixel);
        let focal_point = ray.point_at(self.focal_length);
        
        let mut rng = rand::thread_rng();

        let jitter = Vec3::new(rng.gen_range(-0.5..0.5), rng.gen_range(-0.5..0.5), 0.0) * self.aperture;
                
        let origin = self.position + jitter;
        let direction = Vec3::normalize(focal_point - origin);

        Ray::new(origin, direction)
    }
}

#[cfg(test)]
mod test {
    use crate::camera::*;

    #[test]
    fn test_deserialize() {
        let json = r#"{ "position": [0.0, 1.0, 0.0] }"#;
        let camera: Camera = serde_json::from_str(json).unwrap();
        assert_eq!(camera.position, Vec3::new(0.0, 1.0, 0.0));
    }
}
