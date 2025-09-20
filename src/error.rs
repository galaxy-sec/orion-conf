pub use derive_getters::Getters;
use orion_error::{ErrorCode, UvsReason};
pub use orion_error::{ErrorOwe, ErrorWith, StructError, UvsConfFrom};
pub use serde_derive::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Clone, Debug, Serialize, PartialEq, Error)]
pub enum ConfIOReason {
    // Preferred variant for arbitrary messages
    #[error("{0}")]
    Other(String),

    #[error("{0}")]
    Uvs(UvsReason),
    #[error("no format feature enabled - please enable at least one of: yaml, toml, json, ini")]
    NoFormatEnabled,
}

// Backward-compat deprecated alias
#[deprecated(since = "0.3.0", note = "Use ConfIOReason")]
pub use ConfIOReason as StorageReason;

// Keep legacy alias for compatibility
pub type SerdeReason = ConfIOReason;

impl ErrorCode for ConfIOReason {
    fn error_code(&self) -> i32 {
        match self {
            ConfIOReason::Other(_) => 500,
            ConfIOReason::Uvs(r) => r.error_code(),
            ConfIOReason::NoFormatEnabled => 501,
        }
    }
}

impl From<String> for ConfIOReason {
    fn from(s: String) -> Self {
        ConfIOReason::Other(s)
    }
}

impl From<UvsReason> for ConfIOReason {
    fn from(r: UvsReason) -> Self {
        ConfIOReason::Uvs(r)
    }
}

#[deprecated(since = "0.3.0", note = "Use OrionConfResult<T>")]
pub type SerdeResult<T> = Result<T, StructError<ConfIOReason>>;
#[deprecated(since = "0.3.0", note = "Use OrionConfError")]
pub type SerdeError = StructError<ConfIOReason>;

pub type OrionConfResult<T> = Result<T, StructError<ConfIOReason>>;
pub type OrionConfError = StructError<ConfIOReason>;

#[deprecated(since = "0.3.0", note = "Use OrionConfResult<T>")]
pub type StorageResult<T> = Result<T, StructError<ConfIOReason>>;
#[deprecated(since = "0.3.0", note = "Use OrionConfError")]
pub type StorageError = StructError<ConfIOReason>;
