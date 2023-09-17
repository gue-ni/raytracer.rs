#[cfg(test)]
mod test {
    use crate::vector::*;
    use crate::ray::*;
    use crate::geometry::*;

    #[test]
    fn test_sphere_hit() {
        let sphere = Sphere::new(Vec3::new(0.0, 0.0, 5.0), 1.0);
        let ray = Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0));
        let hit = sphere.hit(&ray, 0.0, f32::INFINITY).unwrap();
        assert_eq!(hit.t, 4.0);
        assert_eq!(hit.point, Vec3::new(0.0, 0.0, 4.0));
        assert_eq!(hit.normal, Vec3::new(0.0, 0.0, -1.0));
    }
}
