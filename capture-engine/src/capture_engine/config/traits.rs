// config/traits.rs
use crate::traits::{Error, Lifecycle, Validate};
/// `ConfigManager` loads and validates configuration data.
use async_trait::async_trait;

/// Trait for managing configurations.
#[async_trait]
pub trait ConfigManager: Lifecycle + Validate + Send + Sync {
    /// Loads the configuration.
    async fn load_configuration(&mut self) -> Result<(), Error>;

    /// Applies a configuration update.
    async fn apply_configuration(&mut self, config: Configuration) -> Result<(), Error>;

    /// Retrieves the current configuration.
    fn current_configuration(&self) -> Configuration;
}

/// Represents the configuration data.
#[derive(Debug, Clone)]
pub struct Configuration {
    pub settings: std::collections::HashMap<String, String>,
}
