use std::path::{Path, PathBuf};
mod io_error;
mod image_error;
mod sled_error;

pub type MosaicResult<T> = Result<T, MosaicError>;

pub enum MosaicError {
    IOError {
        path: PathBuf,
        message: String,
    },
    RuntimeError {
        message: String,
    },
}

