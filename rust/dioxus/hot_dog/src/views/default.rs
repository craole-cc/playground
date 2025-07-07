use super::prelude::*;

pub fn launch() {
  dioxus::launch(view);
}

#[component]
fn view() -> Element {
  rsx! {
    document::Title { {format!("{TITLE}")} }
    // document::Stylesheet { href: CSS }
    document::Link { rel: "icon", href: ICON }
    Header {}
    Main {}
    Footer {}
  }
}
