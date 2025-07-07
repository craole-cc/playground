use crate::prelude::*;

#[derive(serde::Deserialize, Clone, Default)]
pub struct Config {
  pub message: String
}

#[component]
pub fn Dog() -> Element {
  //{ Define the first dog }
  let mut dog = use_resource(|| async move { get_from_dog_ceo().await });

  #[derive(Clone)]
  enum State {
    Loading,
    Error(String),
    Loaded { image_url: String, breed: String }
  }

  let view_state = {
    let state = dog.read();
    match &*state {
      Some(Ok(dog)) => {
        let breed = extract_breed_from_url(&dog.message);
        State::Loaded {
          image_url: dog.message.clone(),
          breed
        }
      }
      Some(Err(e)) => State::Error(e.to_string()),
      None => State::Loading
    }
  };

  rsx! {
    document::Stylesheet { href: DOG_CSS }
    div { id: "dogview",
      match &view_state {
          State::Error(msg) => rsx! {
            p { class: "error", "Error: {msg}" }
          },
          State::Loading => rsx! {
            p { "Loading..." }
          },
          State::Loaded { image_url, breed } => rsx! {
            img {
              class: "dog-image-container",
              max_width: "500px",
              max_height: "500px",
              src: "{image_url}",
            }
            div { class: "dog-info",
              h2 { "{breed}" }
              p { "Source: {image_url}" }
            }
          },
      }
      div { id: "buttons",
        button { onclick: move |_| dog.restart(), id: "skip", "skip" }
        button {
          id: "save",
          onclick: move |_| {
              let value = view_state.clone();
              async move {
                  if let State::Loaded { image_url, .. } = value {
                      let current = image_url.clone();
                      _ = save_dog(current).await;
                      dog.restart();
                  }
              }
          },

          "save!"
        }
      }
    }
  }
}

fn ensure_not_empty(dog: Dog) -> Result<Dog, Error> {
  if dog.message.is_empty() {
    Err(Error::NoResponse)
  } else {
    Ok(dog)
  }
}

async fn get_from_dog_ceo() -> Result<Dog, Error> {
  let response = reqwest::get(DOG_CEO_RANDO).await?;
  let dog = response.json::<Dog>().await?;
  ensure_not_empty(dog)
}

pub fn extract_breed_from_url(dog_ceo_url: &str) -> String {
  let raw = dog_ceo_url
    .split("/breeds/")
    .nth(1)
    .and_then(|s| s.split('/').next())
    .unwrap_or("");

  //{ Format: "sheepdog-english" -> "English Sheepdog" or "Sheepdog English" }
  let mut parts: Vec<&str> = raw.split('-').collect();
  if parts.len() == 2 {
    //{ Try to present as "English Sheepdog" (sub-breed first) }
    parts.reverse();
  }
  let formatted = parts
    .into_iter()
    .map(|s| {
      let mut c = s.chars();
      match c.next() {
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
        None => String::new()
      }
    })
    .collect::<Vec<_>>()
    .join(" ");
  formatted
}

#[server]
async fn save_dog(image: String) -> Result<(), ServerFnError> {
  use std::io::Write;

  // Open the `dogs.txt` file in append-only mode, creating it if it doesn't
  // exist;
  let mut file = std::fs::OpenOptions::new()
    .write(true)
    .append(true)
    .create(true)
    .open(DOG_TXT)
    .unwrap();

  // And then write a newline to it with the image url
  file.write_fmt(format_args!("{image}\n"));

  Ok(())
}
