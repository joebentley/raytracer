use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector {
    x: f64,
    y: f64,
    z: f64,
}

impl Vector {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn zero() -> Self {
        Self {
            x: 0.,
            y: 0.,
            z: 0.,
        }
    }

    pub fn abs_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(&self) -> f64 {
        self.abs_squared().sqrt()
    }

    pub fn normalise(&mut self) {
        let length = self.length();
        self.x /= length;
        self.y /= length;
        self.z /= length;
    }

    pub fn normalised(mut self) -> Self {
        self.normalise();
        return self;
    }

    pub fn dot(&self, other: &Self) -> f64 {
        return self.x * other.x + self.y * other.y + self.z * other.z;
    }
}

impl Add for Vector {
    type Output = Self;
    fn add(mut self, other: Self) -> Self {
        self += other;
        return self;
    }
}

impl AddAssign for Vector {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for Vector {
    type Output = Self;
    fn sub(mut self, other: Self) -> Self {
        self -= other;
        return self;
    }
}

impl SubAssign for Vector {
    fn sub_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Mul<f64> for Vector {
    type Output = Self;
    fn mul(mut self, rhs: f64) -> Self {
        self *= rhs;
        return self;
    }
}

impl MulAssign<f64> for Vector {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl Mul<i32> for Vector {
    type Output = Self;
    fn mul(mut self, rhs: i32) -> Self {
        self *= rhs;
        return self;
    }
}

impl MulAssign<i32> for Vector {
    fn mul_assign(&mut self, rhs: i32) {
        self.x *= rhs as f64;
        self.y *= rhs as f64;
        self.z *= rhs as f64;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adding_vectors() {
        let mut a = Vector::new(1., 2., 3.);
        let b = Vector::new(3., 2., 1.);
        assert_eq!(a + b, Vector::new(4., 4., 4.));
        a += b;
        assert_eq!(a, Vector::new(4., 4., 4.));
    }

    #[test]
    fn subbing_vectors() {
        let mut a = Vector::new(1., 2., 3.);
        let b = Vector::new(3., 2., 1.);
        assert_eq!(a - b, Vector::new(-2., 0., 2.));
        a -= b;
        assert_eq!(a, Vector::new(-2., 0., 2.));
    }

    #[test]
    fn vector_abs_squared_and_length() {
        let a = Vector::new(1., 2., 3.);
        assert_eq!(a.abs_squared(), 14.);
        assert_eq!(a.length(), (14.0 as f64).sqrt());
        assert_eq!(a.abs_squared(), a.dot(&a));
    }

    #[test]
    fn vector_normalisation() {
        let mut a = Vector::new(1., 2., 3.);
        let length = a.length();
        let a_normalised = a.normalised();
        assert_eq!(a_normalised.x, 1. / length);
        a.normalise();
        assert_eq!(a.y, 2. / length);
    }

    #[test]
    fn vector_scalar_mul() {
        let mut a = Vector::new(1., 2., 3.);
        let b = a * 4.;
        assert_eq!(b.y, 8.);
        let c = a * 4;
        assert_eq!(c.y, 8.);
        a *= 2.;
        assert_eq!(a.y, 4.);
        a *= 2;
        assert_eq!(a.y, 8.);
    }

    #[test]
    fn vector_dot() {
        let a = Vector::new(1., 2., 3.);
        let b = Vector::new(2., 3., 4.);
        assert_eq!(a.dot(&b), 20.);
    }
}
