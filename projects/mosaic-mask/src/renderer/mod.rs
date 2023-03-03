use std::path::Path;

use image::{io::Reader, Rgba, RgbaImage};

pub struct MaskRenderer {
    raw: RgbaImage,
    background: Option<Rgba<f32>>,
    grid_size: f32,
    magnification: f32,
}

impl MaskRenderer {
    pub fn load<P: AsRef<Path>>(path: P) -> Self {
        let raw = Reader::open(path).unwrap().decode().unwrap().into_rgba8();
        Self { raw, background: None, grid_size: 10.0, magnification: 1.0 }
    }
}
