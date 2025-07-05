//|-> Modules
mod default;
mod provider;

//|-> Internal Exports
mod prelude {
  pub use super::provider::Provider;
  pub use crate::components::dog::prelude::*;
}

//|-> External Exports
pub use default::Config;
