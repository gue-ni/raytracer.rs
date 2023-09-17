use crate::vector::*;
use crate::geometry::*;
use crate::common::*;

#[derive(Debug, Copy, Clone)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Ray { origin, direction }
    }

    pub fn point_at(&self, t: f32) -> Vec3 {
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
