use image::Rgba;

mod renderer;

#[derive(Debug, Copy, Clone)]
pub enum ColorSampler {
    CenterPixel,
    /// Average of all colors
    Average,
    // rgbaster.js
    Dominant,
    // Vibrant.js
    Vibrant,
}

#[derive(Debug, Copy, Clone)]
pub enum GridShape {
    DiskMatrix,
    DiamondMatrix,
    BoxMatrix,
    IdentityMatrix,
    CrossMatrix,
    GaussianMatrix,
    Custom(),
}

pub struct Mosaic {
    // Color sampler for every grid
    pub sampler: ColorSampler,
    pub shape: GridShape,
    pub background: Option<Rgba<f32>>,
    pub grayscale: bool,
    pub smooth: bool,
    pub grid_size: f32,
    pub shape_size: f32,
}

impl Default for ColorSampler {
    fn default() -> Self {
        Self::CenterPixel
    }
}

impl Default for GridShape {
    fn default() -> Self {
        Self::DiskMatrix
    }
}

impl Default for Mosaic {
    fn default() -> Self {
        Self {
            sampler: Default::default(),
            shape: Default::default(),
            background: None,
            grayscale: false,
            smooth: true,
            grid_size: 10.0,
            shape_size: 10.0,
        }
    }
}
