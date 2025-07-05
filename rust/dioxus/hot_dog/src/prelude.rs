pub use dioxus::prelude::*;
pub const FAVICON: Asset = asset!("/assets/favicon.ico");
pub const MAIN_CSS: Asset = asset!("/assets/styles/main.css");
pub const DOG_CSS: Asset = asset!("/assets/styles/dog.css");
pub const LOGO: Asset = asset!(
  "/assets/logo.png",
  ImageAssetOptions::new()
    .with_size(ImageSize::Manual {
      width: 52,
      height: 25
    })
    .with_format(ImageFormat::Avif)
);
pub const DOG_TXT: &str = "assets/data/dog.txt";
pub const DOG_CEO_RANDO: &str = "https://dog.ceo/api/breeds/image/random";
pub static TITLE: GlobalSignal<&'static str> = Signal::global(|| "HotDogs");
