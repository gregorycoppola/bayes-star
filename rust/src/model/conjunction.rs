use std::error::Error;
use crate::common::interface::FactDB;

use super::objects::Conjunction;

/// Compute the "conjunction probability", but assume INDEPENDENCE.
/// Therefore, the probability of the conjunct is the product of all of the probabilities of the terms.
pub fn get_conjunction_probability(
    fact_db: &dyn FactDB,
    conjunction: &Conjunction,
) -> Result<f64, Box<dyn Error>> {
    trace!("Calculating conjunction probability for a conjunction with {} terms", conjunction.terms.len());
    let mut product = 1f64;
    for (i, term) in conjunction.terms.iter().enumerate() {
        trace!("Getting proposition probability for term {}: {:?}", i, term);
        match fact_db.get_proposition_probability(term) {
            Ok(term_prob_opt) => {
                let term_prob = term_prob_opt.expect("`term_prob` should be Some here.");
                trace!("Term probability for term {}: {}", term.search_string(), term_prob);
                product *= term_prob;
                trace!("Updated min probability after term {}: {}", term.search_string(), product);
            },
            Err(e) => {
                error!("Error getting proposition probability for term {}: {}", i, e);
                return Err(e);
            },
        }
    }
    trace!("Conjunction probability calculated: {}", product);
    Ok(product)
}