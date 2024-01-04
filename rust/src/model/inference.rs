use std::error::Error;

use super::objects::Proposition;

pub fn inference_probability(proposition:&Proposition) -> Result<f64, Box<dyn Error>> {
    Ok(0.5f64)
}