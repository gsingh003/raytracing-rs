use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    #[inline]
    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    // Color aliases (R,G,B)
    #[inline]
    pub fn r(self) -> f64 {
        self.x
    }
    #[inline]
    pub fn g(self) -> f64 {
        self.y
    }
    #[inline]
    pub fn b(self) -> f64 {
        self.z
    }

    #[inline]
    pub fn length(self) -> f64 {
        self.length_squared().sqrt()
    }

    #[inline]
    pub const fn length_squared(self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    #[inline]
    pub fn unit_vector(v: Self) -> Self {
        v / v.length()
    }

    #[inline]
    pub fn dot(u: Self, v: Self) -> f64 {
        u.x * v.x + u.y * v.y + u.z * v.z
    }

    #[inline]
    pub fn cross(u: Self, v: Self) -> Self {
        Self {
            x: u.y * v.z - u.z * v.y,
            y: u.z * v.x - u.x * v.z,
            z: u.x * v.y - u.y * v.x,
        }
    }
}

// Unary -v
impl Neg for Vec3 {
    type Output = Self;
    #[inline]
    fn neg(self) -> Self::Output {
        Self::new(-self.x, -self.y, -self.z)
    }
}

// v + v
impl Add for Vec3 {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl AddAssign for Vec3 {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

// v - v
impl Sub for Vec3 {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl SubAssign for Vec3 {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

// Component-wise v * v
impl Mul for Vec3 {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }
}

impl MulAssign for Vec3 {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
    }
}

// Scalar v * t
impl Mul<f64> for Vec3 {
    type Output = Self;
    #[inline]
    fn mul(self, t: f64) -> Self::Output {
        Self::new(self.x * t, self.y * t, self.z * t)
    }
}

// Scalar t * v
impl Mul<Vec3> for f64 {
    type Output = Vec3;
    #[inline]
    fn mul(self, v: Vec3) -> Self::Output {
        v * self
    }
}

impl MulAssign<f64> for Vec3 {
    #[inline]
    fn mul_assign(&mut self, t: f64) {
        self.x *= t;
        self.y *= t;
        self.z *= t;
    }
}

// Scalar division v / t
impl Div<f64> for Vec3 {
    type Output = Self;
    #[inline]
    fn div(self, t: f64) -> Self::Output {
        let inv = 1.0 / t;
        Self::new(self.x * inv, self.y * inv, self.z * inv)
    }
}

impl DivAssign<f64> for Vec3 {
    #[inline]
    fn div_assign(&mut self, t: f64) {
        let inv = 1.0 / t;
        self.x *= inv;
        self.y *= inv;
        self.z *= inv;
    }
}

// Handy conversions
impl From<(f64, f64, f64)> for Vec3 {
    #[inline]
    fn from(t: (f64, f64, f64)) -> Self {
        Self::new(t.0, t.1, t.2)
    }
}

impl From<[f64; 3]> for Vec3 {
    #[inline]
    fn from(a: [f64; 3]) -> Self {
        Self::new(a[0], a[1], a[2])
    }
}

// Common aliases used in Shirley's book
pub type Point3 = Vec3;
pub type Color = Vec3;
