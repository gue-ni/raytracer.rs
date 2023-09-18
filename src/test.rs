#[cfg(test)]
mod test {
    use crate::geometry::*;
    use crate::ray::*;
    use crate::vector::*;

    #[test]
    fn test_sphere_hit() {
        let sphere = Sphere::new(Vec3f::new(0.0, 0.0, 5.0), 1.0);
        let ray = Ray::new(Vec3f::new(0.0, 0.0, 0.0), Vec3f::new(0.0, 0.0, 1.0));
        let hit = sphere.hit(&ray, 0.0, f32::INFINITY).unwrap();
        assert_eq!(hit.t, 4.0);
        assert_eq!(hit.point, Vec3f::new(0.0, 0.0, 4.0));
        assert_eq!(hit.normal, Vec3f::new(0.0, 0.0, -1.0));
    }

    #[test]
    fn test_triangle_hit() {
        let t = Triangle(
            Vec3f::new(-0.5, 0.0, 0.0),
            Vec3f::new(0.0, 1.0, 0.0),
            Vec3f::new(0.5, 0.0, 0.0),
        );
    }
}
