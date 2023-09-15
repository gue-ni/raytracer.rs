#[allow(unused)]
#[allow(dead_code)]

use std::vec;
use image::{Rgb, ImageBuffer};

mod la {

    use std::ops::{Add, Mul, Sub, Div};
    use image::Rgb;

    #[derive(Debug, Copy, Clone)]
    pub struct Vec3 {
        pub x: f32, 
        pub y: f32, 
        pub z: f32
    }
    
    impl Vec3 {
        pub fn new(x: f32, y: f32, z: f32) -> Self {
            Vec3{ x, y, z }
        }

        pub fn fill(v: f32) -> Self {
            Vec3::new(v, v, v)
        }
    
        pub fn length(&self) -> f32 {
            (self.x * self.x) + (self.y * self.y) + (self.z * self.z)
        }
    }
    
    // scalar multiplication (scalar must be on the left)
    impl Mul<f32> for Vec3 {
        type Output = Self;
        fn mul(self, scalar: f32) -> Self {
            Vec3::new(self.x * scalar, self.y * scalar, self.z * scalar)
        }
    }
    
    impl Div<f32> for Vec3 {
        type Output = Self;
        fn div(self, scalar: f32) -> Self {
            Vec3::new(self.x / scalar, self.y / scalar, self.z / scalar)
        }
    }
    
    impl Add<f32> for Vec3 {
        type Output = Self;
        fn add(self, scalar: f32) -> Self {
            Vec3::new(self.x + scalar, self.y + scalar, self.z + scalar)
        }
    }
    
    impl Sub<f32> for Vec3 {
        type Output = Self;
        fn sub(self, scalar: f32) -> Self {
            Vec3::new(self.x - scalar, self.y - scalar, self.z - scalar)
        }
    }
    
    // vector multiplication
    impl Mul<Vec3> for Vec3 {
        type Output = Self;
        fn mul(self, other: Vec3) -> Self {
            Vec3::new(self.x * other.x, self.y * other.y, self.z * other.z)
        }
    } 
    
    // vector addition
    impl Add<Vec3> for Vec3 {
        type Output = Self;
        fn add(self, other: Vec3) -> Self {
            Vec3::new(self.x + other.x, self.y + other.y, self.z + other.z)
        }
    } 
    
    // vector subtraction
    impl Sub<Vec3> for Vec3 {
        type Output = Self;
        fn sub(self, other: Vec3) -> Self {
            Vec3::new(self.x - other.x, self.y - other.y, self.z - other.z)
        }
    }
    
    // dot product
    pub fn dot(a: Vec3, b: Vec3) -> f32 {
        (a.x * b.x) + (a.y * b.y) + (a.z * b.z)
    }
    
    // cross product
    #[allow(dead_code)]
    pub fn cross(a: Vec3, b: Vec3) -> Vec3 {
        Vec3::new(
            a.y * b.z - a.z * b.y,
            a.z * b.x - a.x * b.z,
            a.x * b.y - a.y * b.x
        )
    }
    
    // normalize
    pub fn normalize(v: Vec3) -> Vec3 {
        v * (1.0 / v.length())
    }
}

use la::{Vec3, normalize, dot};

#[derive(Debug, Copy, Clone)]
pub struct Sphere {
    center: Vec3,
    radius: f32
}

impl Sphere {
    fn new(center: Vec3, radius: f32) -> Self {
        Sphere{ center, radius }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Ray {
    origin: Vec3,
    direction: Vec3
}

impl Ray {
    fn new(origin: Vec3, direction: Vec3) -> Self {
        Ray{ origin, direction }
    }
    
    fn point_at(&self, t: f32) -> Vec3 {
        self.origin + self.direction * t
    }

    fn cast(&self, scene: &Vec<Sphere>) -> Option<HitRecord> {
        let mut closest = HitRecord{ t: f32::MAX, normal: Vec3::fill(0.0) };
    
        for object in scene {
            match object.test(self, closest.t) {
                None => {},
                Some(hit) => {
                   closest = hit; 
                }
            }
        }
    
        if closest.t < f32::MAX {
            Some(closest)
        } else {
            None
        }
    }
}

pub struct HitRecord {
    t: f32,
    normal: Vec3
}

pub trait Hittable {
    fn test(&self, ray: &Ray, min_t: f32) -> Option<HitRecord>;
}

impl Hittable for Sphere {
    fn test(&self, ray: &Ray, min_t: f32) -> Option<HitRecord> {
        let m = ray.origin - self.center;
        let b = dot(m, ray.direction);
        let c = dot(m, m) - self.radius * self.radius;

        if c > 0.0 && b > 0.0 {
            return None;
        }
            
        let discr = b * b - c;
        if discr < 0.0 {
            return None;       
        }

        let mut t = -b - discr.sqrt();
        if t < 0.0 {  
            t = 0.0;
        } 

        if t < min_t {
            return Some(HitRecord{ t: t, normal: normalize(ray.point_at(t) - self.center) });
        } else {
            return None; 
        }
    }
}

pub fn camera_ray(x: u32, y: u32, x_res: u32, y_res: u32) -> Ray {

    let mut ray_target = Vec3::new(
        (x as f32 / x_res as f32) * 2.0 - 1.0, 
        (y as f32 / y_res as f32) * 2.0 - 1.0, 
        1.0
    );

    let aspect_ratio = x_res as f32 / y_res as f32;
    ray_target.y /= aspect_ratio;

    let origin = Vec3::new(0.0, 0.0, 0.0);
    let direction = normalize(ray_target - origin);

    Ray::new( origin, direction )
}

pub fn find_hit(ray: &Ray, scene: &Vec<Sphere>) -> Option<HitRecord> {
    let mut closest = HitRecord{ t: f32::MAX, normal: Vec3::new(0.0, 0.0, 0.0) };
    
    for object in scene {
        let hit = object.test(ray, closest.t);

        match hit {
            None => {},
            Some(hit_record) => {
               closest = hit_record; 
            }
        }
    }
    
    if closest.t < f32::MAX {
        Some(closest)
    } else {
        None
    }
}

pub fn cast_ray(ray: &Ray, scene: &Vec<Sphere>) -> Vec3 {    
    //let hit = find_hit(ray, scene);
    let hit = ray.cast(scene);
    
    let color = match hit {
        None => {
            Vec3::new(0.0, 1.0, 0.0)
        },
        Some(hit_record) => {
            hit_record.normal * 0.5 + 0.5
        }
    };
    color
}

pub fn main() { 

    const WIDTH: u32    = 640;
    const HEIGHT: u32   = 480;
    const SAMPLES: u32  = 1;

    let mut scene: Vec<Sphere> = Vec::new();
    scene.push(Sphere{ center: Vec3::new(0.0, 0.0, 3.0), radius: 1.0 });

    let pixels = vec![0; 3 * WIDTH as usize * HEIGHT as usize];
    let mut buffer: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::from_raw(WIDTH, HEIGHT, pixels).unwrap();

    for x in 0..WIDTH {
        for y in 0..HEIGHT {

            let mut pixel = Vec3::new(0.0, 0.0, 0.0);

            for _ in 0..SAMPLES {
                let ray = camera_ray(x, y, WIDTH, HEIGHT);
                pixel = pixel + cast_ray(&ray, &scene);
            }

            pixel = pixel * (u8::MAX as f32) / (SAMPLES as f32);
            buffer.put_pixel(x, y, Rgb([pixel.x as u8, pixel.y as u8, pixel.z as u8]));
        }
    }

    buffer.save("output.png");
}
