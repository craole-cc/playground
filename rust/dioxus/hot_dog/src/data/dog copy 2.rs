use super::prelude::{
  async_trait, capitalize, Deserialize, HashMap, Serialize
};

// Simple result struct that users get
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DogInfo {
  pub photo_url: String,
  pub breed: String,
  pub sub_breed: Option<String>,
  pub display_name: String,
  pub reference_url: String
}

impl DogInfo {
  pub fn new(
    photo_url: String,
    breed: String,
    sub_breed: Option<String>
  ) -> Self {
    let display_name = match &sub_breed {
      Some(sub) => format!("{} {}", capitalize(sub), capitalize(&breed)),
      None => capitalize(&breed)
    };

    let reference_url = match &sub_breed {
      Some(sub) => format!("https://dog.ceo/api/breed/{breed}/{sub}"),
      None => format!("https://dog.ceo/api/breed/{breed}")
    };

    Self {
      photo_url,
      breed,
      sub_breed,
      display_name,
      reference_url
    }
  }
}

// Trait ensures all providers follow the same rules
#[async_trait]
pub trait Provider {
  async fn get_random_photo(&self) -> Result<String, ProviderError>;
  async fn get_breed_photos(
    &self,
    breed: &str,
    sub_breed: Option<&str>,
    count: usize
  ) -> Result<Vec<String>, ProviderError>;
  async fn parse_breed_from_url(
    &self,
    url: &str
  ) -> Result<(String, Option<String>), ProviderError>;
  async fn get_available_breeds(
    &self
  ) -> Result<HashMap<String, Vec<String>>, ProviderError>;
}

// Dog CEO provider implementation
pub struct DogCeoProvider {
  base_url: String
}

impl DogCeoProvider {
  pub fn new() -> Self {
    Self {
      base_url: "https://dog.ceo/api".to_string()
    }
  }
}

#[async_trait]
impl Provider for DogCeoProvider {
  async fn get_random_photo(&self) -> Result<String, ProviderError> {
    let url = format!("{}/breeds/image/random", self.base_url);
    let response = reqwest::get(&url).await?;
    let json: serde_json::Value = response.json().await?;

    json
      .get("message")
      .and_then(|v| v.as_str())
      .map(|s| s.to_string())
      .ok_or(ProviderError::InvalidResponse(
        "No photo URL found".to_string()
      ))
  }

  async fn get_breed_photos(
    &self,
    breed: &str,
    sub_breed: Option<&str>,
    count: usize
  ) -> Result<Vec<String>, ProviderError> {
    let url = match sub_breed {
      Some(sub) => format!("{}/breed/{}/{}/images", self.base_url, breed, sub),
      None => format!("{}/breed/{}/images", self.base_url, breed)
    };

    let response = reqwest::get(&url).await?;
    let json: serde_json::Value = response.json().await?;

    let urls = json.get("message").and_then(|v| v.as_array()).ok_or(
      ProviderError::InvalidResponse("No images found".to_string())
    )?;

    Ok(
      urls
        .iter()
        .take(count)
        .filter_map(|v| v.as_str())
        .map(|s| s.to_string())
        .collect()
    )
  }

  async fn parse_breed_from_url(
    &self,
    url: &str
  ) -> Result<(String, Option<String>), ProviderError> {
    // Extract breed from URL like: https://images.dog.ceo/breeds/hound-afghan/n02088094_1003.jpg
    let breed_part = url
      .split("/breeds/")
      .nth(1)
      .and_then(|s| s.split('/').next())
      .ok_or(ProviderError::ParseError(
        "Could not extract breed from URL".to_string()
      ))?;

    let parts: Vec<&str> = breed_part.split('-').collect();
    match parts.as_slice() {
      [main] => Ok((main.to_string(), None)),
      [main, sub] => Ok((main.to_string(), Some(sub.to_string()))),
      _ => Err(ProviderError::ParseError(
        "Invalid breed format".to_string()
      ))
    }
  }

  async fn get_available_breeds(
    &self
  ) -> Result<HashMap<String, Vec<String>>, ProviderError> {
    let url = format!("{}/breeds/list/all", self.base_url);
    let response = reqwest::get(&url).await?;
    let json: serde_json::Value = response.json().await?;

    json
      .get("message")
      .and_then(|v| serde_json::from_value(v.clone()).ok())
      .ok_or(ProviderError::InvalidResponse(
        "Invalid breeds response".to_string()
      ))
  }
}

// Custom provider implementation
pub struct CustomProvider {
  base_url: String
}

impl CustomProvider {
  pub fn new(base_url: String) -> Self {
    Self { base_url }
  }
}

#[async_trait]
impl Provider for CustomProvider {
  async fn get_random_photo(&self) -> Result<String, ProviderError> {
    let response = reqwest::get(&self.base_url).await?;
    let json: serde_json::Value = response.json().await?;

    json
      .get("url")
      .or_else(|| json.get("photo"))
      .or_else(|| json.get("image"))
      .and_then(|v| v.as_str())
      .map(|s| s.to_string())
      .ok_or(ProviderError::InvalidResponse(
        "No photo URL found".to_string()
      ))
  }

