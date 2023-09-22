use crate::vector::*;

/// Orthonormal Basis
pub struct Onb {
    axis: [Vec3f; 3],
}

impl Onb {
    pub fn new(w: Vec3f) -> Self {
        let unit_w = Vec3f::normalize(w);
        let a = (unit_w.x.abs() > 0.9) ? Vec3f::new(0.0, 1.0, 0.0) : Vec3f::new(1.0, 0.0, 0.0);
        let v = Vec3f::normalize(Vec3f::cross(unit_w, a));
        let u = Vec3f::cross(unit_w, u);
        Self {
            axis: [u, v, unit_w]
        }
    }

    pub fn local(&self, a: Vec3f) -> Vec3f {
        a * self.u() + a * self.v() + a * self.w()
    }

    pub fn u(&self) -> Vec3f { self.axis[0] }
    pub fn v(&self) -> Vec3f { self.axis[1] }
    pub fn w(&self) -> Vec3f { self.axis[2] }
}