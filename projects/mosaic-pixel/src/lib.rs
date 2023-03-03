#![feature(slice_pattern)]

use image::Rgba;

pub use crate::{
    color::KeyColor,
    error::{MosaicError, MosaicResult},
    storage::{GalleryStorage, ImageSignature, StorageManager},
};

mod color;
mod error;
mod renderer;
mod storage;

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
