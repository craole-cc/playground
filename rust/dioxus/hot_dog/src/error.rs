pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
  #[error("Network error: {0}")]
  Network(#[from] reqwest::Error),
  #[error("API returned no response")]
  NoResponse
}
