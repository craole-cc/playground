pub use crate::prelude::*;
use crate::utils::format::capitalize_first_letter;
pub use async_trait::async_trait;
pub use std::path::Path;
use tokio::fs;

// Generic source enum for any data type
#[derive(Debug, Clone)]
pub enum DataSource<'a> {
  Url(&'a str),
  File(&'a Path),
  Raw(&'a [u8])
}

// Simplified Content trait - providers handle their own parsing
#[async_trait]
pub trait Content {
  async fn photo(&self, source: DataSource<'_>) -> Result<String>;
  async fn breed(&self, source: DataSource<'_>) -> Result<Breed>;
}

#[derive(Debug)]
pub struct Breed {
  pub main_breed: String,
  pub sub_breed: Option<String>,
  pub url_reference: String,
  pub display_name: String
}

impl Breed {
  pub fn new(
    main_breed: impl Into<String>,
    sub_breed: Option<impl Into<String>>,
    url_reference: impl Into<String>,
    display_name: impl Into<String>
  ) -> Self {
    Self {
      main_breed: main_breed.into(),
      sub_breed: sub_breed.map(Into::into),
      url_reference: url_reference.into(),
      display_name: display_name.into()
    }
  }

  pub fn format_name(main: &str, sub: Option<&str>) -> String {
    match sub {
      Some(s) => format!("{} {}", capitalize(s), capitalize(main)),
      None => capitalize(main)
    }
  }
}

// Helper functions for common parsing patterns
pub async fn fetch_data(source: DataSource<'_>) -> Result<Vec<u8>> {
  match source {
    DataSource::Url(url) => {
      let response = reqwest::get(url).await?;
      Ok(response.bytes().await?.to_vec())
    }
    DataSource::File(path) => Ok(fs::read(path).await?),
    DataSource::Raw(data) => Ok(data.to_vec())
  }
}

pub async fn fetch_json(source: DataSource<'_>) -> Result<serde_json::Value> {
  let data = fetch_data(source).await?;
  Ok(serde_json::from_slice(&data)?)
}

pub async fn fetch_text(source: DataSource<'_>) -> Result<String> {
  let data = fetch_data(source).await?;
  Ok(String::from_utf8_lossy(&data).to_string())
}
