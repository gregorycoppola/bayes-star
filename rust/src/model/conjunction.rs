use std::error::Error;
use crate::model::storage::PropositionProbability;
use super::objects::Conjunction;

pub fn get_conjunction_probability<T: PropositionProbability>(
    storage: &mut T,
    conjunction: &Conjunction,
) -> Result<f64, Box<dyn Error>> {
    trace!("Calculating conjunction probability for a conjunction with {} terms", conjunction.terms.len());
    let mut min_prob = 1f64;

    for (i, term) in conjunction.terms.iter().enumerate() {
        trace!("Getting proposition probability for term {}: {:?}", i, term);
        
        match storage.get_proposition_probability(term) {
            Ok(term_prob_opt) => {
                let term_prob = term_prob_opt.expect("`term_prob` should be Some here.");
                trace!("Term probability for term {}: {}", term.search_string(), term_prob);
                min_prob = min_prob.min(term_prob);
                trace!("Updated min probability after term {}: {}", term.search_string(), min_prob);
            },
            Err(e) => {
                error!("Error getting proposition probability for term {}: {}", i, e);
                return Err(e);
            },
        }
    }

    trace!("Conjunction probability calculated: {}", min_prob);
    Ok(min_prob)
}