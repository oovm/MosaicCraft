use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::path::{Path, PathBuf};

mod io_error;
mod image_error;
mod sled_error;

pub type MosaicResult<T> = Result<T, MosaicError>;

#[derive(Debug)]
pub enum MosaicError {
    IOError {
        path: PathBuf,
        message: String,
    },
    RuntimeError {
        message: String,
    },
}

impl Display for MosaicError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            MosaicError::IOError { path, message } => {
                write!(f, "IO Error: {} {}", path.display(), message)
            }
            MosaicError::RuntimeError { message } => {
                write!(f, "Runtime Error: {}", message)
            }
        }
    }
}

impl Error for MosaicError {}