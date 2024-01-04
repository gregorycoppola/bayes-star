use std::error::Error;

use super::{objects::Conjunction, storage::Storage};

pub fn get_conjunction_probability(
    storage: &mut Storage,
    conjunction: &Conjunction,
) -> Result<f64, Box<dyn Error>> {
    info!("Calculating conjunction probability for a conjunction with {} terms", conjunction.terms.len());
    let mut min_prob = 1f64;

    for (i, term) in conjunction.terms.iter().enumerate() {
        info!("Getting proposition probability for term {}: {:?}", i, term);
        
        match storage.get_proposition_probability(term) {
            Ok(term_prob) => {
                info!("Term probability for term {}: {}", i, term_prob);
                min_prob = min_prob.min(term_prob);
                info!("Updated min probability after term {}: {}", i, min_prob);
            },
            Err(e) => {
                error!("Error getting proposition probability for term {}: {}", i, e);
                return Err(e);
            },
        }
    }

    info!("Conjunction probability calculated: {}", min_prob);
    Ok(min_prob)
}