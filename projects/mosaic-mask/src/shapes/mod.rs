use image::{GrayImage, Luma, RgbaImage};

#[derive(Debug, Clone)]
pub enum MosaicShape {
    Square,
    Circle,
    Custom(GrayImage),
}

pub enum MosaicAction {}

impl MosaicShape {
    /// A mask image, 255 = accept, 0 = reject
    pub fn get_mask(&self, size: u32) -> GrayImage {
        // all zero image
        let mut shape = GrayImage::new(size, size);
        match self {
            MosaicShape::Square => {
                shape.fill(255);
            }
            MosaicShape::Circle => {
                let radius = size as f32 / 2.0;
                for (x, y, pixel) in shape.enumerate_pixels_mut() {
                    let x = x as f32 - radius;
                    let y = y as f32 - radius;
                    let distance = (x * x + y * y).sqrt();
                    if distance < radius {
                        *pixel = Luma([255]);
                    }
                }
            }
            MosaicShape::Custom(s) => shape = s.clone(),
        }
        shape
    }
    pub fn mask_cell(&self, image: &mut RgbaImage, size: u32, x_start: u32, y_start: u32) {
        for (mx, row) in self.get_mask(size).rows().enumerate() {
            for (my, alpha) in row.enumerate() {
                let px = x_start + mx as u32;
                let py = y_start + my as u32;
                match image.get_pixel_mut_checked(px, py) {
                    Some(s) => s.0[3] = alpha.0[0],
                    None => continue,
                }
            }
        }
    }
}

#[test]
fn test() {
    let shape = MosaicShape::Square {};
    let mask = shape.get_mask(100);
    mask.save("mask.png").unwrap();
}
