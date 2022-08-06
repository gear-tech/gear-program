//! Metadata result

/// Metadata error
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Memory not exists")]
    MemoryNotExists,
    #[error("Metadata {0} not exists")]
    MetadataNotExists(String),
    #[error(transparent)]
    Anyhow(#[from] anyhow::Error),
}

/// Metadata result
pub type Result<T> = std::result::Result<T, Error>;
