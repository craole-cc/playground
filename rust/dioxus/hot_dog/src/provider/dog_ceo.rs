use super::prelude::*;

#[derive(serde::Deserialize, Debug)]
pub struct Api {
  pub message: String,
  pub status: String
}

pub struct Provider;

impl Content for Provider {
  async fn photo(&self, url: Option<&str>) -> Result<String> {
    let url = url.unwrap_or(DOG_CEO_RANDOM_URL);
    let response = reqwest::get(url).await?;
    let dog_response = response.json::<Api>().await?;

    if dog_response.message.is_empty() {
      Err(Error::NoResponse)
    } else {
      Ok(dog_response.message)
    }
  }

  fn breed(&self, photo_url: &str) -> String {
    let raw = photo_url
      .split("/breeds/")
      .nth(1)
      .and_then(|s| s.split('/').next())
      .unwrap_or("");

    // Format: "sheepdog-english" -> "English Sheepdog"
    let mut parts: Vec<&str> = raw.split('-').collect();
    if parts.len() == 2 {
      parts.reverse();
    }

    if parts.is_empty() || parts[0].is_empty() {
      return "Unknown".to_string();
    }

    parts
      .into_iter()
      .map(|s| {
        let mut c = s.chars();
        match c.next() {
          Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
          None => String::new()
        }
      })
      .collect::<Vec<_>>()
      .join(" ")
  }
}
