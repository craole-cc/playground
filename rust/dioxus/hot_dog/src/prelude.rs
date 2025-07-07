//|-> External
pub use dioxus::prelude::*;
pub use tracing::{debug, error, info, trace, warn};

//|-> Internal
pub use crate::{
  error::{Error, Result},
  utils::{format::capitalize_first_letter, *}
};

//|-> Constants
pub static TITLE: GlobalSignal<&'static str> = Signal::global(|| "HotDogs");
pub const CSS: Asset = asset!("/assets/styles/main.css");
pub const ICON: Asset = asset!("/assets/favicon.ico");
pub const LOGO: Asset = asset!(
  "/assets/logo.png",
  ImageAssetOptions::new()
    .with_size(ImageSize::Manual {
      width: 52,
      height: 25
    })
    .with_format(ImageFormat::Avif)
);
