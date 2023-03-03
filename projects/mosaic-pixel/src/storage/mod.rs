use std::{
    fs::metadata,
    path::{Path, PathBuf},
    time::SystemTime,
};

use palette_extract::{get_palette_with_options, MaxColors, PixelEncoding, PixelFilter, Quality};
use sled::{Db, Tree};

use serde_derive::{Deserialize, Serialize};
use std::fs::create_dir_all;

use crate::{KeyColor, MosaicResult};
use image::RgbaImage;

pub mod signature;
mod workspace;

pub struct StorageManager {
    workspace: PathBuf,
    database: Db,
}

#[derive(Clone, Debug)]
pub struct GalleryStorage {
    database: Tree,
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
