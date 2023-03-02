use std::fs::metadata;
use std::path::Path;
use crate::MosaicResult;

pub struct ImageSignature {
    pub name: String,
    pub width: u32,
    pub height: u32,
    pub last_modified: u64,
}


impl ImageSignature {
    pub fn new<P: AsRef<Path>>(path: P) -> MosaicResult<Self> {
        let metadata = metadata(path)?;
        let name = metadata.modified()?;


        let width = metadata.width();
        let height = metadata.height();
        let last_modified = metadata.last_modified();
        Self {
            name,
            width,
            height,
            last_modified,
        }
    }
}