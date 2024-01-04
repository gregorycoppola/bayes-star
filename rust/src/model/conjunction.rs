use std::error::Error;

use super::{objects::Conjunction, storage::Storage};

pub fn get_conjunction_probability(
    storage: &mut Storage,
    conjunction: &Conjunction,
) -> Result<f64, Box<dyn Error>> {
    let mut min_prob = 1f64;
    for term in &conjunction.terms {
        let term_prob = storage.get_proposition_probability(term)?;
        min_prob = min_prob.min(term_prob);
    }

    Ok(min_prob)
}