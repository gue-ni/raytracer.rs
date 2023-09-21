use crate::ray::*;
use crate::vector::*;

#[derive(Debug, Copy, Clone)]
pub struct Camera {
    pub position: Vec3f,
    pub resolution: Vec2f,
}

impl Camera {
    pub fn new(position: Vec3f, res: (u32, u32)) -> Self {
        Camera {
            position: position,
            resolution: Vec2f::from(res),
        }
    }

    pub fn ray(&self, pixel: (u32, u32)) -> Ray {
        let uv = (Vec2f::from(pixel) - self.resolution * 0.5) / self.resolution.y;
        let origin = self.position;
        let target = Vec3f::new(uv.x, uv.y, 1.0);
        Ray::new(origin, Vec3f::normalize(target - origin))
    }
}
