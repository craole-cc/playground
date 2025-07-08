//|-> Modules
mod data;
mod error;
mod prelude;
mod provider;
mod utils;
// mod views;

use prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
  log::init();
  // views::launch();

  Ok(())
}
