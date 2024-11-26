use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Colour {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Colour {
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Colour { r, g, b }
    }

    pub fn white() -> Self {
        Self::new(1., 1., 1.)
    }

    pub fn black() -> Self {
        Self::new(0., 0., 0.)
    }

    pub fn as_rgb24(&self) -> u32 {
        let r = f32_0_1_to_u8_0_255(self.r) as u32;
        let g = f32_0_1_to_u8_0_255(self.g) as u32;
        let b = f32_0_1_to_u8_0_255(self.b) as u32;

        return b + (g << 8) + (r << 16);
    }
}

impl Add for Colour {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Colour {
            r: self.r + other.r,
            g: self.g + other.g,
            b: self.b + other.b,
        }
    }
}

impl AddAssign for Colour {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
        }
    }
}

impl Mul<f32> for Colour {
    type Output = Colour;

    fn mul(mut self, rhs: f32) -> Self::Output {
        self *= rhs;
        return self;
    }
}

impl MulAssign<f32> for Colour {
    fn mul_assign(&mut self, rhs: f32) {
        self.r *= rhs;
        self.g *= rhs;
        self.b *= rhs;
    }
}

impl Div<f32> for Colour {
    type Output = Colour;

    fn div(mut self, rhs: f32) -> Self::Output {
        self /= rhs;
        return self;
    }
}

impl DivAssign<f32> for Colour {
    fn div_assign(&mut self, rhs: f32) {
        self.r /= rhs;
        self.g /= rhs;
        self.b /= rhs;
    }
}

fn f32_0_1_to_u8_0_255(value_f: f32) -> u8 {
    (value_f.clamp(0., 1.) * 255.).floor() as u8
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adding_colours() {
        let mut red = Colour::new(1., 0., 0.);
        let blue = Colour::new(0., 1., 0.);
        assert_eq!(red + blue, Colour::new(1., 1., 0.));
        red += blue;
        assert_eq!(red, Colour::new(1., 1., 0.));
    }

    #[test]
    fn colour_to_rgb24() {
        let red = Colour::new(1., 0., 0.);
        assert_eq!(red.as_rgb24(), 0xFF0000);
    }

    #[test]
    fn f32_0_1_to_u8_0_255_captures_whole_range() {
        assert_eq!(f32_0_1_to_u8_0_255(0.), 0);
        assert_eq!(f32_0_1_to_u8_0_255(-0.5), 0);
        assert_eq!(f32_0_1_to_u8_0_255(1.), 255);
        assert_eq!(f32_0_1_to_u8_0_255(1.2), 255);
    }
}
