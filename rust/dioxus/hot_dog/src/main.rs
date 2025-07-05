mod dog;
mod prelude;

use prelude::*;

fn main() {
  dioxus::launch(App);
}

#[component]
fn App() -> Element {
  rsx! {
    document::Stylesheet { href: MAIN_CSS }
    document::Link { rel: "icon", href: FAVICON }
    div { id: "app-container",
      Title {}
      main { role: "main", class: "content-area",
        dog::DogView {}
      }
    }
  }
}

#[component]
fn Title() -> Element {
  rsx! {
    div { id: "title",
      img { src: LOGO }
      h1 { {format!("{TITLE}!")} }
    }
  }
}
