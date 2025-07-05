//|-> Modules
mod app;
mod error;
mod prelude;
pub use prelude::*;

fn main() {
  app::launch();
}
