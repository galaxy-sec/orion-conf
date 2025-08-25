pub use derive_getters::Getters;
use derive_more::From;
use orion_error::{ErrorCode, UvsReason};
pub use orion_error::{ErrorOwe, ErrorWith, StructError, UvsConfFrom};
pub use serde_derive::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Clone, Debug, Serialize, PartialEq, Error, From)]
pub enum StorageReason {
    #[error("brief {0}")]
    Brief(String),
    #[error("{0}")]
    Uvs(UvsReason),
}
pub type SerdeReason = StorageReason;

impl ErrorCode for StorageReason {
    fn error_code(&self) -> i32 {
        match self {
            StorageReason::Brief(_) => 500,
            StorageReason::Uvs(r) => r.error_code(),
        }
    }
}
pub type SerdeResult<T> = Result<T, StructError<StorageReason>>;
pub type SerdeError = StructError<StorageReason>;

pub type StorageResult<T> = Result<T, StructError<StorageReason>>;
pub type StorageError = StructError<StorageReason>;
