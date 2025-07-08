use crate::provider::prelude::*;
use serde::{Deserialize, Serialize};
use std::{
  fs,
  path::{Path, PathBuf},
  sync::OnceLock
};

const DOG_CEO_RANDOM_URL: &str = "https://dog.ceo/api/breeds/image/random";
const DOG_CEO_BREEDS_URL: &str = "https://dog.ceo/api/breeds/list/all";
const DOG_CEO_BREEDS_JSON: &str = "assets/data/dog_ceo_breeds.json";

#[derive(Deserialize, Debug)]
pub struct PhotoApiResponse {
  pub message: String,
  pub status: String
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct BreedsApiResponse {
  pub message: std::collections::HashMap<String, Vec<String>>,
  pub status: String
}

pub struct Provider;

// Static cache for breeds data to avoid repeated API calls
static BREEDS_CACHE: OnceLock<BreedsApiResponse> = OnceLock::new();

impl Provider {
  pub fn extract_breed_from_url(url: &str) -> Result<(String, Option<String>)> {
    debug!("Extracting breed from URL: {}", url);
    let parts: Vec<&str> = url
      .split("/breeds/")
      .nth(1)
      .ok_or_else(|| Error::Url(format!("URL missing breed segment: {url}")))?
      .split('/')
      .next()
      .ok_or_else(|| {
        Error::Url(format!("URL missing breed identifier: {url}"))
      })?
      .split('-')
      .collect();

    match parts.as_slice() {
      [main] => {
        debug!("Found main breed: {}", main);
        Ok((main.to_string(), None))
      }
      [main, sub] => {
        debug!("Found sub-breed: {} - {}", sub, main);
        Ok((main.to_string(), Some(sub.to_string())))
      }
      _ => Err(Error::BreedFormat(format!(
        "Expected 'main' or 'main-sub' format, got: {url}"
      )))
    }
  }

  pub async fn get_breeds<P: AsRef<Path>>(
    breeds_path: Option<P>,
    breeds_url: Option<&str>
  ) -> Result<BreedsApiResponse> {
    // Check static cache first
    if let Some(cached) = BREEDS_CACHE.get() {
      debug!("Using cached breeds data");
      return Ok((*cached).clone()); // Only clone when we actually need it
    }

    let path = breeds_path.map_or_else(
      || PathBuf::from(DOG_CEO_BREEDS_JSON),
      |p| p.as_ref().to_path_buf()
    );

    let url = breeds_url.unwrap_or(DOG_CEO_BREEDS_URL);
    debug!("Fetching breeds from: {}", url);

    // Check if we should use cached data from file
    if fs::metadata(&path).is_ok() {
      debug!("Using cached breeds file");
      let content = fs::read_to_string(&path)?;
      let breeds_data: BreedsApiResponse = serde_json::from_str(&content)?;

      // Cache in memory for next time
      let _ = BREEDS_CACHE.set(breeds_data.clone());
      return Ok(breeds_data);
    }

    debug!("Breeds file not found, downloading...");
    let response = reqwest::get(url).await?;
    let breeds_data: BreedsApiResponse = response.json().await?;

    // Cache the response both in memory and on disk
    let _ = BREEDS_CACHE.set(breeds_data.clone());

    // Save to file for persistence
    if let Some(parent) = path.parent() {
      fs::create_dir_all(parent)?;
    }
    let content = serde_json::to_string_pretty(&breeds_data)?;
    fs::write(&path, content)?;

    Ok(breeds_data)
  }

  pub fn verify_breed(
    main: &str,
    sub: &Option<String>,
    breeds: &BreedsApiResponse
  ) -> Result<()> {
    debug!("Verifying breed: {} {:?}", main, sub);
    let valid = match sub {
      Some(s) => breeds
        .message
        .get(main)
        .map(|subs| subs.contains(s))
        .unwrap_or(false),
      None => breeds.message.contains_key(main)
    };

    if !valid {
      let breed_name = sub
        .as_ref()
        .map_or(main.to_string(), |s| format!("{s} {main}"));
      warn!("Unrecognized breed: {}", breed_name);
      Err(Error::UnrecognizedBreed(breed_name))
    } else {
      debug!("Breed verified");
      Ok(())
    }
  }

  pub fn build_reference_url(
    breed: &str,
    sub_breed: &Option<String>
  ) -> String {
    match sub_breed {
      Some(sub) => format!("https://dog.ceo/api/breed/{breed}/{sub}"),
      None => format!("https://dog.ceo/api/breed/{breed}")
    }
  }

  async fn fetch_photo_from_url(&self, url: &str) -> Result<String> {
    debug!("Fetching photo from: {}", url);
    let response = reqwest::get(url).await?;
    let photo_response: PhotoApiResponse = response.json().await?;
    Ok(photo_response.message)
  }

