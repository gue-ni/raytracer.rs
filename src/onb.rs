use crate::vector::*;

/// Orthonormal Bases
#[derive(Debug)]
pub struct Onb {
    axis: [Vec3f; 3],
}

impl Onb {
    ///
    pub fn new(up: Vec3f) -> Self {
        // 'a' must not be parallel to 'up'
        let a = if up.x.abs() > 0.9 {
            Vec3f::new(0.0, 1.0, 0.0)
        } else {
            Vec3f::new(1.0, 0.0, 0.0)
        };

        let front = Vec3f::normalize(Vec3f::cross(up, a));
        let right = Vec3f::cross(up, front);

        Self {
            axis: [right, front, up],
        }
    }

    /// Create coordinate system around w and transform a
    pub fn local_to_world(w: Vec3f, a: Vec3f) -> Vec3f {
        let onb = Self::new(w);
        onb.transform(Vec3f::normalize(a))
    }

    pub fn transform(&self, a: Vec3f) -> Vec3f {
        self.right() * a.x + self.front() * a.y + self.up() * a.z
    }

    pub fn right(&self) -> Vec3f {
        self.axis[0]
    }
    pub fn front(&self) -> Vec3f {
        self.axis[1]
    }
    pub fn up(&self) -> Vec3f {
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
        assert_eq!(Vec3::dot(onb.front(), onb.up()), 0.0);
        assert_eq!(Vec3::dot(onb.front(), onb.right()), 0.0);
        assert_eq!(Vec3::dot(onb.up(), onb.right()), 0.0);
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