  async fn get_breed_photos(
    &self,
    _breed: &str,
    _sub_breed: Option<&str>,
    _count: usize
  ) -> Result<Vec<String>, ProviderError> {
    Err(ProviderError::UnsupportedOperation(
      "Custom provider doesn't support breed-specific queries".to_string()
    ))
  }

  async fn parse_breed_from_url(
    &self,
    url: &str
  ) -> Result<(String, Option<String>), ProviderError> {
    // Try to extract breed from custom URL format
    let breed_str = url
      .split('/')
      .next_back()
      .and_then(|s| s.split('.').next())
      .ok_or(ProviderError::ParseError(
        "Could not extract breed".to_string()
      ))?;

    let parts: Vec<&str> = breed_str.split('-').collect();
    match parts.as_slice() {
      [sub, main] => Ok((main.to_string(), Some(sub.to_string()))),
      [main] => Ok((main.to_string(), None)),
      _ => Ok(("unknown".to_string(), None))
    }
  }

  async fn get_available_breeds(
    &self
  ) -> Result<HashMap<String, Vec<String>>, ProviderError> {
    Err(ProviderError::UnsupportedOperation(
      "Custom provider doesn't support breed listing".to_string()
    ))
  }
}

// Main Dog config struct - this is what users interact with
pub struct Dog {
  provider: Box<dyn Provider + Send + Sync>
}

impl Dog {
  // Factory methods for easy creation
  pub fn dog_ceo() -> Self {
    Self {
      provider: Box::new(DogCeoProvider::new())
    }
  }

  pub fn custom(base_url: impl Into<String>) -> Self {
    Self {
      provider: Box::new(CustomProvider::new(base_url.into()))
    }
  }

  // High-level API methods that users actually want
  pub async fn get_random(&self) -> Result<DogInfo, ProviderError> {
    let photo_url = self.provider.get_random_photo().await?;
    let (breed, sub_breed) =
      self.provider.parse_breed_from_url(&photo_url).await?;
    Ok(DogInfo::new(photo_url, breed, sub_breed))
  }

  pub async fn get_multiple(
    &self,
    count: usize
  ) -> Result<Vec<DogInfo>, ProviderError> {
    let mut dogs = Vec::new();

    for _ in 0..count {
      match self.get_random().await {
        Ok(dog) => dogs.push(dog),
        Err(e) => eprintln!("Failed to get dog: {e:?}")
      }
    }

    Ok(dogs)
  }

  pub async fn get_breed(
    &self,
    breed: &str,
    sub_breed: Option<&str>,
    count: usize
  ) -> Result<Vec<DogInfo>, ProviderError> {
    let photo_urls = self
      .provider
      .get_breed_photos(breed, sub_breed, count)
      .await?;

    let mut dogs = Vec::new();
    for url in photo_urls {
      dogs.push(DogInfo::new(
        url,
        breed.to_string(),
        sub_breed.map(|s| s.to_string())
      ));
    }

    Ok(dogs)
  }

  pub async fn get_breeds(
    &self
  ) -> Result<HashMap<String, Vec<String>>, ProviderError> {
    self.provider.get_available_breeds().await
  }

  pub async fn dog_info_from_url(
    &self,
    url: &str
  ) -> Result<DogInfo, ProviderError> {
    let (breed, sub_breed) = self.provider.parse_breed_from_url(url).await?;
    Ok(DogInfo::new(url.to_string(), breed, sub_breed))
  }

  // Convenience methods for common operations
  pub async fn get_random_hound(&self) -> Result<DogInfo, ProviderError> {
    let hounds = self.get_breed("hound", None, 1).await?;
    hounds
      .into_iter()
      .next()
      .ok_or(ProviderError::InvalidResponse(
        "No hounds found".to_string()
      ))
  }

  pub async fn get_random_breed(
    &self,
    breed: &str
  ) -> Result<DogInfo, ProviderError> {
    let dogs = self.get_breed(breed, None, 1).await?;
    dogs
      .into_iter()
      .next()
      .ok_or(ProviderError::InvalidResponse(
        "No dogs found for breed".to_string()
      ))
  }

  pub async fn get_all_breeds_sample(
    &self
  ) -> Result<Vec<DogInfo>, ProviderError> {
    let breeds = self.get_breeds().await?;
    let mut dogs = Vec::new();

    for (breed, sub_breeds) in breeds.iter().take(5) {
      // Limit to 5 for demo
      if sub_breeds.is_empty() {
        if let Ok(mut breed_dogs) = self.get_breed(breed, None, 1).await {
          dogs.append(&mut breed_dogs);
        }
      } else {
        for sub_breed in sub_breeds.iter().take(1) {
          // One sub-breed per main breed
          if let Ok(mut breed_dogs) =
            self.get_breed(breed, Some(sub_breed), 1).await
          {
            dogs.append(&mut breed_dogs);
          }
        }
      }
    }

    Ok(dogs)
  }
}

