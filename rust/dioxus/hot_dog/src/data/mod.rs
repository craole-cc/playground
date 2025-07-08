//|-> Modules
mod dog;

//|-> Internal Exports
mod prelude {
  pub use crate::prelude::*;
  pub use async_trait::async_trait;
  pub use serde::{Deserialize, Serialize};
  pub use std::collections::HashMap;
}

//|-> External Exports
pub use dog::Dog;
