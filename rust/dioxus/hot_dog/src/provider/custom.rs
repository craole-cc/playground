use super::prelude::*;

pub struct Provider {
  pub base_url: String
}

impl Provider {
  pub fn new(base_url: String) -> Self {
    Self { base_url }
  }

  pub fn parse_custom_format(&self, url: &str) -> Result<Breed> {
    let breed_str = url
      .split('/')
      .nth_back(0)
      .ok_or_else(|| Error::Url(format!("No breed in URL: {url}")))?
      .split('.')
      .next()
      .ok_or_else(|| Error::Url(format!("No breed identifier: {url}")))?;

    let mut parts = breed_str.splitn(2, '-');
    match (parts.next(), parts.next()) {
      (Some(a), Some(b)) => Ok(Breed::new(
        b,
        Some(a),
        format!("{}/breed/{}/{}", self.base_url, b, a),
        Breed::format_name(b, Some(a))
      )),
      (Some(a), None) => Ok(Breed::new(
        a,
        None::<String>,
        format!("{}/breed/{}", self.base_url, a),
        Breed::format_name(a, None)
      )),
      _ => Err(Error::BreedFormat(format!("Invalid format: {breed_str}")))
    }
  }

  // Provider-specific photo parsing
  async fn parse_photo_response(
    &self,
    source: DataSource<'_>
  ) -> Result<String> {
    let json = fetch_json(source).await?;

    // Try common photo URL field names
    json
      .get("url")
      .or_else(|| json.get("message"))
      .or_else(|| json.get("image"))
      .or_else(|| json.get("photo"))
      .and_then(|v| v.as_str())
      .map(|s| s.to_string())
      .ok_or_else(|| Error::EmptyResponse)
  }

  // Provider-specific breed parsing
  async fn parse_breed_response(
    &self,
    source: DataSource<'_>
  ) -> Result<Breed> {
    let json = fetch_json(source).await?;

    let main_breed = json
      .get("breed")
      .and_then(|v| v.as_str())
      .ok_or_else(|| Error::EmptyResponse)?;

    let sub_breed = json.get("sub_breed").and_then(|v| v.as_str());

    Ok(Breed::new(
      main_breed,
      sub_breed,
      format!("{}/breed/{}", self.base_url, main_breed),
      Breed::format_name(main_breed, sub_breed)
    ))
  }
}

#[async_trait]
impl Content for Provider {
  async fn photo(&self, source: DataSource<'_>) -> Result<String> {
    // Try parsing as JSON first
    match self.parse_photo_response(source.clone()).await {
      Ok(photo_url) => Ok(photo_url),
      Err(_) => {
        // Fallback: if it's a URL, just return it directly
        match source {
          DataSource::Url(url) => Ok(url.to_string()),
          _ => Err(Error::Provider(
            "Cannot extract photo from non-URL source".to_string()
          ))
        }
      }
    }
  }

  async fn breed(&self, source: DataSource<'_>) -> Result<Breed> {
    // Try parsing as structured breed data first
    match self.parse_breed_response(source.clone()).await {
      Ok(breed) => Ok(breed),
      Err(_) => {
        // Fallback: try extracting from photo URL
        match source {
          DataSource::Url(url) => {
            let photo_url = self.photo(DataSource::Url(url)).await?;
            self.parse_custom_format(&photo_url)
          }
          _ => Err(Error::Provider(
            "Cannot parse breed from this source".to_string()
          ))
        }
      }
    }
  }
}
