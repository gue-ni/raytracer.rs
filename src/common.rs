use crate::vector::Vec3f;

pub struct HitRecord {
    pub t: f32,
    pub normal: Vec3f,
    pub point: Vec3f,
    pub idx: usize,
}

impl HitRecord {
    pub fn new() -> Self {
        HitRecord {
            t: f32::INFINITY,
            normal: Vec3f::fill(0.0),
            point: Vec3f::fill(0.0),
            idx: 0,
        }
    }
}
