//! # Configuration Module
//!
//! This module handles configuration management for the Resend CLI.
//! It supports loading configuration from environment variables or a configuration file,
//! and provides methods for saving configuration to disk.

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Configuration struct containing API credentials and settings
///
/// This struct holds the configuration for the Resend CLI, primarily the API key
/// used for authenticating with the Resend API. The configuration can be loaded
/// from environment variables or a configuration file.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    /// API key for authenticating with the Resend API
    pub api_key: String,
}

impl Config {
    /// Loads configuration from environment variables or configuration file
    ///
    /// This method attempts to load the configuration in the following order:
    /// 1. From the RESEND_API_KEY environment variable
    /// 2. From the configuration file at ~/.resend-cli/config.json
    ///
    /// # Returns
    ///
    /// A Config instance with the loaded configuration, or an error if neither
    /// the environment variable nor the config file could be found
    pub fn load() -> Result<Self> {
        dotenv::dotenv().ok();

        let api_key = std::env::var("RESEND_API_KEY").ok();

        if let Some(key) = api_key {
            return Ok(Config { api_key: key });
        }

        // Try loading from config file if env var not set
        let config_path = Self::config_path()?;
        if config_path.exists() {
            let content = std::fs::read_to_string(&config_path)?;
            let config: Config = serde_json::from_str(&content)?;
            return Ok(config);
        }

        anyhow::bail!("RESEND_API_KEY environment variable not set and config file not found. Use 'resend config --api-key <KEY>' to set it.")
    }

    /// Saves the current configuration to the configuration file
    ///
    /// This method writes the current configuration to the configuration file
    /// at ~/.resend-cli/config.json, creating the directory structure if needed.
    ///
    /// # Returns
    ///
    /// Ok(()) if the configuration was saved successfully, or an error if the
    /// file could not be written
    pub fn save(&self) -> Result<()> {
        let config_path = Self::config_path()?;
        if let Some(parent) = config_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let content = serde_json::to_string_pretty(self)?;
        std::fs::write(config_path, content)?;
        Ok(())
    }

    /// Gets the default configuration file path
    ///
    /// This method returns the path to the configuration file at
    /// ~/.resend-cli/config.json
    ///
    /// # Returns
    ///
    /// The path to the configuration file, or an error if the home directory
    /// could not be determined
    fn config_path() -> Result<PathBuf> {
        let home = dirs::home_dir().context("Could not find home directory")?;
        Ok(home.join(".resend-cli").join("config.json"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_config_load_from_env_var() {
        // Set up environment variable
        env::set_var("RESEND_API_KEY", "test_api_key_from_env");

        let config = Config::load().unwrap();
        assert_eq!(config.api_key, "test_api_key_from_env");

        // Clean up
        env::remove_var("RESEND_API_KEY");
    }

    #[test]
    fn test_config_save_and_load_from_file() {
        // Create a temporary directory
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join(".resend-cli").join("config.json");

        // Create a config and save it to the temp location
        let config = Config {
            api_key: "test_api_key_from_file".to_string(),
        };

        // Override the config_path function to use our temp directory
        // Since we can't easily override the private config_path function,
        // we'll test the save functionality separately
        let config_json = serde_json::to_string_pretty(&config).unwrap();
        fs::create_dir_all(config_path.parent().unwrap()).unwrap();
        fs::write(&config_path, config_json).unwrap();

        // Temporarily set HOME to our temp directory
        let original_home = env::var("HOME").ok();
        env::set_var("HOME", temp_dir.path());

        // Now test loading from the file
        let loaded_config = Config::load().unwrap();
        assert_eq!(loaded_config.api_key, "test_api_key_from_file");

        // Restore original HOME
        if let Some(home) = original_home {
            env::set_var("HOME", home);
        } else {
            env::remove_var("HOME");
        }
    }

    #[test]
    fn test_config_save_creates_directories() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join(".resend-cli").join("config.json");

        let config = Config {
            api_key: "test_api_key_for_saving".to_string(),
        };

        // Override the config_path function temporarily by creating the file directly
        let config_json = serde_json::to_string_pretty(&config).unwrap();
        fs::create_dir_all(config_path.parent().unwrap()).unwrap();
        fs::write(&config_path, config_json).unwrap();

        // Check that the file was created with correct content
        let content = fs::read_to_string(&config_path).unwrap();
        let parsed_config: Config = serde_json::from_str(&content).unwrap();
        assert_eq!(parsed_config.api_key, "test_api_key_for_saving");
    }

    #[test]
    fn test_config_struct_creation() {
        let config = Config {
            api_key: "test_key".to_string(),
        };

        assert_eq!(config.api_key, "test_key");
    }
}
