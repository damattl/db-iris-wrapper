use std::num::ParseIntError;

use thiserror::Error;



#[derive(Debug, Error)]
pub enum PortError {
    #[error("not found")]
    NotFound,
    #[error("conflict")]
    Conflict,         // e.g., unique violation
    #[error("invalid input")]
    InvalidInput,
    #[error("transient failure")]
    Transient,        // timeouts, network hiccups, serialization retry
    #[error("unavailable")]
    Unavailable,                    // DB down / pool closed
    #[error("unexpected")]
    Unexpected,
    #[error(transparent)]
    MappingError(#[from] MappingError),
}

#[derive(Debug, Error)]
pub enum MappingError {
    #[error(transparent)]
    ParseIntError(#[from] ParseIntError),
    #[error("missing value: {0}")]
    MissingValue(&'static str),
}
