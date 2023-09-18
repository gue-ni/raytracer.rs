use crate::common::*;
use crate::geometry::*;
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

    pub fn cast<T: Hittable>(&self, scene: &Vec<T>) -> Option<HitRecord> {
        let mut closest = HitRecord::new();
        closest.t = f32::INFINITY;

        for (i, object) in scene.iter().enumerate() {
            match object.hit(self, 0.0, closest.t) {
                None => {}
                Some(hit) => {
                    closest = hit;
                    closest.idx = i;
                }
            }
        }

        if closest.t < f32::INFINITY {
            Some(closest)
        } else {
            None
        }
    }
}
