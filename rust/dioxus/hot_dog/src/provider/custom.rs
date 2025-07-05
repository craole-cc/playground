use super::prelude::*;

pub struct Provider {
  pub base_url: String
}

impl Provider {
  pub fn new(base_url: String) -> Self {
    Self { base_url }
  }
}

impl Content for Provider {
  async fn photo(&self, url: Option<&str>) -> Result<String> {
    let url = url.unwrap_or(&self.base_url);
    let response = reqwest::get(url).await?;

    // Try to parse as JSON first
    if let Ok(json) = response.json::<serde_json::Value>().await {
      // Common JSON fields for photo URLs
      let photo_url = json
        .get("url")
        .or_else(|| json.get("message"))
        .or_else(|| json.get("image"))
        .or_else(|| json.get("photo"))
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());

      photo_url.ok_or(Error::NoResponse)
    } else {
      Err(Error::NoResponse)
    }
  }

  fn breed(&self, _photo_url: &str) -> String {
    // Custom APIs might have different breed extraction logic
    "Unknown".to_string()
  }
}
