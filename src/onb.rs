use crate::vector::*;

/// Orthonormal Bases
#[derive(Debug)]
pub struct Onb {
    axis: [Vec3f; 3],
}

impl Onb {
    /// Returns new Orthonormal Bases
    /// 'w' - normalized vector
    pub fn new(w: Vec3f) -> Self {
        // 'a' must not be parallel to 'w'
        let a = if w.x.abs() > 0.9 {
            Vec3f::new(0.0, 1.0, 0.0)
        } else {
            Vec3f::new(1.0, 0.0, 0.0)
        };

        let v = Vec3f::normalize(Vec3f::cross(w, a));
        let u = Vec3f::cross(w, v);

        Self { axis: [u, v, w] }
    }

    /// Create coordinate system around w and transform a
    pub fn local_to_world(w: Vec3f, a: Vec3f) -> Vec3f {
        let onb = Self::new(w);
        onb.transform(Vec3f::normalize(a))
    }

    pub fn transform(&self, a: Vec3f) -> Vec3f {
        self.u() * a.x + self.v() * a.y + self.w() * a.z
    }

    pub fn u(&self) -> Vec3f {
        self.axis[0]
    }
    pub fn v(&self) -> Vec3f {
        self.axis[1]
    }
    pub fn w(&self) -> Vec3f {
        self.axis[2]
    }
}

#[cfg(test)]
mod test {

    use crate::onb::*;
    use crate::vector::*;

    #[test]
    fn test_00() {
        let normal = Vec3::new(0.0, 1.0, 0.0);
        let onb = Onb::new(normal);

        // all vectors must be orthogonal
        assert_eq!(Vec3::dot(onb.v(), onb.w()), 0.0);
        assert_eq!(Vec3::dot(onb.v(), onb.u()), 0.0);
        assert_eq!(Vec3::dot(onb.w(), onb.u()), 0.0);
    }

    #[test]
    fn test_01() {
        // the vector to center the coordinate system
        let normal = Vec3::new(0.0, 1.0, 0.0);

        let onb = Onb::new(normal);
        println!("{:?}", onb);

        // the input vector, could be randomly generated in hemisphere
        let v = Vec3::new(0.0, 0.0, 1.0);

        assert_eq!(onb.transform(v), Vec3::new(0.0, 1.0, 0.0));
    }
}
