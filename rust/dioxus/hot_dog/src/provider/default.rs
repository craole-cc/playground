use super::{prelude::*, *};
use std::{
  collections::hash_map::DefaultHasher,
  hash::{Hash, Hasher},
  path::{Path, PathBuf},
  time::{SystemTime, UNIX_EPOCH}
};

#[derive(Debug, Default, Clone)]
pub enum Provider {
  #[default]
  DogCeo,
  Random,
  Custom(String)
}

impl Provider {
  fn known_providers() -> Vec<Provider> {
    vec![Provider::DogCeo]
  }

  fn select_random() -> Provider {
    let known = Self::known_providers();
    if known.is_empty() {
      return Provider::DogCeo;
    }

    let now = SystemTime::now()
      .duration_since(UNIX_EPOCH)
      .unwrap_or_default()
      .as_nanos();

    let mut hasher = DefaultHasher::new();
    now.hash(&mut hasher);
    let hash = hasher.finish();

    let index = (hash as usize) % known.len();
    known[index].clone()
  }

  pub async fn photo(&self, url: Option<&str>) -> Result<String> {
    match self {
      Provider::DogCeo => {
        let source = match url {
          Some(url_str) => DataSource::Url(url_str),
          None => DataSource::Url("https://dog.ceo/api/breeds/image/random")
        };
        dog_ceo::Provider.photo(source).await
      }
      Provider::Random => {
        let random_provider = Self::select_random();
        match random_provider {
          Provider::DogCeo => {
            let source = match url {
              Some(url_str) => DataSource::Url(url_str),
              None => DataSource::Url("https://dog.ceo/api/breeds/image/random")
            };
            dog_ceo::Provider.photo(source).await
          }
          Provider::Custom(base_url) => {
            let source = match url {
              Some(url_str) => DataSource::Url(url_str),
              None => DataSource::Url(&base_url)
            };
            custom::Provider::new(base_url.clone()).photo(source).await
          }
          Provider::Random => {
            // Prevent infinite recursion - fallback to DogCeo
            let source = match url {
              Some(url_str) => DataSource::Url(url_str),
              None => DataSource::Url("https://dog.ceo/api/breeds/image/random")
            };
            dog_ceo::Provider.photo(source).await
          }
        }
      }
      Provider::Custom(base_url) => {
        let source = match url {
          Some(url_str) => DataSource::Url(url_str),
          None => DataSource::Url(base_url)
        };
        custom::Provider::new(base_url.clone()).photo(source).await
      }
    }
  }

  pub async fn breed<P: AsRef<Path> + Send + Sync>(
    &self,
    photo_url: Option<&str>,
    breeds_url: Option<&str>,
    breeds_path: Option<P>
  ) -> Result<Breed> {
    match self {
      Provider::DogCeo => {
        let source = match photo_url {
          Some(url_str) => DataSource::Url(url_str),
          None => DataSource::Url("https://dog.ceo/api/breeds/image/random")
        };
        dog_ceo::Provider.breed(source).await
      }
      Provider::Random => {
        if let Some(url) = photo_url {
          if url.contains("dog.ceo") {
            let source = DataSource::Url(url);
            return dog_ceo::Provider.breed(source).await;
          }
        }
        Ok(Breed::new(
          String::from("Unknown"),
          None::<String>,
          String::from(""),
          String::from("Unknown Breed")
        ))
      }
      Provider::Custom(base_url) => {
        let source = match photo_url {
          Some(url_str) => DataSource::Url(url_str),
          None => DataSource::Url(base_url)
        };
        custom::Provider::new(base_url.clone()).breed(source).await
      }
    }
  }

  pub fn dog_ceo() -> Self {
    Provider::DogCeo
  }

  pub fn random() -> Self {
    Provider::Random
  }

  pub fn custom<S: Into<String>>(base_url: S) -> Self {
    Provider::Custom(base_url.into())
  }
}

#[cfg(test)]
mod tests {
  use super::{prelude::*, *};
  use mockito::Server;
  use std::fs;
  use tempfile::tempdir;

