use super::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Config {
  pub photo: String,
  pub breed: String
}

impl Default for Config {
  fn default() -> Self {
    Self {
      photo: String::new(),
      breed: "Unknown".to_string()
    }
  }
}

impl Config {
  /// Fetch a photo using the specified provider
  pub async fn fetch_with_provider(
    provider: Provider,
    url: Option<&str>
  ) -> Result<Self> {
    let photo = provider.photo(url).await?;
    let breed = provider.breed(&photo);
    Ok(Config { photo, breed })
  }

  /// Fetch a photo using the default provider (Dog CEO)
  pub async fn fetch(url: Option<&str>) -> Result<Self> {
    Self::fetch_with_provider(Provider::default(), url).await
  }

  /// Fetch from Dog CEO API specifically
  pub async fn fetch_dog_ceo(url: Option<&str>) -> Result<Self> {
    Self::fetch_with_provider(Provider::dog_ceo(), url).await
  }

  /// Fetch from Random provider (randomly selects from known providers)
  pub async fn fetch_random(url: Option<&str>) -> Result<Self> {
    Self::fetch_with_provider(Provider::random(), url).await
  }

  /// Fetch from a custom API
  pub async fn fetch_custom<S: Into<String>>(
    base_url: S,
    url: Option<&str>
  ) -> Result<Self> {
    Self::fetch_with_provider(Provider::custom(base_url), url).await
  }

  /// Create config from existing photo URL using specified provider for breed
  /// extraction
  pub fn from_photo_url_with_provider(
    photo: String,
    provider: Provider
  ) -> Self {
    let breed = provider.breed(&photo);
    Self { photo, breed }
  }

  /// Create config from existing photo URL using default provider for breed
  /// extraction
  pub fn from_photo_url(photo: String) -> Self {
    Self::from_photo_url_with_provider(photo, Provider::default())
  }

  /// Create config with explicit breed (no extraction)
  pub fn new(photo: String, breed: String) -> Self {
    Self { photo, breed }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_config_default() {
    let config = Config::default();
    assert_eq!(config.photo, "");
    assert_eq!(config.breed, "Unknown");
  }

  #[test]
  fn test_config_new() {
    let config = Config::new(
      "https://example.com/dog.jpg".to_string(),
      "Golden Retriever".to_string()
    );
    assert_eq!(config.photo, "https://example.com/dog.jpg");
    assert_eq!(config.breed, "Golden Retriever");
  }

  #[test]
  fn test_config_clone() {
    let config1 = Config::new(
      "https://example.com/dog.jpg".to_string(),
      "Labrador".to_string()
    );
    let config2 = config1.clone();
    assert_eq!(config1, config2);
  }

  #[test]
  fn test_config_partial_eq() {
    let config1 = Config::new(
      "https://example.com/dog.jpg".to_string(),
      "Labrador".to_string()
    );
    let config2 = Config::new(
      "https://example.com/dog.jpg".to_string(),
      "Labrador".to_string()
    );
    let config3 = Config::new(
      "https://example.com/other.jpg".to_string(),
      "Labrador".to_string()
    );

    assert_eq!(config1, config2);
    assert_ne!(config1, config3);
  }

  #[test]
  fn test_config_ordering() {
    let config1 = Config::new(
      "https://example.com/a.jpg".to_string(),
      "Beagle".to_string()
    );
    let config2 = Config::new(
      "https://example.com/b.jpg".to_string(),
      "Labrador".to_string()
    );

    assert!(config1 < config2);
  }

  #[test]
  fn test_provider_constructors() {
    let dog_ceo = Provider::dog_ceo();
    let random = Provider::random();
    let custom = Provider::custom("https://my-api.com");

    assert!(matches!(dog_ceo, Provider::DogCeo));
    assert!(matches!(random, Provider::Random));
    assert!(matches!(custom, Provider::Custom(_)));
  }

  #[test]
  fn test_from_photo_url_with_provider() {
    let photo_url =
      "https://images.dog.ceo/breeds/hound-afghan/n02088094_1003.jpg"
        .to_string();
    let config = Config::from_photo_url_with_provider(
      photo_url.clone(),
      Provider::dog_ceo()
    );
    assert_eq!(config.photo, photo_url);
    assert_eq!(config.breed, "Afghan Hound");
  }

  #[test]
  fn test_from_photo_url_default() {
    let photo_url =
      "https://images.dog.ceo/breeds/retriever-golden/n02099601_100.jpg"
        .to_string();
    let config = Config::from_photo_url(photo_url.clone());
    assert_eq!(config.photo, photo_url);
    assert_eq!(config.breed, "Golden Retriever");
  }

  // Integration tests for actual API calls
  #[cfg(test)]
  mod integration {
    // use super::*;

    #[test]
    #[ignore] // Ignore by default since it requires network access
    fn test_fetch_dog_ceo_integration() {
      // This would require an async test runner
      // For dioxus, you'd use your existing async runtime
    }

    #[test]
    #[ignore] // Ignore by default since it requires network access
    fn test_fetch_random_integration() {
      // This would require an async test runner
      // For dioxus, you'd use your existing async runtime
    }
  }
}

// Usage examples:
//
// // Using default provider (Dog CEO)
// let config = Config::fetch(None).await?;
//
// // Using specific providers
// let config = Config::fetch_dog_ceo(None).await?;
// let config = Config::fetch_random(None).await?;
// let config = Config::fetch_custom("https://my-api.com/photos", None).await?;
//
// // Using provider enum directly
// let config = Config::fetch_with_provider(Provider::DogCeo, None).await?;
// let config = Config::fetch_with_provider(Provider::Random, None).await?;
// let config = Config::fetch_with_provider(Provider::custom("https://my-api.com"), None).await?;
//
// // From existing URL
// let config = Config::from_photo_url(photo_url);
// let config = Config::from_photo_url_with_provider(photo_url,
// Provider::Random);
