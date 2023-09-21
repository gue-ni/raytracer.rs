use std::ops::{Add, Div, Index, IndexMut, Mul, Neg, Sub};

pub trait SquareRoot {
    fn sqrt(self) -> Self;
}

impl SquareRoot for f32 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}

impl SquareRoot for f64 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}

pub trait Number:
    Copy
    + Clone
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + Neg<Output = Self>
{
}

impl Number for f32 {}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Vec3<T>
where
    T: Number,
{
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }

    pub fn fill(v: T) -> Self {
        Self::new(v, v, v)
    }
}

impl<T> From<(T, T, T)> for Vec3<T>
where
    T: Number,
{
    fn from(item: (T, T, T)) -> Self {
        Self {
            x: item.0,
            y: item.1,
            z: item.2,
        }
    }
}

impl<T> Add for Vec3<T>
where
    T: Number,
{
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl<T> Add<T> for Vec3<T>
where
    T: Number,
{
    type Output = Self;
    fn add(self, other: T) -> Self {
        Self::new(self.x + other, self.y + other, self.z + other)
    }
}

impl<T> Mul<T> for Vec3<T>
where
    T: Number,
{
    type Output = Self;
    fn mul(self, scalar: T) -> Self {
        Self::new(self.x * scalar, self.y * scalar, self.z * scalar)
    }
}

impl<T> Div<T> for Vec3<T>
where
    T: Number,
{
    type Output = Self;
    fn div(self, scalar: T) -> Self {
        Self::new(self.x / scalar, self.y / scalar, self.z / scalar)
    }
}

impl<T> Sub<T> for Vec3<T>
where
    T: Number,
{
    type Output = Self;
    fn sub(self, scalar: T) -> Self {
        Self::new(self.x - scalar, self.y - scalar, self.z - scalar)
    }
}

/*
impl<T> Add for Vec3<T>
where
    T: Number,
{
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}
*/

impl<T> Sub for Vec3<T>
where
    T: Number,
{
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl<T> Mul for Vec3<T>
where
    T: Number,
{
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Self::new(self.x * other.x, self.y * other.y, self.z * other.z)
    }
}

impl<T> Div for Vec3<T>
where
    T: Number,
{
    type Output = Self;
    fn div(self, other: Self) -> Self {
        Self::new(self.x / other.x, self.y / other.y, self.z / other.z)
    }
}

impl<T> Neg for Vec3<T>
where
    T: Number,
{
    type Output = Self;
    fn neg(self) -> Self {
        Self::new(-self.x, -self.y, -self.z)
    }
}

impl<T> Index<usize> for Vec3<T> {
    type Output = T;
    fn index(&self, i: usize) -> &T {
        match i {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => &self.z,
        }
    }
}

impl<T> IndexMut<usize> for Vec3<T> {
    fn index_mut(&mut self, i: usize) -> &mut T {
        match i {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => &mut self.z,
        }
    }
}

pub trait Magnitude<T>
where
    T: Number + SquareRoot,
{
    fn length(self) -> T;
    fn normalize(v: Self) -> Self;
}

impl<T> Magnitude<T> for Vec3<T>
where
    T: Number + SquareRoot,
{
    fn length(self) -> T {
        Self::dot(self, self).sqrt()
    }

    fn normalize(v: Self) -> Self {
        v / v.length()
    }
}

pub trait Dot<T> {
    fn dot(a: Self, b: Self) -> T;
}

impl<T> Dot<T> for Vec3<T>
where
    T: Number,
{
    fn dot(a: Self, b: Self) -> T {
        a.x * b.x + a.y * b.y + a.z * b.z
    }
}

pub trait Cross<T> {
    fn cross(a: Self, b: Self) -> Self;
}

impl<T> Cross<T> for Vec3<T>
where
    T: Number,
{
    fn cross(a: Self, b: Self) -> Self {
        Self::new(
            a.y * b.z - a.z * b.y,
            a.z * b.x - a.x * b.z,
            a.x * b.y - a.y * b.x,
        )
    }
}

pub trait Lerp<T> {
    fn lerp(a: Self, b: Self, t: T) -> Self;
}

impl<T> Lerp<T> for Vec3<T>
where
    T: Number,
{
    fn lerp(a: Self, b: Self, t: T) -> Self {
        a + (b - a) * t
    }
}

impl<T> Lerp<T> for Vec2<T>
where
    T: Number,
{
    fn lerp(a: Self, b: Self, t: T) -> Self {
        a + (b - a) * t
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vec2<T>
where
    T: Number,
{
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    #[allow(dead_code)]
    pub fn fill(v: T) -> Self {
        Self::new(v, v)
    }
}

impl<T> From<(T, T)> for Vec2<T>
where
    T: Number,
{
    fn from(item: (T, T)) -> Self {
        Self {
            x: item.0,
            y: item.1,
        }
    }
}

impl<T> Div<T> for Vec2<T>
where
    T: Number,
{
    type Output = Self;
    fn div(self, scalar: T) -> Self {
        Self::new(self.x / scalar, self.y / scalar)
    }
}

impl<T> Sub<T> for Vec2<T>
where
    T: Number,
{
    type Output = Self;
    fn sub(self, scalar: T) -> Self {
        Self::new(self.x - scalar, self.y - scalar)
    }
}

impl<T> Mul<T> for Vec2<T>
where
    T: Number,
{
    type Output = Self;
    fn mul(self, scalar: T) -> Self {
        Self::new(self.x * scalar, self.y * scalar)
    }
}

impl<T> Add<T> for Vec2<T>
where
    T: Number,
{
    type Output = Self;
    fn add(self, scalar: T) -> Self {
        Self::new(self.x + scalar, self.y + scalar)
    }
}

impl<T> Mul for Vec2<T>
where
    T: Number,
{
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Self::new(self.x * other.x, self.y * other.y)
    }
}

impl<T> Sub for Vec2<T>
where
    T: Number,
{
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self::new(self.x - other.x, self.y - other.y)
    }
}

impl<T> Add for Vec2<T>
where
    T: Number,
{
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self::new(self.x + other.x, self.y + other.y)
    }
}

