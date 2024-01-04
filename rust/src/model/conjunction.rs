use std::error::Error;

use super::{objects::Conjunction, storage::Storage};

pub fn get_conjunction_probability(
    storage: &mut Storage,
    conjunction: &Conjunction,
) -> Result<f64, Box<dyn Error>> {
    todo!("implement me")
}