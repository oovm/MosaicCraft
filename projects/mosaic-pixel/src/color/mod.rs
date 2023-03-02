use sled::IVec;

use serde_derive::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
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
                    break
                }
            }
        }
        out
    }
}