impl<T> Div for Vec2<T>
where
    T: Number,
{
    type Output = Self;
    fn div(self, other: Self) -> Self {
        Self::new(self.x / other.x, self.y / other.y)
    }
}

#[allow(dead_code)]
pub type Vec3f = Vec3<f32>;

#[allow(dead_code)]
pub type Vec3i = Vec3<i32>;

#[allow(dead_code)]
pub type Vec2f = Vec2<f32>;

#[allow(dead_code)]
pub type Vec2i = Vec2<i32>;

#[allow(dead_code)]
pub type Vec2u = Vec2<u32>;

impl From<(u32, u32)> for Vec2f {
    fn from(item: (u32, u32)) -> Self {
        Self {
            x: item.0 as f32,
            y: item.1 as f32,
        }
    }
}

impl From<Vec2u> for Vec2f {
    fn from(item: Vec2u) -> Self {
        Self {
            x: item.x as f32,
            y: item.y as f32,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::vector::*;

    #[test]
    fn test_length() {
        let v0 = Vec3f::new(1.0, 0.0, 0.0);
        assert_eq!(v0.length(), 1.0);
    }

    #[test]
    fn test_normalize() {
        let v0 = Vec3f::new(1.0, 5.0, 1.0);
        assert_eq!(Vec3f::normalize(v0).length(), 1.0);
    }

    #[test]
    fn test_cross() {
        let a = Vec3f::new(2.0, 3.0, 4.0);
        let b = Vec3f::new(5.0, 6.0, 7.0);
        assert_eq!(Vec3f::cross(a, b), Vec3f::new(-3.0, 6.0, -3.0));
    }

    #[test]
    fn test_dot() {
        assert_eq!(
            Vec3f::dot(Vec3f::new(0.0, 1.0, 0.0), Vec3f::new(1.0, 0.0, 0.0)),
            0.0
        );
    }

    #[test]
    fn test_math_vec3() {
        let a = Vec3f::new(2.0, 3.0, 4.0);
        let b = Vec3f::new(5.0, 6.0, 7.0);
        let s = 1.5;
        assert_eq!(a + b, Vec3f::new(4.0,0.0,0.0));
        assert_eq!(a - b, Vec3f::new(-2.0, 1.0, 1.0));
        assert_eq!(a * b, Vec3f::new(-2.0, 1.0, 1.0));
        assert_eq!(a / b, Vec3f::new(-2.0, 1.0, 1.0));


        assert_eq!(a + s, Vec3f::new(1.0, 1.0, 6.0));
        assert_eq!(a - s, Vec3f::new(1.0, 1.0, 6.0));
        assert_eq!(a * s, Vec3f::new(1.0, 1.0, 6.0));
        assert_eq!(a / s, Vec3f::new(1.0, 1.0, 6.0));
    }

    #[test]
    fn test_math_vec2() {
        let a = Vec2f::fill(1.0);
        let b = Vec2f::fill(3.0);
        assert_eq!(a + b, Vec2f::fill(4.0));
        assert_eq!(a - b, Vec2f::fill(-2.0));
        assert_eq!(b * 2.0, Vec2f::fill(6.0));
    }

    #[test]
    fn test_index() {
        let v = Vec3f::new(1.0, 2.0, 3.0);
        assert_eq!(v.x, v[0]);
        assert_eq!(v.y, v[1]);
        assert_eq!(v.z, v[2]);
    }
}
