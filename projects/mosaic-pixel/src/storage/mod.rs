use std::fs::metadata;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use palette_extract::{get_palette_with_options, MaxColors, PixelEncoding, PixelFilter, Quality};
use serde_derive::{Deserialize, Serialize};
use sled::Db;

use crate::MosaicResult;

pub mod signature;
mod workspace;


pub struct ImageStorage {
    workspace: PathBuf,
    database: Db,
}


#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
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
    pub buffer: Vec<u8>,
}
