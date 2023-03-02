#![feature(slice_pattern)]


use image::Rgba;

pub use crate::error::{MosaicError, MosaicResult};
pub use crate::storage::{ImageStorage, KeyColor};
pub use crate::storage::ImageSignature;

mod renderer;
mod storage;
mod error;

#[derive(Debug, Copy, Clone)]
pub enum ColorSampler {
    CenterPixel,
    Average,
    Dominant,
    Vibrant,
}

#[derive(Debug, Copy, Clone)]
pub enum MosaicPixel {
    DiskMatrix,
    DiamondMatrix,
    BoxMatrix,
    IdentityMatrix,
    CrossMatrix,
    GaussianMatrix,
    Custom(),
}

pub struct MosaicRenderer {
    // Color sampler for every grid
    pub sampler: ColorSampler,
    pub shape: MosaicPixel,
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

impl Default for MosaicPixel {
    fn default() -> Self {
        Self::DiskMatrix
    }
}

impl Default for MosaicRenderer {
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
