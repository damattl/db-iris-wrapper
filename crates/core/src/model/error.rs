use std::num::ParseIntError;

use thiserror::Error;

// deprecated?
#[derive(Debug, Error)]
pub enum MappingError {
    #[error(transparent)]
    ParseIntError(#[from] ParseIntError),
    #[error("missing value: {0}")]
    MissingValue(&'static str),
}
