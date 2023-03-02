

use super::*;

impl WorkspaceStorage {
    pub fn new<P: AsRef<Path>>(directory: P) -> MosaicResult<Self> {
        let path = directory.as_ref();
        new_storage(path).map_err(|e| e.with_path(path))
    }
    pub fn get_closest_color(&self, color: &KeyColor) -> &ImageSignature {
        todo!()
    }
    pub fn get_gallery(&self, name: &str) -> MosaicResult<GalleryStorage> {
        Ok(GalleryStorage {
            database: self.database.open_tree(name.as_bytes())?,
        })
    }
    pub fn pack_gallery<P: AsRef<Path>>(&self, name: &str, path: P) -> MosaicResult<()> {
        let tree = self.database.open_tree(name.as_bytes())?;
        todo!()
    }
    pub fn load_gallery<P: AsRef<Path>>(&self, path: P) -> MosaicResult<GalleryStorage> {
        todo!()
    }
    pub fn drop_gallery(&self, name: &str) -> MosaicResult<bool> {
        Ok(self.database.drop_tree(name.as_bytes())?)
    }
}

impl GalleryStorage {
    pub fn get_closest_color(&self, color: &KeyColor) -> KeyColor {
        let mut closest = KeyColor::default();
        let mut distance = u16::MAX;
        for item in self.database.iter() {
            match item {
                Ok((key, _)) => {
                    key.to_vec()
                }
                Err(_) => {}
            }
        }
        closest
    }
    pub fn count(&self) -> usize {
        self.database.len()
    }
    pub fn get_image(&self, color: &KeyColor) -> MosaicResult<RgbaImage> {
        match self.database.get(color.as_bytes())? {
            Some(s) => {
                // let sig: ImageSignature;
                // match RgbaImage::from_raw(&sig.width, &sig.height, &sig.buffer) {
                //     Some(s) => {}
                //     None => {}
                // }
                //
                // Ok(&sig.buffer)
                todo!()
            }
            None => {
                todo!()
            }
        }
    }
}

impl AsRef<[u8]> for GalleryStorage {
    fn as_ref(&self) -> &[u8] {
        todo!()
    }
}

impl KeyColor {
    pub fn as_bytes(&self) -> [u8; 3] {
        [self.r, self.g, self.b]
    }
}

#[inline(always)]
fn new_storage(path: &Path) -> MosaicResult<WorkspaceStorage> {
    if !path.exists() {
        create_dir_all(path)?;
    }
    let workspace = path.canonicalize()?;
    if !path.is_dir() {
        todo!()
    }


    let db = sled::open(&workspace)?;
    Ok(WorkspaceStorage {
        workspace,
        database: db,
    })
}