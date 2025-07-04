use dioxus::prelude::*;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");
const LOGO: Asset = asset!(
  "/assets/logo.png",
  ImageAssetOptions::new()
    .with_size(ImageSize::Manual {
      width: 52,
      height: 25
    })
    .with_format(ImageFormat::Avif)
);

static TITLE: GlobalSignal<&'static str> = Signal::global(|| "HotDogs");

#[derive(Clone, Copy)]
struct Dog {
  breed: Signal<&'static str>,
  url: Signal<&'static str>
}

#[derive(serde::Deserialize)]
struct DogApi {
  message: String
}

fn main() {
  dioxus::launch(App);
}

#[component]
fn App() -> Element {
  let default_dog_breed = use_signal(|| "pitbull");
  let default_dog_url =
    use_signal(|| "https://images.dog.ceo/breeds/pitbull/dog-3981540_1280.jpg");
  use_context_provider(|| Dog {
    breed: default_dog_breed,
    url: default_dog_url
  });

  rsx! {
    document::Stylesheet { href: MAIN_CSS }
    document::Link { rel: "icon", href: FAVICON }
    Title {}
    DogView {}
  }
}

#[component]
fn Title() -> Element {
  rsx! {
    div { id: "title",
      img { src: LOGO }
      h1 { {format!("{}!", TITLE)} }
    }
  }
}

#[component]
fn DogView() -> Element {
  let dog = use_context::<Dog>();
  let photo = use_hook(|| dog.url);
  let breed = use_hook(|| dog.breed);

  let next_dog_breed = "leonberg";
  let next_dog_url = "https://images.dog.ceo/breeds/leonberg/n02111129_974.jpg";

  let skip = move |_| {};
  let save = move |_| {
    consume_context::<Dog>().url.set(next_dog_url);
    consume_context::<Dog>().breed.set(next_dog_breed)
  };

  rsx! {
    div { id: "dogview",
      img { src: "{photo}" }
    }
    p { "This is a {breed}." }
    div { id: "buttons",
      button { onclick: skip, id: "skip", "skip" }
      button { onclick: save, id: "save", "save!" }
    }
  }
}
