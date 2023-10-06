use serde::{Deserialize, Serialize};
use std::ops::{Add, AddAssign, Div, Index, IndexMut, Mul, MulAssign, Neg, Sub};

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

impl Number for f64 {}
impl Number for i32 {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
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
}

impl<T> From<(T, T, T)> for Vec3<T>
where
    T: Number,
{
    fn from(item: (T, T, T)) -> Self {
        Self::new(item.0, item.1, item.2)
    }
}

impl<T> From<[T; 3]> for Vec3<T>
where
    T: Number,
{
    fn from(item: [T; 3]) -> Self {
        Self::new(item[0], item[1], item[2])
    }
}

impl<T> From<T> for Vec3<T>
where
    T: Number,
{
    fn from(v: T) -> Self {
        Self::new(v, v, v)
    }
}

impl<T> Default for Vec3<T>
where
    T: Number + std::default::Default,
{
    fn default() -> Self {
        Self::new(T::default(), T::default(), T::default())
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

impl<T> AddAssign for Vec3<T>
where
    T: Number,
{
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        };
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

impl<T> MulAssign for Vec3<T>
where
    T: Number,
{
    fn mul_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        };
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
    fn normalize(self) -> Self;
}

impl<T> Magnitude<T> for Vec3<T>
where
    T: Number + SquareRoot,
{
    fn length(self) -> T {
        Self::dot(self, self).sqrt()
    }

    fn normalize(self) -> Self {
        self / self.length()
    }
}

pub trait Dot<T> {
    fn dot(self, b: Self) -> T;
}

impl<T> Dot<T> for Vec3<T>
where
    T: Number,
{
    fn dot(self, b: Self) -> T {
        self.x * b.x + self.y * b.y + self.z * b.z
    }
}

pub trait Cross<T> {
    fn cross(self, b: Self) -> Self;
}

impl<T> Cross<T> for Vec3<T>
where
    T: Number,
{
    fn cross(self, b: Self) -> Self {
        Self::new(
            self.y * b.z - self.z * b.y,
            self.z * b.x - self.x * b.z,
            self.x * b.y - self.y * b.x,
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

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
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
}

impl<T> Default for Vec2<T>
where
    T: Number + std::default::Default,
{
    fn default() -> Self {
        Self::new(T::default(), T::default())
    }
}

impl<T> From<T> for Vec2<T>
where
    T: Number,
{
    fn from(v: T) -> Self {
        Self::new(v, v)
    }
}

impl<T> From<(T, T)> for Vec2<T>
where
    T: Number,
{
    fn from(item: (T, T)) -> Self {
        Self::new(item.0, item.1)
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

impl From<(u32, u32)> for Vec2f {
    fn from(item: (u32, u32)) -> Self {
        Self {
            x: item.0 as f64,
            y: item.1 as f64,
        }
    }
}

impl From<Vec2u> for Vec2f {
    fn from(item: Vec2u) -> Self {
        Self {
            x: item.x as f64,
            y: item.y as f64,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Mat3<T> {
    pub m: [T; 3 * 3],
}

impl<T> Mat3<T>
where
    T: Number + SquareRoot,
{
    pub fn look_at(eye: Vec3<T>, target: Vec3<T>, up: Vec3<T>) -> Self {
        let z_axis = (target - eye).normalize();
        let x_axis = Vec3::cross(z_axis, up).normalize();
        let y_axis = Vec3::cross(z_axis, x_axis);

        Mat3::from([x_axis, y_axis, z_axis])
    }
}

impl<T> Default for Mat3<T>
where
    T: Number + std::default::Default,
{
    fn default() -> Self {
        Self {
            m: [T::default(); 3 * 3],
        }
    }
}

impl<T> From<T> for Mat3<T>
where
    T: Number,
{
    fn from(item: T) -> Self {
        Self { m: [item; 3 * 3] }
    }
}

impl<T> From<[T; 3 * 3]> for Mat3<T>
where
    T: Number,
{
    fn from(item: [T; 3 * 3]) -> Self {
        Self { m: item }
    }
}

impl<T> From<[Vec3<T>; 3]> for Mat3<T>
where
    T: Number,
{
    fn from(item: [Vec3<T>; 3]) -> Self {
        Self {
            m: [
                item[0].x, item[0].y, item[0].z, item[1].x, item[1].y, item[1].z, item[2].x,
                item[2].y, item[2].z,
            ],
        }
    }
}

impl<T> Mul for Mat3<T>
where
    T: Number + std::default::Default + AddAssign,
{
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        let mut result = Self::default();

        const ROWS: usize = 3;
        const COLUMNS: usize = 3;

        for i in 0..ROWS {
            for j in 0..COLUMNS {
                let mut sum = T::default();
                for k in 0..COLUMNS {
                    sum += self.m[i * COLUMNS + k] * other.m[k * COLUMNS + j];
                }
                result.m[i * COLUMNS + j] = sum;
            }
        }

        result
    }
}

impl<T> Mul<Vec3<T>> for Mat3<T>
where
    T: Number + std::default::Default + AddAssign,
{
    type Output = Vec3<T>;
    fn mul(self, other: Vec3<T>) -> Vec3<T> {
        let mut result = Vec3::<T>::default();

        const ROWS: usize = 3;
        const COLUMNS: usize = 3;

        for i in 0..ROWS {
            let mut sum = T::default();
            for j in 0..COLUMNS {
                sum += self.m[i * COLUMNS + j] * other[j];
            }
            result[i] = sum;
        }

        result
    }
}

pub type Vec3f = Vec3<f64>;
pub type Vec3i = Vec3<i32>;
pub type Vec3u = Vec3<u32>;

pub type Vec2f = Vec2<f64>;
pub type Vec2i = Vec2<i32>;
pub type Vec2u = Vec2<u32>;

pub type Mat3f = Mat3<f64>;

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
        assert_eq!(Vec3::normalize(v0).length(), 1.0);
    }

    #[test]
    fn test_cross() {
        let a = Vec3::new(2.0, 3.0, 4.0);
        let b = Vec3::new(5.0, 6.0, 7.0);
        let c = Vec3::new(-3.0, 6.0, -3.0);
        assert_eq!(Vec3::cross(a, b), c);
    }

    #[test]
    fn test_cross_2() {
        let a = Vec3::new(1.0, 0.0, 0.0);
        let b = Vec3::new(0.0, 1.0, 0.0);
        let c = Vec3::new(0.0, 0.0, 1.0);
        assert_eq!(Vec3::cross(a, b), c);
    }

    #[test]
    fn test_dot() {
        assert_eq!(
            Vec3::dot(Vec3::new(0.0, 1.0, 0.0), Vec3::new(1.0, 0.0, 0.0)),
            0.0
        );
    }

    #[test]
    fn test_math_vec3() {
        let a = Vec3::new(2.0, 3.0, 4.0);
        let b = Vec3::new(5.0, 6.0, 7.0);
        let _s = 1.5;
        assert_eq!(a + b, Vec3::new(7.0, 9.0, 11.0));
        assert_eq!(a - b, Vec3::new(-3.0, -3.0, -3.0));
        assert_eq!(a * b, Vec3::new(10.0, 18.0, 28.0));

        let c = Vec3::from(6.0);
        let d = Vec3::from(3.0);
        assert_eq!(c / d, Vec3::from(2.0));

        //assert_eq!(a + s, Vec3f::new(1.0, 1.0, 6.0));
        //assert_eq!(a - s, Vec3f::new(1.0, 1.0, 6.0));
        //assert_eq!(a * s, Vec3f::new(1.0, 1.0, 6.0));
        //assert_eq!(a / s, Vec3f::new(1.0, 1.0, 6.0));
    }

    #[test]
    fn test_math_vec2() {
        let _a = Vec2::new(2.0, 3.0);
        let _b = Vec2::new(4.0, 5.0);
        let _s = 1.5;

        // assert_eq!(a + b, Vec2f::new(1.0, 1.0));
        // assert_eq!(a - b, Vec2f::new(1.0, 1.0));
        // assert_eq!(a / b, Vec2f::new(1.0, 1.0));
        // assert_eq!(a * b, Vec2f::new(1.0, 1.0));

        // assert_eq!(a + s, Vec2f::new(1.0, 1.0));
        // assert_eq!(a - s, Vec2f::new(1.0, 1.0));
        // assert_eq!(a / s, Vec2f::new(1.0, 1.0));
        // assert_eq!(a * s, Vec2f::new(1.0, 1.0));
    }

    #[test]
    fn test_index() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(v.x, v[0]);
        assert_eq!(v.y, v[1]);
        assert_eq!(v.z, v[2]);
    }

    #[test]
    fn test_deserialize() {
        {
            let json = r#"{"x": 1.0, "y": 2.0, "z": 3.0}"#;
            let vec: Vec3f = serde_json::from_str(json).unwrap();
            println!("{:?}", vec);
            assert_eq!(vec, Vec3f::new(1.0, 2.0, 3.0));
        }
        {
            let json = r#"[1.0, 2.0, 3.0]"#;
            let vec: Vec3f = serde_json::from_str(json).unwrap();
            println!("{:?}", vec);
            assert_eq!(vec, Vec3f::new(1.0, 2.0, 3.0));
        }
    }

    #[test]
    fn test_serialize() {
        let vec = Vec3f::new(1.0, 2.0, 3.0);
        let json = serde_json::to_string(&vec).unwrap();
        println!("{:?}", json);
    }

    #[test]
    fn test_mat3_mat3_mult() {
        let a = Mat3::from([2.0, 7.0, 3.0, 1.0, 5.0, 8.0, 0.0, 4.0, 1.0]);
        let b = Mat3::from([3.0, 0.0, 1.0, 2.0, 1.0, 0.0, 1.0, 2.0, 4.0]);
        let c = Mat3::from([23.0, 13.0, 14.0, 21.0, 21.0, 33.0, 9.0, 6.0, 4.0]);
        assert_eq!(a * b, c);
    }

    #[test]
    fn test_mat3_vec3_mult() {
        let a = Mat3::from([1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]);
        let b = Vec3::from([2.0, 1.0, 3.0]);
        let c = Vec3::from([13.0, 31.0, 49.0]);
        assert_eq!(a * b, c);
    }

    #[test]
    fn test_look_at() {
        let up = Vec3::new(0.0, 1.0, 0.0);
        let eye = Vec3::new(0.0, 0.0, 0.0);
        let target = Vec3::new(5.0, 0.0, 0.0);

        let mat = Mat3::look_at(eye, target, up);
        println!("{:?}", mat);

        let v0 = Vec3::new(0.0, 0.0, 1.0);
        let v1 = Vec3::new(1.0, 0.0, 0.0);

        let r = mat * v0;

        println!("{:?}", r);

        assert_eq!(r, v1)
    }
}
