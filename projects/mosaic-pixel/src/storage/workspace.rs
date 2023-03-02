use super::*;

impl ImageStorage {
    pub fn new<P: AsRef<Path>>(directory: P) -> MosaicResult<Self> {
        let path = directory.as_ref();
        new_storage(path).map_err(|e| e.with_path(path))
    }
    pub fn get_closest_color(&self, color: &KeyColor) -> &ImageSignature {
        todo!()
    }
}

#[inline(always)]
fn new_storage(path: &Path) -> MosaicResult<ImageStorage> {
    let workspace = path.canonicalize()?;
    let db = sled::open(&workspace)?;
    Ok(ImageStorage {
        workspace,
        database: db,
    })
}