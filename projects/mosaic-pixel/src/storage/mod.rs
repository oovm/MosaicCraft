use std::fs::metadata;
use std::path::Path;
use std::time::SystemTime;

use image::Rgb;
use palette_extract::{get_palette_with_options, MaxColors, PixelEncoding, PixelFilter, Quality};
use serde::{Deserialize, Serialize};
use sled_typed::DiskMap;

use crate::MosaicResult;

pub mod signature;


pub struct ImageStorage {
    store: DiskMap<KeyColor, ImageSignature>,
}

impl ImageStorage {
    pub fn new<P: AsRef<Path>>(dir: P) -> Self {
        todo!()
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub struct KeyColor {
    r: u8,
    g: u8,
    b: u8,
}

#[derive(Serialize, Deserialize)]
pub struct ImageSignature {
    pub name: String,
    pub width: u32,
    pub height: u32,
    pub last_modified: SystemTime,
    pub main_color: KeyColor,
    pub buffer: Vec<u8>
}