  #[tokio::test]
  async fn test_dog_ceo_photo() {
    // Initialize the logger
    log::testing::init();

    // Start a new mock server
    let mut server = Server::new_async().await;

    // Use the server URL for the provider
    // let url = server.url();

    // Mock the API endpoint that dog_ceo::Provider will call
    let mock = server
            .mock("GET", "/api/breeds/image/random")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{"message":"https://images.dog.ceo/breeds/hound-afghan/n02088094_1003.jpg","status":"success"}"#
            )
            .create();

    let provider = Provider::dog_ceo();

    // Call the provider with the mock server URL
    let base_url = server.url();
    let full_url = format!("{base_url}/api/breeds/image/random");
    let result = provider.photo(Some(&full_url)).await;
    // let result = provider.photo(Some(&url)).await;

    // Ensure the mock was called
    mock.assert_async().await;

    // Assert the result is as expected
    assert!(result.is_ok());
    assert_eq!(
      result.unwrap(),
      "https://images.dog.ceo/breeds/hound-afghan/n02088094_1003.jpg"
    );

    // let _m = mock("GET", "/api/breeds/image/random")
    //   .with_status(200)
    //   .with_header("content-type", "application/json")
    //   .with_body(
    //     r#"{"message":"https://images.dog.ceo/breeds/hound-afghan/n02088094_1003.jpg","status":"success"}"#
    //   )
    //   .create();

    // let provider = Provider::dog_ceo();
    // eprintln!("Provider: {:?}", &provider);
    // info!("Provider: {:?}", &provider);
    // Pass the mock server URL to the provider
    // let test_url = format!("{}/api/breeds/image/random",
    // mockito::server_url());
    // let result = provider.photo(Some(&test_url)).await;
    // mock.assert_async().await;

    // assert!(result.is_ok());
    // assert_eq!(
    //   result.unwrap(),
    //   "https://images.dog.ceo/breeds/hound-afghan/n02088094_1003.jpg"
    // );
  }

  // #[tokio::test]
  // async fn test_random_provider_photo() {
  //   let provider = Provider::random();
  //   let result = provider.photo(None).await;
  //   assert!(result.is_ok());
  // }

  // #[tokio::test]
  // async fn test_custom_provider_photo() {
  //   let _m = mock("GET", "/")
  //     .with_status(200)
  //     .with_header("content-type", "application/json")
  //     .with_body(r#"{"url":"https://custom.com/dog.jpg"}"#)
  //     .create();

  //   let provider = Provider::custom(mockito::server_url());
  //   let result = provider.photo(None).await;

  //   assert!(result.is_ok());
  //   assert_eq!(result.unwrap(), "https://custom.com/dog.jpg");
  // }

  // #[tokio::test]
  // async fn test_dog_ceo_breed() {
  //   let temp_dir = tempdir().unwrap();
  //   let breeds_path = temp_dir.path().join("breeds.json");

  //   // Create the breeds file with test data BEFORE running the test
  //   let breeds_data =
  // r#"{"message":{"hound":["afghan"]},"status":"success"}"#;   fs::write(&
  // breeds_path, breeds_data).unwrap();

  //   // Mock the photo API endpoint
  //   let _m = mock("GET", "/api/breeds/image/random")
  //     .with_status(200)
  //     .with_header("content-type", "application/json")
  //     .with_body(r#"{"message":"https://images.dog.ceo/breeds/hound-afghan/n02088094_1003.jpg","status":"success"}"#)
  //     .create();

  //   let provider = Provider::dog_ceo();
  //   let test_url = format!("{}/api/breeds/image/random",
  // mockito::server_url());   let result = provider
  //     .breed(
  //       Some(&test_url),
  //       None, // breeds_url - not used in this test
  //       Some(&breeds_path)
  //     )
  //     .await;

  //   assert!(result.is_ok());
  //   let breed = result.unwrap();
  //   assert_eq!(breed.display_name, "Afghan Hound");
  //   assert_eq!(breed.main_breed, "hound");
  //   assert_eq!(breed.sub_breed, Some("afghan".to_string()));
  // }
}
