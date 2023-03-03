use sled::{Config, Mode};

use crate::MosaicError;

use super::*;

impl StorageManager {
    pub fn new<P: AsRef<Path>>(directory: P) -> MosaicResult<Self> {
        let path = directory.as_ref();
        new_storage(path).map_err(|e| e.with_path(path))
    }
    pub fn get_closest_image(&self, name: &str, color: KeyColor) -> MosaicResult<RgbaImage> {
        let store = self.get_gallery(name)?;
        let color = store.get_closest_color(color);
        store.get_image(color)
    }
    pub fn get_gallery(&self, name: &str) -> MosaicResult<GalleryStorage> {
        let store = GalleryStorage { database: self.database.open_tree(name.as_bytes())? };
        store.set_name(name);
        Ok(store)
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
    pub fn get_closest_color(&self, color: KeyColor) -> KeyColor {
        let mut closest = KeyColor::default();
        let mut distance = u32::MAX;
        for item in self.database.iter() {
            match item {
                Ok((key, _)) if key.len() == 3 => {
                    let key = KeyColor::from(key);
                    let this = color ^ key;
                    if this < distance {
                        distance = this;
                        closest = key;
                    }
                }
                Ok(_) => continue,
                Err(e) => {
                    log::error!("Error while iterating over gallery: {}", e);
                }
            }
        }
        closest
    }
    pub fn count(&self) -> usize {
        self.database.len()
    }
    pub fn get_image(&self, color: KeyColor) -> MosaicResult<RgbaImage> {
        match self.database.get(color.as_bytes())? {
            Some(s) => {
                let sig = ImageSignature::try_from(s)?;
                match RgbaImage::from_raw(sig.width, sig.height, sig.buffer) {
                    Some(s) => Ok(s),
                    None => MosaicError::runtime_error(format!(
                        "Image with main color `{:X}` in gallery `{}` is corrupted",
                        color,
                        self.get_name()
                    )),
                }
            }
            None => {
                MosaicError::runtime_error(format!("No image with main color `{:X}` in gallery `{}`", color, self.get_name()))
            }
        }
    }
    pub fn get_name(&self) -> String {
        match self.database.get("$name") {
            Ok(Some(s)) => unsafe { String::from_utf8_unchecked(s.to_vec()) },
            _ => "<anonymous>".to_string(),
        }
    }
    pub fn set_name(&self, name: &str) {
        self.database.insert("$name", name.as_bytes()).ok();
    }
}

impl KeyColor {
    pub fn as_bytes(&self) -> [u8; 3] {
        [self.r, self.g, self.b]
    }
}

#[inline(always)]
fn new_storage(path: &Path) -> MosaicResult<StorageManager> {
    if !path.exists() {
        create_dir_all(path)?;
    }
    let workspace = path.canonicalize()?;
    if !path.is_dir() {
        MosaicError::runtime_error(format!("Path `{}` is not a directory", workspace.display()))?
    }
    let db = Config::new().mode(Mode::HighThroughput).use_compression(true).path(path).open()?;
    Ok(StorageManager { workspace, database: db })
}
