//|-> Modules
mod carousel;
mod default;
mod footer;
mod header;
mod main;

//|-> Internal Exports
mod prelude {
  pub use super::{
    carousel::ImageCarousel, footer::Footer, header::Header, main::Main
  };
  pub use crate::prelude::*;
}

//|-> External Exports
pub use default::launch;
