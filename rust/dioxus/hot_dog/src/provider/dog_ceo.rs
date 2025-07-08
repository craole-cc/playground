use crate::provider::prelude::*;
use serde::{Deserialize, Serialize};
use std::{
  collections::HashMap,
  fs,
  path::{Path, PathBuf},
  sync::OnceLock
};

const URL: &str = "https://dog.ceo";
const AST: &str = "assets/data";
const API_RANDOM: &str = "api/breeds/image/random";
const API_BREEDS: &str = "api/breeds/list/all";
const AST_BREEDS: &str = "dog_ceo_breeds.json";

#[derive(Deserialize, Debug)]
pub struct PhotoApiResponse {
  pub message: String,
  pub status: String
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct BreedsApiResponse {
  pub message: HashMap<String, Vec<String>>,
  pub status: String
}
//~@ Define and use a static cache for breeds data to avoid repeated API calls
static BREEDS_CACHE: OnceLock<BreedsApiResponse> = OnceLock::new();

pub struct Provider;

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
    //~@ Check static cache first
    if let Some(cached) = BREEDS_CACHE.get() {
      debug!("Using cached breeds data");
      return Ok(
        //~@ Clone and return the cached data
        (*cached).clone()
      );
    }

    //~@ Prepare paths and URLs
    let json_cache = format! {"{AST}/{AST_BREEDS}"};
    let api_breeds = format! {"{URL}/{API_BREEDS}"};
    let path = breeds_path
      .map_or_else(|| PathBuf::from(json_cache), |p| p.as_ref().to_path_buf());
    let url = breeds_url.unwrap_or(&api_breeds);
    debug!("Fetching breeds from: {url}");

    //~@ Check if the breeds list is cached
    if fs::metadata(&path).is_ok() {
      debug!("Using cached breeds file");
      let content = fs::read_to_string(&path)?;
      let breeds_data: BreedsApiResponse = serde_json::from_str(&content)?;

      //~@ Cache in memory for next time
      let _ = BREEDS_CACHE.set(breeds_data.clone());
      return Ok(breeds_data);
    }

    //~@ Cache the response both in memory and on disk
    debug!("Breeds file not found, downloading...");
    let response = reqwest::get(url).await?;
    let breeds_data: BreedsApiResponse = response.json().await?;
    let _ = BREEDS_CACHE.set(breeds_data.clone());

    //~@ Save to file for persistence
    if let Some(parent) = path.parent() {
      fs::create_dir_all(parent)?;
    }
    let content = serde_json::to_string_pretty(&breeds_data)?;
    fs::write(&path, content)?;

    //~@ Return the fetched data
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
    let url_ref = match sub_breed {
      Some(sub) => format!("https://dog.ceo/api/breed/{breed}/{sub}"),
      None => format!("https://dog.ceo/api/breed/{breed}")
    };
    error!("{:#?}", &url_ref);
    url_ref
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
      Ok(_) => Breed::format_name(&main, sub.as_deref()),
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
        //~@ If it's already a direct image URL, return it
        if url.contains("images.dog.ceo") {
          Ok(url.to_string())
        } else {
          //~@ Otherwise, fetch from the API
          self.fetch_photo_from_url(url).await
        }
      }
      DataSource::File(path) => {
        //~@ Read JSON from file and extract photo URL
        let json = fetch_json(source).await?;
        json
          .get("message")
          .and_then(|v| v.as_str())
          .map(|s| s.to_string())
          .ok_or_else(|| Error::EmptyResponse)
      }
      DataSource::Raw(data) => {
        //~@ Parse JSON from raw data
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
        //~@ If it's a direct image URL, extract breed from it
        if url.contains("images.dog.ceo") {
          self.breed_from_photo_url(url).await
        } else {
          //~@ Otherwise, fetch photo URL first, then extract breed
          let photo_url = self.photo(source).await?;
          self.breed_from_photo_url(&photo_url).await
        }
      }
      DataSource::File(_) | DataSource::Raw(_) => {
        //~@ For file/raw data, get the photo URL first
        let photo_url = self.photo(source).await?;
        self.breed_from_photo_url(&photo_url).await
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use mockito::Server;
  use tempfile::tempdir;

  const TEST_PHOTO_1: &str =
    "https://images.dog.ceo/breeds/hound-afghan/n02088094_1003.jpg";
  const TEST_BREED_1: &str = "hound";
  const TEST_SUB_BREED_1: &str = "afghan";

  const TEST_PHOTO_2: &str = "https://images.dog.ceo/breeds/sheepdog-english/Finnigan_Old_English_Sheepdog_sml.jpg";
  const TEST_BREED_2: &str = "sheepdog";
  const TEST_SUB_BREED_2: &str = "english";

  const TEST_BREEDS_DATA: &str =
    r#"{"message":{"hound":["afghan"]},"status":"success"}"#;

  #[tokio::test]
  async fn test_photo_from_api() {
    //~@ Initialize the logger
    log::testing::init();

    //~@ Request a new server from the pool
    let mut server = Server::new_async().await;

    let url = format!("{}/{}", server.url(), API_RANDOM);
    trace!("URL: {}", &url);

    //~@ Mock the API endpoint that dog_ceo::Provider will call
    let mock = server
      .mock("GET", &*format!("/{API_RANDOM}"))
      .with_status(200)
      .with_header("content-type", "application/json")
      .with_body(format!(
        r#"{{"message":"{TEST_PHOTO_1}","status":"success"}}"#
      ))
      .create_async()
      .await;
    trace!("{:#?}", &mock);

    //~@ Call the provider with the mock server URL
    let result = Provider.photo(DataSource::Url(&url)).await;
    info!("Result: {:?}", &result);

    //~@ Ensure the mock was called
    mock.assert_async().await;
  }
}
