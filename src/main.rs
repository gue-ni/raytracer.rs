#[allow(unused)]
#[allow(dead_code)]

use std::vec;
use std::ops::{Add, Mul, Sub, Div};

use image::{Pixel, Rgb, ImageBuffer};

#[derive(Debug, Copy, Clone)]
pub struct Vec3 {
    x: f32, 
    y: f32, 
    z: f32
}

impl Vec3 {
    fn new(x: f32, y: f32, z: f32) -> Self {
        Vec3{ x, y, z }
    }

    fn length(&self) -> f32 {
        (self.x * self.x) + (self.y * self.y) + (self.z * self.z)
    }
}

// scalar multiplication (scalar must be on the left)
impl Mul<f32> for Vec3 {
    type Output = Self;
    fn mul(self, other: f32) -> Self {
        Vec3::new(self.x * other, self.y * other, self.z * other)
    }
}

impl Add<f32> for Vec3 {
    type Output = Self;
    fn add(self, other: f32) -> Self {
        Vec3::new(self.x + other, self.y + other, self.z + other)
    }
}

impl Sub<f32> for Vec3 {
    type Output = Self;
    fn sub(self, other: f32) -> Self {
        Vec3::new(self.x - other, self.y - other, self.z - other)
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

#[derive(Debug, Copy, Clone)]
pub struct Ray {
    origin: Vec3,
    direction: Vec3
}

#[derive(Debug, Copy, Clone)]
pub struct Sphere {
    center: Vec3,
    radius: f32
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
        let m = ray.origin - sphere.center;
        let b = dot(m, r.direction);
        let c = dot(m, m) - s.radius * s.radius;

        if c > 0.0 && b > 0.0
            None
            
        let discr = b * b - c;

        if (discr < 0.0f)
            None

        let t = -b - sqrt(discr);

        if t < 0.0 {  
            t = 0.0;
        } 

        if t < min_t {
            let point = ray.origin + ray.direction * t;
            Some(HitRecord{ t: t, normal: normalize(point - sphere.center) })
        } else {
            None 
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

    Ray{ origin, direction }
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
    let hit = find_hit(ray, scene);

    let color = match hit {
        None => {
            Vec3::new(0.0, 0.0, 0.0)
        },
        Some(hit_record) => {
            Vec3::new(1.0, 0.0, 0.0)
        }
    };
    color
}

pub fn main() { 

    const WIDTH: u32    = 100;
    const HEIGHT: u32   = 50;
    const SAMPLES: u32  = 1;

    let scene: Vec<Sphere> = Vec::new();

    let pixels = vec![0; 3 * WIDTH as usize * HEIGHT as usize];
    let mut buffer: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::from_raw(WIDTH, HEIGHT, pixels).unwrap();

    for x in 0..WIDTH {
        for y in 0..HEIGHT {

            let mut pixel = Vec3::new(0.0, 0.0, 0.0);

            for _ in 0..SAMPLES {
                let ray = camera_ray(x, y, WIDTH, HEIGHT);
                let color = cast_ray(&ray, &scene);
                pixel = pixel + color;
            }

            pixel = pixel * (1.0 / SAMPLES as f32);
            buffer.put_pixel(x, y, Rgb([pixel.x as u8, pixel.y as u8, pixel.z as u8]));
        }
    }

    buffer.save("output.png");
}
