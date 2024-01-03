use serde::Deserialize;
use once_cell::sync::OnceCell;
#[derive(Deserialize, Clone, Debug)]
pub struct Config {
    pub entities_per_domain: i32,
    pub print_training_loss: bool,
}

// Global, immutable singleton instance
pub static CONFIG: OnceCell<Config> = OnceCell::new();

// Function to set the configuration
pub fn set_config(new_config: Config) -> Result<(), &'static str> {
    CONFIG.set(new_config)
          .map_err(|_| "Config has already been set")
}