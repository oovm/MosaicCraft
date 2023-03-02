use core::slice::SlicePattern;

use image::DynamicImage;
use image::io::Reader;

use crate::{KeyColor, MosaicError};

use super::*;

impl ImageSignature {
    pub fn new<P: AsRef<Path>>(path: P) -> MosaicResult<Self> {
        new_signature(path.as_ref()).map_err(|e| e.with_path(path.as_ref()))
    }
    pub fn update<P: AsRef<Path>>(&mut self, path: P) -> MosaicResult<()> {
        let path = path.as_ref();
        if metadata(path)?.modified()? > self.last_modified {
            *self = Self::new(path)?;
        }
        Ok(())
    }
}

#[inline(always)]
fn new_signature(path: &Path) -> MosaicResult<ImageSignature> {
    let metadata = metadata(path)?;
    let name = match path.file_name().and_then(|s| s.to_str()) {
        Some(s) => { s.to_string() }
        None => {
            MosaicError::io_error("File has no file name")?
        }
    };
    let last_modified = metadata.modified()?;
    let image = Reader::open(path)?.with_guessed_format()?.decode()?;
    let main_color = find_main_color(&image)?;
    Ok(ImageSignature {
        name,
        width: image.width(),
        height: image.height(),
        last_modified,
        main_color,
        buffer: image.to_rgba8().to_vec(),
    })
}

pub fn find_main_color(image: &DynamicImage) -> MosaicResult<KeyColor> {
    let color_palette = get_palette_with_options(
        &image.to_rgb8().as_slice(),
        PixelEncoding::Rgb,
        Quality::new(5),
        MaxColors::new(1),
        PixelFilter::White,
    );
    match color_palette.first() {
        Some(s) => {
            Ok(KeyColor::new(s.r, s.g, s.b))
        }
        None => {
            MosaicError::io_error("No main color found")
        }
    }
}