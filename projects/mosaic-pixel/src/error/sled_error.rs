use sled::Error;

use crate::MosaicError;

impl From<Error> for MosaicError {
    fn from(value: Error) -> Self {
        match value {
            Error::Io(e) => {
                Self::from(e)
            }
            _ => {
                Self::RuntimeError {
                    message: value.to_string(),
                }
            }
        }
    }
}