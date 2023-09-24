use crate::vector::*;

#[derive(Debug, Copy, Clone)]
pub struct Ray {
    pub origin: Vec3f,
    pub direction: Vec3f,
}

impl Ray {
    pub fn new(origin: Vec3f, direction: Vec3f) -> Self {
        Ray { origin, direction }
    }

    pub fn point_at(&self, t: f32) -> Vec3f {
        self.origin + self.direction * t
    }

    pub fn towards(origin: Vec3f, target: Vec3f) -> Self {
        Ray::new(origin, Vec3::normalize(target - origin))
    }
}
