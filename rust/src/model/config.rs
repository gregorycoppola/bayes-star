use serde::Deserialize;
use once_cell::sync::Lazy;
use std::sync::Mutex;

#[derive(Deserialize, Clone, Debug)]
pub struct Config {
    pub entities_per_domain: i32,
    pub print_training_loss: bool,
}

// Global, mutable singleton instance
pub static CONFIG: Lazy<Mutex<Config>> = Lazy::new(|| {
    Mutex::new(Config {
        entities_per_domain: 32, // default values
        print_training_loss: false,
    })
});

// Function to initialize/update the configuration
pub fn set_config(new_config: Config) {
    println!("BAYES STAR starts with config {:?}", &new_config);
    let mut config = CONFIG.lock().unwrap();
    *config = new_config;
}
