use std::fmt::{Debug, Display, Formatter, LowerHex, UpperHex};
use std::ops::BitXor;

use sled::IVec;

use serde_derive::{Deserialize, Serialize};

#[derive(Copy, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct KeyColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Default for KeyColor {
    fn default() -> Self {
        Self::BLACK
    }
}

impl UpperHex for KeyColor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "#{:02X}{:02X}{:02X}", self.r, self.g, self.b)
    }
}

impl LowerHex for KeyColor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "#{:02x}{:02x}{:02x}", self.r, self.g, self.b)
    }
}

impl Debug for KeyColor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Color({}, {}, {})", self.r, self.g, self.b)
    }
}

impl Display for KeyColor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Color({}, {}, {})", self.r as f32 / 255.0, self.g as f32 / 255.0, self.b as f32 / 255.0)
    }
}

impl BitXor for KeyColor {
    type Output = u32;
    // 255^2 + 255^2 + 255^2 = 195075 < u32::MAX
    fn bitxor(self, rhs: Self) -> Self::Output {
        let mut distance = 0;
        distance += (self.r.abs_diff(rhs.r) as u32).pow(2);
        distance += (self.g.abs_diff(rhs.g) as u32).pow(2);
        distance += (self.b.abs_diff(rhs.b) as u32).pow(2);
        distance
    }
}

impl KeyColor {
    pub const BLACK: Self = Self {
        r: 0,
        g: 0,
        b: 0,
    };
    pub const WHITE: Self = Self {
        r: 255,
        g: 255,
        b: 255,
    };
    pub const RED: Self = Self {
        r: 255,
        g: 0,
        b: 0,
    };
    pub const GREEN: Self = Self {
        r: 0,
        g: 255,
        b: 0,
    };
    pub const BLUE: Self = Self {
        r: 0,
        g: 0,
        b: 255,
    };
    pub const YELLOW: Self = Self {
        r: 255,
        g: 255,
        b: 0,
    };
    pub const CYAN: Self = Self {
        r: 0,
        g: 255,
        b: 255,
    };
    pub const MAGENTA: Self = Self {
        r: 255,
        g: 0,
        b: 255,
    };
}

impl KeyColor {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
}

impl From<IVec> for KeyColor {
    fn from(value: IVec) -> Self {
        let mut out = KeyColor::default();
        for (idx, value) in value.to_vec().iter().enumerate() {
            match idx {
                0 => {
                    out.r = *value
                }
                1 => {
                    out.g = *value
                }
                2 => {
                    out.b = *value
                }
                _ => {
                    break;
                }
            }
        }
        out
    }
}