//TODO: Replace with logline
use std::str::FromStr;
use tracing::debug;
use tracing_subscriber::{fmt, EnvFilter};

/// Represents a logging level.
#[derive(Debug, Clone)]
pub enum Level {
  Level(tracing::Level),
  String(String)
}

// -- Trait Implementations --
impl From<tracing::Level> for Level {
  fn from(level: tracing::Level) -> Self {
    Self::Level(level)
  }
}
impl From<&str> for Level {
  fn from(s: &str) -> Self {
    Self::String(s.to_owned())
  }
}
impl From<String> for Level {
  fn from(s: String) -> Self {
    Self::String(s)
  }
}
impl Default for Level {
  fn default() -> Self {
    Self::Level(tracing::Level::INFO)
  }
}

impl Level {
  fn to_tracing_level(&self) -> tracing::Level {
    match self {
      Level::Level(level) => *level,
      Level::String(s) => tracing::Level::from_str(s.to_lowercase().as_str())
        .unwrap_or(tracing::Level::INFO)
    }
  }
}

// -- Private Helper --

/// Builds and initializes the subscriber with a given filter.
fn setup_subscriber(filter: EnvFilter) {
  fmt().without_time().with_env_filter(filter).init();
}

// -- Public API --

/// Initializes the logger, prioritizing the `RUST_LOG` environment variable.
///
/// If `RUST_LOG` is not set, it falls back to the `INFO` level.
pub fn init() {
  let filter = match std::env::var("RUST_LOG") {
    Ok(env_var) if !env_var.is_empty() => EnvFilter::new(env_var),
    _ =>
      EnvFilter::from_default_env().add_directive(tracing::Level::INFO.into()),
  };
  setup_subscriber(filter);
  debug!("Logging initialized, respecting RUST_LOG.");
}

/// Initializes the logger with a specific level, **ignoring `RUST_LOG`**.
///
/// This provides a hard override, which is useful for forcing a specific log
/// level in tests or applications regardless of the environment.
pub fn init_with_level<L>(level: L)
where
  L: Into<Level>
{
  let tracing_level = level.into().to_tracing_level();
  let filter =
    EnvFilter::from_default_env().add_directive(tracing_level.into());
  setup_subscriber(filter);
  debug!("Logging initialized with forced level: {:?}", tracing_level);
}

// -- Test Utilities --

#[cfg(test)]
pub mod testing {
  use std::sync::Once;
  static INIT: Once = Once::new();

  /// Initializes logging for tests.
  ///
  /// This will default to the level set in `super::init()` (typically INFO)
  /// but can be overridden by the `RUST_LOG` environment variable.
  /// Call this function at the beginning of every test.
  pub fn init() {
    INIT.call_once(|| {
      super::init();
    });
  }
}
