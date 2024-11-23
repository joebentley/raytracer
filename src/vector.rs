use std::ops::{Add, AddAssign, Sub, SubAssign};

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
}
