use std::ops::{ Add, Mul, Sub, Div, Neg, IndexMut, Index };

pub trait SquareRoot {
    fn sqrt(&self) -> Self;
}

impl SquareRoot for f32 {
    fn sqrt(&self) -> Self {
        (*self).sqrt()
    }
}

impl SquareRoot for f64 {
    fn sqrt(&self) -> Self {
        (*self).sqrt()
    }
}

pub trait Number: Copy + Clone + 
    Add<Output = Self> + 
    Sub<Output = Self> + 
    Mul<Output = Self> + 
    Div<Output = Self> + 
    Neg<Output = Self> + 
    SquareRoot
    {}

impl Number for f32 {}


#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec3T<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Vec3T<T> where T: Number {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }

    pub fn fill(v: T) -> Self {
        Self::new(v, v, v)
    }
}

pub trait Magnitude<T> where T: Number + SquareRoot {
    fn length(self) -> T;
}

impl<T> Magnitude<T> for Vec3T<T> where T: Number {
    fn length(self) -> T {
        (Self::dot(self, self)).sqrt()
    }
}

// vector addition
impl<T> Add for Vec3T<T> where T: Number  {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

// scalar multiplication (scalar must be on the left)
impl<T> Mul<T> for Vec3T<T> where T: Number {
    type Output = Self;
    fn mul(self, scalar: T) -> Self {
        Self::new(self.x * scalar, self.y * scalar, self.z * scalar)
    }
}

pub trait Dot<T> {
    fn dot(a: Self, b: Self) -> T;
}

impl<T> Dot<T> for Vec3T<T> where T: Number {
    fn dot(a: Self, b: Self) -> T {
        a.x * b.x + a.y * b.y + a.z * b.z
    }
}


#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vec3 { x, y, z }
    }

    pub fn zero() -> Self {
        Vec3::fill(0.0)
    }

    pub fn one() -> Self {
        Vec3::fill(1.0)
    }

    pub fn fill(v: f32) -> Self {
        Vec3::new(v, v, v)
    }

    pub fn length2(self) -> f32 {
        dot(self, self)
    }

    pub fn length(self) -> f32 {
        self.length2().sqrt()
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
impl Mul for Vec3 {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Vec3::new(self.x * other.x, self.y * other.y, self.z * other.z)
    }
}

// vector addition
impl Add for Vec3 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Vec3::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

// vector subtraction
impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Vec3::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

// vector division
impl Div for Vec3 {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        Vec3::new(self.x / other.x, self.y / other.y, self.z / other.z)
    }
}

// negation
impl Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self {
        Vec3::new(-self.x, -self.y, -self.z)
    }
}

impl Index<usize> for Vec3 {
    type Output = f32;
    fn index(&self, i: usize) -> &f32 {
        match i {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => &self.z
        }
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, i: usize) -> &mut f32 {
        match i {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => &mut self.z
        }
    }
}

// dot product
pub fn dot(a: Vec3, b: Vec3) -> f32 {
    a.x * b.x + a.y * b.y + a.z * b.z
}

// cross product
#[allow(dead_code)]
pub fn cross(a: Vec3, b: Vec3) -> Vec3 {
    Vec3::new(a.y * b.z - a.z * b.y, a.z * b.x - a.x * b.z, a.x * b.y - a.y * b.x)
}

// normalize
pub fn normalize(v: Vec3) -> Vec3 {
    v / v.length()
}

pub fn reflect(incoming: Vec3, normal: Vec3) -> Vec3 {
    incoming - normal * 2.0 * dot(incoming, normal)
}




#[cfg(test)]
mod tests {
    use crate::vector::*;

    #[test]
    fn test_length() {
        let v0 = Vec3::new(1.0, 0.0, 0.0);
        assert_eq!(v0.length(), 1.0);
    }

    #[test]
    fn test_normalize() {
        let v0 = Vec3::new(1.0, 5.0, 1.0);
        assert_eq!(normalize(v0).length(), 1.0);
    }

    #[test]
    fn test_cross() {
        let a = Vec3::new(2.0, 3.0, 4.0);
        let b = Vec3::new(5.0, 6.0, 7.0);
        assert_eq!(cross(a, b), Vec3::new(-3.0, 6.0, -3.0));
    }

    #[test]
    fn test_dot() {
        assert_eq!(dot(Vec3::new(0.0, 1.0, 0.0), Vec3::new(1.0, 0.0, 0.0)), 0.0);
    }

    #[test]
    fn test_index()  {
        let v = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(v.x == v[0]);
        assert_eq!(v.y == v[1]);
        assert_eq!(v.z == v[2]);
    }

        
}
