use super::{prelude::*, *};
use std::{
  collections::hash_map::DefaultHasher,
  hash::{Hash, Hasher},
  time::{SystemTime, UNIX_EPOCH}
};

// The trait that all API providers must implement
pub trait Content {
  async fn photo(&self, url: Option<&str>) -> Result<String>;
  fn breed(&self, photo_url: &str) -> String;
}

// Main Provider enum that uses the trait implementations
#[derive(Debug, Default, Clone)]
pub enum Provider {
  #[default]
  DogCeo,
  Random,
  Custom(String)
}

impl Provider {
  // Get all known providers (excluding Custom and Random to avoid recursion)
  fn known_providers() -> Vec<Provider> {
    vec![
      Provider::DogCeo,
      // Add other known providers here as you implement them
      // Do NOT include Provider::Random here to avoid recursion
    ]
  }

  // Select a random provider from known providers
  fn select_random() -> Provider {
    let known = Self::known_providers();
    if known.is_empty() {
      return Provider::DogCeo; // fallback
    }

    // Simple pseudo-random selection using system time
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
      Provider::DogCeo => dog_ceo::Provider.photo(url).await,
      Provider::Random => {
        //? Randomly select and call implementations from known providers
        let random_provider = Self::select_random();
        match random_provider {
          Provider::DogCeo => dog_ceo::Provider.photo(url).await,
          Provider::Custom(base_url) =>
            custom::Provider::new(base_url).photo(url).await,
          Provider::Random => {
            //? As a failsafe fallback to DogCeo
            dog_ceo::Provider.photo(url).await
          }
        }
      }
      Provider::Custom(base_url) =>
        custom::Provider::new(base_url.clone()).photo(url).await,
    }
  }

  pub fn breed(&self, photo_url: &str) -> String {
    match self {
      Provider::DogCeo => dog_ceo::Provider.breed(photo_url),
      Provider::Random => {
        // For breed extraction, we need to determine which provider was
        // actually used This is a bit tricky since we don't know which
        // random provider fetched the photo We could either:
        // 1. Try to infer from the URL structure
        // 2. Store the actual provider used during fetch
        // 3. Use a default approach

        // For now, let's try to infer from URL structure
        if photo_url.contains("dog.ceo") {
          dog_ceo::Provider.breed(photo_url)
        } else {
          "Unknown".to_string()
        }
      }
      Provider::Custom(base_url) =>
        custom::Provider::new(base_url.clone()).breed(photo_url),
    }
  }

  // Convenience constructors
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
