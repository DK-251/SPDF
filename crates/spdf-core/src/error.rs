use thiserror::Error;

/// Top-level error type for all SPDF core operations.
#[derive(Debug, Error)]
pub enum SpdfError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON serialization error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("ZIP container error: {0}")]
    Zip(#[from] zip::result::ZipError),

    #[error("manifest error: {0}")]
    Manifest(String),

    #[error("validation error: {0}")]
    Validation(String),

    #[error("invalid state transition: {from:?} -> {to:?}")]
    InvalidStateTransition {
        from: crate::types::DocumentState,
        to: crate::types::DocumentState,
    },

    #[error("element not found: {0}")]
    ElementNotFound(String),

    #[error("checksum mismatch: expected {expected}, got {actual}")]
    ChecksumMismatch { expected: String, actual: String },

    #[error("unsupported SPDF version: {0}")]
    UnsupportedVersion(String),

    #[error("container entry missing: {0}")]
    MissingEntry(String),

    #[error("file size exceeds limit: {size} bytes (max: {max} bytes)")]
    FileSizeExceeded { size: u64, max: u64 },

    #[error("decompression ratio exceeded: {ratio:.2} (max: {max:.2})")]
    DecompressionBomb { ratio: f64, max: f64 },

    #[error("signing error: {0}")]
    Signing(String),

    #[error("wrong document state: expected {expected:?}, got {actual:?}")]
    WrongState {
        expected: crate::types::DocumentState,
        actual: crate::types::DocumentState,
    },

    #[error("redaction error: {0}")]
    Redaction(String),
}

pub type SpdfResult<T> = Result<T, SpdfError>;
