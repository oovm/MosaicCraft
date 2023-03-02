use super::*;

impl ImageSignature {
    pub fn new<P: AsRef<Path>>(path: P) -> MosaicResult<Self> {
        let metadata = metadata(path)?;
        let name = metadata.modified()?;
        let width = metadata.width();
        let height = metadata.height();
        let last_modified = metadata.modified()?;
        Self {
            name,
            width,
            height,
            last_modified,
        }
    }
}

pub fn find_main_color(path: &Path) -> Rgb<u8> {
    let img = image::open(path).unwrap();

    let pixels: [u8; 12] = [255, 0, 0, 255, 0, 0, 255, 0, 0, 255, 0, 0];

    let color_palette = get_palette_with_options(&pixels,
                                                 PixelEncoding::Rgb,
                                                 Quality::new(1),
                                                 MaxColors::new(4),
                                                 PixelFilter::White);
}