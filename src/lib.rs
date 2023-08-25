pub mod http;
#[allow(missing_docs)]
pub mod json;

/// An error that can occur during span creation
#[allow(missing_docs)]
#[derive(Debug, thiserror::Error)]
pub enum SpanError {
    #[error("Error during parsing")]
    ParseError,
    #[error("Found invalid ranges")]
    InvalidRange,
    #[error("Custom error: {0}")]
    Custom(String),
}
