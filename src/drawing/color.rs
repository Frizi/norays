use math::{Float, Vector};

#[derive(Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

pub mod const_color {
    use super::*;

    pub const PURPLE: Color = Color {
        r: 255,
        g: 0,
        b: 255,
    };
}

impl Color {
    pub fn rgb(r: u8, g: u8, b: u8) -> Color {
        Color { r, g, b }
    }

    pub fn as_rgb_u32(&self) -> u32 {
        ((self.r as u32) << 24) | ((self.g as u32) << 16) | ((self.b as u32) << 8)
    }
}

impl<'a, F: Float> Into<Color> for &'a Vector<F> {
    fn into(self) -> Color {
        Color {
            r: unit_to_u8(self.x),
            g: unit_to_u8(self.y),
            b: unit_to_u8(self.z),
        }
    }
}

fn unit_to_u8<F: Float>(unit: F) -> u8 {
    let two = F::one() + F::one();
    let eight = two + two + two + two;
    let max_val = (eight * eight * two * two) - F::one();
    let val = (unit + F::one()) / (F::one() + F::one());
    (val * max_val).to_u8().unwrap_or(0)
}
