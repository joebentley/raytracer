pub struct Colour {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Colour {
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Colour { r, g, b }
    }

    pub fn as_rgb24(&self) -> u32 {
        let r = f32_0_1_to_u8_0_255(self.r) as u32;
        let g = f32_0_1_to_u8_0_255(self.g) as u32;
        let b = f32_0_1_to_u8_0_255(self.b) as u32;

        return b + (g << 8) + (r << 16);
    }
}

fn f32_0_1_to_u8_0_255(value_f: f32) -> u8 {
    (value_f.clamp(0., 1.) * 255.).floor() as u8
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn f32_0_1_to_u8_0_255_captures_whole_range() {
        assert_eq!(f32_0_1_to_u8_0_255(0.), 0);
        assert_eq!(f32_0_1_to_u8_0_255(-0.5), 0);
        assert_eq!(f32_0_1_to_u8_0_255(1.), 255);
        assert_eq!(f32_0_1_to_u8_0_255(1.2), 255);
    }
}
