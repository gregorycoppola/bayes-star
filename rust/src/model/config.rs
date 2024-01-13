use serde::Deserialize;
use once_cell::sync::OnceCell;
#[derive(Deserialize, Clone, Debug)]
pub struct ConfigurationOptions {
    pub entities_per_domain: i32,
    pub print_training_loss: bool,
}