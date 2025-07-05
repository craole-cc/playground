//|-> Modules
mod custom;
mod default;
mod dog_ceo;

//|-> Internal Exports
mod prelude {
  pub use super::default::Content;
  pub use crate::components::dog::config::prelude::*;
}

//|-> External Exports
pub use default::Provider;
// pub use default::Config;
// pub use provider::Provider;
