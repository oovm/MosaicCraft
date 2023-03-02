use std::io::Error;

use super::*;

impl From<Error> for MosaicError {
    fn from(value: Error) -> Self {
        Self::IOError {
            path: PathBuf::new(),
            message: value.to_string(),
        }
    }
}

impl MosaicError {
    pub fn with_path(mut self, new: &Path) -> Self {
        match self {
            Self::IOError { ref mut path, .. } => {
                *path = new.to_path_buf();
            }
        }
        self
    }
}