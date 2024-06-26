use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize)]
pub struct Config {
  #[serde(default)]
  pub input_update_interval_ms: u64,
}

impl Default for Config {
  fn default() -> Self {
    Self {
      input_update_interval_ms: 3,
    }
  }
}

impl Config {
  const FILENAME: &'static str = "config.json";

  /// Try to load the config from `config.json`.
  /// If the file does not exist, create a new one with the default values.
  pub fn init() -> Self {
    fs::read_to_string(Self::FILENAME)
      .ok()
      .and_then(|content| serde_json::from_str(&content).ok())
      .unwrap_or_else(|| {
        let config = Config::default();
        config.save();
        config
      })
  }

  /// Save the config to `config.json`.
  pub fn save(&self) {
    fs::write(Self::FILENAME, serde_json::to_string_pretty(self).unwrap()).unwrap();
  }
}
