pub enum LogLevel {
  Level(tracing::Level),
  String(String)
}

impl From<tracing::Level> for LogLevel {
  fn from(level: tracing::Level) -> Self {
    LogLevel::Level(level)
  }
}

impl From<&str> for LogLevel {
  fn from(s: &str) -> Self {
    LogLevel::String(s.to_string())
  }
}

impl From<String> for LogLevel {
  fn from(s: String) -> Self {
    LogLevel::String(s)
  }
}

pub fn init<T: Into<LogLevel>>(lvl: T) {
  let mut subscriber = tracing_subscriber::fmt();

  match std::env::var("RUST_LOG") {
    Ok(env_filter) => {
      subscriber
        .with_env_filter(tracing_subscriber::EnvFilter::new(env_filter))
        .init();
    }
    Err(_) => {
      let level = match lvl.into() {
        LogLevel::Level(level) => level,
        LogLevel::String(level_str) => {
          // Try to parse as a full level name first
          match level_str.to_lowercase().as_str() {
            "error" => tracing::Level::ERROR,
            "warn" => tracing::Level::WARN,
            "info" => tracing::Level::INFO,
            "debug" => tracing::Level::DEBUG,
            "trace" => tracing::Level::TRACE,
            _ => {
              // Fall back to first character parsing
              let level_char =
                level_str.chars().next().unwrap_or('i').to_ascii_lowercase();

              match level_char {
                'e' => tracing::Level::ERROR,
                'd' => tracing::Level::DEBUG,
                't' => tracing::Level::TRACE,
                'w' => tracing::Level::WARN,
                _ => tracing::Level::INFO
              }
            }
          }
        }
      };

      subscriber.with_max_level(level).init();
    }
  }
}
