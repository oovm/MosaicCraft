use std::fs::metadata;
use std::path::Path;
use std::time::SystemTime;
use image::Rgb;

use palette_extract::{get_palette_with_options, MaxColors, PixelEncoding, PixelFilter, Quality};

use crate::MosaicResult;

mod signature;

pub struct ImageSignature {
    pub name: String,
    pub width: u32,
    pub height: u32,
    pub last_modified: SystemTime,
    pub main_color: Rgb<u8>,
}

