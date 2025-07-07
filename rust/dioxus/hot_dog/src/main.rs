//|-> Modules
mod error;
mod prelude;
mod provider;
mod utils;
mod views;

use prelude::*;

fn main() {
  log::init("WARN");
  // views::launch();
}
