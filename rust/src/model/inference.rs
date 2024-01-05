use std::error::Error;

use crate::model::{
    choose::compute_backlinks,
    maxent::{compute_probability, features_from_backlinks},
    weights::read_weights,
};

use super::{
    objects::{Conjunction, Proposition},
    storage::Storage,
};

fn ensure_probabilities_are_stored(
    storage: &mut Storage,
    conjunction: &Conjunction,
) -> Result<(), Box<dyn Error>> {
    for (i, term) in conjunction.terms.iter().enumerate() {
        assert!(term.is_fact());
        info!("Getting proposition probability for term {}: {:?}", i, term.search_string());

        match storage.get_proposition_probability(term) {
            Ok(term_prob_opt) => {
                match term_prob_opt {
                    Some(_term_prob) => {
                        // exists.. do nothing
                    }
                    None => {
                        // doesn't exist.. recursively compute
                        inference_probability(storage, &term)?;
                    }
                }
            }
            Err(e) => {
                error!(
                    "Error getting proposition probability for term {}: {}",
                    i, e
                );
                return Err(e);
            }
        }
    }
    Ok(())
}

pub fn inference_probability(
    storage: &mut Storage,
    proposition: &Proposition,
) -> Result<f64, Box<dyn Error>> {
    info!("inference_probability - Start: {:?}", proposition.search_string());
    info!("inference_probability - Getting features from backlinks");
    let backlinks = compute_backlinks(storage, &proposition)?;

    for backlink in &backlinks {
        ensure_probabilities_are_stored(storage, &backlink.conjunction)?;
    }

    let features = match features_from_backlinks(storage, &backlinks) {
        Ok(f) => f,
        Err(e) => {
            info!(
                "inference_probability - Error in features_from_backlinks: {:?}",
                e
            );
            return Err(e);
        }
    };

    info!("inference_probability - Reading weights");
    let weight_vector = match read_weights(
        storage.get_redis_connection(),
        &features.keys().cloned().collect::<Vec<_>>(),
    ) {
        Ok(w) => w,
        Err(e) => {
            info!("inference_probability - Error in read_weights: {:?}", e);
            return Err(e);
        }
    };

    info!("inference_probability - Computing probability");
    let probability = compute_probability(&weight_vector, &features);

    info!("inference_probability - Computed probability {} {:?}", probability, proposition.search_string());

    storage.store_proposition(proposition, probability)?;

    Ok(probability)
}
