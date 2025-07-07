#[derive(Debug, thiserror::Error)]
pub enum Error {
  #[error("Network request failed: {0}")]
  Network(#[from] reqwest::Error),

  #[error("API returned empty response")]
  EmptyResponse,

  #[error("Filesystem operation failed: {0}")]
  Filesystem(#[from] std::io::Error),

  #[error("JSON parsing failed: {0}")]
  Json(#[from] serde_json::Error),

  #[error("Malformed URL: {0}")]
  Url(String),

  #[error("Invalid breed format: {0}")]
  BreedFormat(String),

  #[error("Unrecognized breed: {0}")]
  UnrecognizedBreed(String),

  #[error("Provider-specific issue: {0}")]
  Provider(String)
}

pub type Result<T> = std::result::Result<T, Error>;
