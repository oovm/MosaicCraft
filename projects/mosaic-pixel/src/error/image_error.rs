use image::ImageError;

use crate::MosaicError;

impl From<ImageError> for MosaicError {
    fn from(value: ImageError) -> Self {
        match value {
            ImageError::IoError(o) => {
                Self::from(o)
            }
            _ => {
                Self::RuntimeError {
                    message: value.to_string(),
                }
            }
        }
    }
}