// Simple error type
#[derive(Debug, thiserror::Error)]
pub enum ProviderError {
  #[error("HTTP request failed: {0}")]
  Http(#[from] reqwest::Error),

  #[error("JSON parsing failed: {0}")]
  Json(#[from] serde_json::Error),

  #[error("Invalid response: {0}")]
  InvalidResponse(String),

  #[error("Parse error: {0}")]
  ParseError(String),

  #[error("Unsupported operation: {0}")]
  UnsupportedOperation(String)
}

// Usage examples - clean API with trait consistency!
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  // Super simple usage - users only see Dog struct
  let dog_api = Dog::dog_ceo();

  // Get a random dog - clean and simple
  let dog = dog_api.get_random().await?;
  println!("🐕 {}", dog.display_name);
  println!("📸 {}", dog.photo_url);

  // Get multiple dogs
  let dogs = dog_api.get_multiple(3).await?;
  for dog in dogs {
    println!("Got: {}", dog.display_name);
  }

  // Get specific breed
  let hounds = dog_api.get_breed("hound", Some("afghan"), 2).await?;
  for hound in hounds {
    println!("Afghan Hound: {}", hound.photo_url);
  }

  // Convenience methods
  let random_hound = dog_api.get_random_hound().await?;
  println!("Random hound: {}", random_hound.display_name);

  // Custom provider - same interface!
  let custom_api = Dog::custom("https://my-dog-api.com");
  match custom_api.get_random().await {
    Ok(dog) => println!("Custom dog: {}", dog.display_name),
    Err(e) => println!("Custom provider failed: {e}")
  }

  // Get sample from all breeds
  let samples = dog_api.get_all_breeds_sample().await?;
  println!("Got {} breed samples", samples.len());

  Ok(())
}

// // Testing is straightforward
// #[cfg(test)]
// mod tests {
//   use super::*;

//   #[tokio::test]
//   async fn test_simple_usage() {
//     let dog_api = Dog::dog_ceo();
//     let dog = dog_api.get_random().await.unwrap();

//     assert!(!dog.photo_url.is_empty());
//     assert!(!dog.breed.is_empty());
//     assert!(!dog.display_name.is_empty());
//     assert!(dog.photo_url.starts_with("https://"));
//   }

//   #[tokio::test]
//   async fn test_multiple_dogs() {
//     let dog_api = Dog::dog_ceo();
//     let dogs = dog_api.get_multiple(2).await.unwrap();

//     assert_eq!(dogs.len(), 2);
//     assert!(dogs.iter().all(|d| !d.photo_url.is_empty()));
//   }

//   #[tokio::test]
//   async fn test_breed_specific() {
//     let dog_api = Dog::dog_ceo();
//     let beagles = dog_api.get_breed("beagle", None, 1).await.unwrap();

//     assert!(!beagles.is_empty());
//     assert_eq!(beagles[0].breed, "beagle");
//     assert_eq!(beagles[0].display_name, "Beagle");
//   }

//   #[tokio::test]
//   async fn test_convenience_methods() {
//     let dog_api = Dog::dog_ceo();
//     let hound = dog_api.get_random_hound().await.unwrap();

//     assert_eq!(hound.breed, "hound");
//     assert!(hound.display_name.contains("Hound"));
//   }

//   #[tokio::test]
//   async fn test_dog_info_from_url() {
//     let dog_api = Dog::dog_ceo();
//     let url = "https://images.dog.ceo/breeds/hound-afghan/n02088094_1003.jpg";
//     let dog = dog_api.dog_info_from_url(url).await.unwrap();

//     assert_eq!(dog.breed, "hound");
//     assert_eq!(dog.sub_breed, Some("afghan".to_string()));
//     assert_eq!(dog.display_name, "Afghan Hound");
//   }
// }

// // CLI example
// use clap::{App, Arg};

// #[tokio::main]
// async fn cli_main() -> Result<(), Box<dyn std::error::Error>> {
//   let matches = App::new("dog-fetcher")
//     .version("1.0")
//     .about("Simple dog photo fetcher")
//     .arg(
//       Arg::with_name("count")
//         .short("c")
//         .long("count")
//         .value_name("COUNT")
//         .help("Number of dogs to fetch")
//         .takes_value(true)
//         .default_value("1")
//     )
//     .arg(
//       Arg::with_name("breed")
//         .short("b")
//         .long("breed")
//         .value_name("BREED")
//         .help("Specific breed to fetch")
//         .takes_value(true)
//     )
//     .arg(
//       Arg::with_name("custom")
//         .long("custom")
//         .value_name("URL")
//         .help("Use custom provider URL")
//         .takes_value(true)
//     )
//     .get_matches();

//   let dog_api = match matches.value_of("custom") {
//     Some(url) => Dog::custom(url),
//     None => Dog::dog_ceo()
//   };

//   let count: usize = matches.value_of("count").unwrap().parse()?;

//   if let Some(breed) = matches.value_of("breed") {
//     let dogs = dog_api.get_breed(breed, None, count).await?;
//     for dog in dogs {
//       println!("🐕 {} - {}", dog.display_name, dog.photo_url);
//     }
//   } else {
//     let dogs = dog_api.get_multiple(count).await?;
//     for dog in dogs {
//       println!("🐕 {} - {}", dog.display_name, dog.photo_url);
//     }
//   }

//   Ok(())
// }
