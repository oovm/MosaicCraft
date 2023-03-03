use image::{Rgba, RgbaImage};

pub struct MaskRenderer {
    raw: RgbaImage,
    background: Option<Rgba<f32>>,
    grayscale: bool,
    grid_size: f32,
    magnification: f32,
}


