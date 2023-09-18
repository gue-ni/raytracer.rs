use crate::vector::Vec3;

pub struct HitRecord {
    pub t: f32,
    pub normal: Vec3,
    pub point: Vec3,
    pub idx: usize,
}

impl HitRecord {
    pub fn new() -> Self {
        HitRecord {
            t: f32::INFINITY,
            normal: Vec3::zero(),
            point: Vec3::zero(),
            idx: 0,
        }
    }
}
