use math::{Float, Vector};

#[derive(Debug, Copy, Clone)]
pub struct ScreenSpaceColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

pub mod const_color {
    use super::*;

    pub const PURPLE: ScreenSpaceColor = ScreenSpaceColor {
        r: 255,
        g: 0,
        b: 255,
        a: 255,
    };
}

impl ScreenSpaceColor {
    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
    }

    pub fn as_rgb_u32(&self) -> u32 {
        ((self.r as u32) << 24) | ((self.g as u32) << 16) | ((self.b as u32) << 8)
    }
}

impl<F: Float> Into<ScreenSpaceColor> for Vector<F> {
    fn into(self) -> ScreenSpaceColor {
        ScreenSpaceColor::rgb(unit_to_u8(self.x), unit_to_u8(self.y), unit_to_u8(self.z))
    }
}

fn unit_to_u8<F: Float>(unit: F) -> u8 {
    let two = F::one() + F::one();
    let eight = two + two + two + two;
    let max_val = (eight * eight * two * two) - F::one();
    let val = (unit + F::one()) / (F::one() + F::one());
    (val * max_val).to_u8().unwrap_or(0)
}