  async fn breed_from_photo_url(&self, photo_url: &str) -> Result<Breed> {
    let (main, sub) = Self::extract_breed_from_url(photo_url)?;

    debug!("Getting breeds data");
    let breeds_data = Self::get_breeds(None::<PathBuf>, None).await?;

    let url_reference = Self::build_reference_url(&main, &sub);
    debug!("Reference URL: {url_reference}");

    let display_name = match Self::verify_breed(&main, &sub, &breeds_data) {
      Ok(_) => {
        debug!("Breed verified");
        Breed::format_name(&main, sub.as_deref())
      }
      Err(_) => {
        warn!("Breed not verified");
        format!("{} (unverified)", Breed::format_name(&main, sub.as_deref()))
      }
    };

    info!("Formatted breed: {}", display_name);
    Ok(Breed::new(main, sub, url_reference, display_name))
  }
}

#[async_trait]
impl Content for Provider {
  async fn photo(&self, source: DataSource<'_>) -> Result<String> {
    match source {
      DataSource::Url(url) => {
        // If it's already a direct image URL, return it
        if url.contains("images.dog.ceo") {
          Ok(url.to_string())
        } else {
          // Otherwise, fetch from the API
          self.fetch_photo_from_url(url).await
        }
      }
      DataSource::File(path) => {
        // Read JSON from file and extract photo URL
        let json = fetch_json(source).await?;
        json
          .get("message")
          .and_then(|v| v.as_str())
          .map(|s| s.to_string())
          .ok_or_else(|| Error::EmptyResponse)
      }
      DataSource::Raw(data) => {
        // Parse JSON from raw data
        let json: serde_json::Value = serde_json::from_slice(data)?;
        json
          .get("message")
          .and_then(|v| v.as_str())
          .map(|s| s.to_string())
          .ok_or_else(|| Error::EmptyResponse)
      }
    }
  }

  async fn breed(&self, source: DataSource<'_>) -> Result<Breed> {
    match source {
      DataSource::Url(url) => {
        // If it's a direct image URL, extract breed from it
        if url.contains("images.dog.ceo") {
          self.breed_from_photo_url(url).await
        } else {
          // Otherwise, fetch photo URL first, then extract breed
          let photo_url = self.photo(source).await?;
          self.breed_from_photo_url(&photo_url).await
        }
      }
      DataSource::File(_) | DataSource::Raw(_) => {
        // For file/raw data, get the photo URL first
        let photo_url = self.photo(source).await?;
        self.breed_from_photo_url(&photo_url).await
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use mockito::{mock, server_url};
  use tempfile::tempdir;

  const TEST_IMAGE_URL: &str =
    "https://images.dog.ceo/breeds/hound-afghan/n02088094_1003.jpg";
  const TEST_BREEDS_DATA: &str =
    r#"{"message":{"hound":["afghan"]},"status":"success"}"#;

  #[tokio::test]
  async fn test_photo_from_api() {
    let _m = mock("GET", "/api/breeds/image/random")
      .with_status(200)
      .with_header("content-type", "application/json")
      .with_body(format!(
        r#"{{"message":"{TEST_IMAGE_URL}","status":"success"}}"#
      ))
      .create();

    let provider = Provider;
    let test_url = format!("{}/api/breeds/image/random", server_url());
    let result = provider.photo(DataSource::Url(&test_url)).await;

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), TEST_IMAGE_URL);
  }

  #[tokio::test]
  async fn test_photo_direct_url() {
    let provider = Provider;
    let result = provider.photo(DataSource::Url(TEST_IMAGE_URL)).await;

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), TEST_IMAGE_URL);
  }

  #[tokio::test]
  async fn test_extract_breed_from_url() {
    let result = Provider::extract_breed_from_url(TEST_IMAGE_URL);
    assert!(result.is_ok());
    let (main, sub) = result.unwrap();
    assert_eq!(main, "hound");
    assert_eq!(sub, Some("afghan".to_string()));
  }

  #[tokio::test]
  async fn test_breed_from_direct_url() {
    let temp_dir = tempdir().unwrap();
    let breeds_path = temp_dir.path().join("breeds.json");

    // Create mock breeds file
    fs::write(&breeds_path, TEST_BREEDS_DATA).unwrap();

    let _m = mock("GET", "/api/breeds/list/all")
      .with_status(200)
      .with_body(TEST_BREEDS_DATA)
      .create();

    let provider = Provider;
    let result = provider.breed(DataSource::Url(TEST_IMAGE_URL)).await;

    assert!(result.is_ok());
    let breed = result.unwrap();
    assert_eq!(breed.main_breed, "hound");
    assert_eq!(breed.sub_breed, Some("afghan".to_string()));
    assert_eq!(breed.display_name, "Afghan Hound");
  }

  #[tokio::test]
  async fn test_breed_from_api() {
    let _photo_mock = mock("GET", "/api/breeds/image/random")
      .with_status(200)
      .with_body(format!(
        r#"{{"message":"{TEST_IMAGE_URL}","status":"success"}}"#
      ))
      .create();

    let _breeds_mock = mock("GET", "/api/breeds/list/all")
      .with_status(200)
      .with_body(TEST_BREEDS_DATA)
      .create();

    let provider = Provider;
    let test_url = format!("{}/api/breeds/image/random", server_url());
    let result = provider.breed(DataSource::Url(&test_url)).await;

    assert!(result.is_ok());
    let breed = result.unwrap();
    assert_eq!(breed.main_breed, "hound");
    assert_eq!(breed.sub_breed, Some("afghan".to_string()));
    assert_eq!(breed.display_name, "Afghan Hound");
  }
}
