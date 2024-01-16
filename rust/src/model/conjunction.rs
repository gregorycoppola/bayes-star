use std::error::Error;
use crate::common::interface::PropositionDB;

use super::objects::{PredicateGroup, PropositionGroup};

/// Compute the "conjunction probability", but assume INDEPENDENCE.
/// Therefore, the probability of the group is the product of all of the probabilities of the terms.
pub fn get_conjunction_probability(
    proposition_db: &dyn PropositionDB,
    conjunction: &PropositionGroup,
) -> Result<f64, Box<dyn Error>> {
    trace!("Calculating conjunction probability for a conjunction with {} terms", conjunction.terms.len());
    let mut product = 1f64;
    for (i, term) in conjunction.terms.iter().enumerate() {
        trace!("Getting proposition probability for term {}: {:?}", i, term);
        match proposition_db.get_proposition_probability(term) {
            Ok(term_prob_opt) => {
                let term_prob = term_prob_opt.expect("`term_prob` should be Some here.");
                trace!("Term probability for term {}: {}", term.predicate.hash_string(), term_prob);
                product *= term_prob;
                trace!("Updated min probability after term {}: {}", term.predicate.hash_string(), product);
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