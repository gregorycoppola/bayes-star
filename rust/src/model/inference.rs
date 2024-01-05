use std::error::Error;

use crate::model::{
    choose::compute_backlinks,
    maxent::{compute_potential, features_from_backlinks},
    weights::{read_weights, CLASS_LABELS},
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
        trace!("Getting proposition probability for term {}: {:?}", i, term.search_string());

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

fn print_premise_probabilities(
    storage: &mut Storage,
    conjunction: &Conjunction,
) -> Result<(), Box<dyn Error>> {
    for (i, term) in conjunction.terms.iter().enumerate() {
        assert!(term.is_fact());
        match storage.get_proposition_probability(term) {
            Ok(term_prob_opt) => {
                match term_prob_opt {
                    Some(term_prob) => {
                        trace!("activation: {} {}", term.search_string(), term_prob);
                    }
                    None => {
                        panic!("Should have the probability by now");
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
    trace!("inference_probability - Start: {:?}", proposition.search_string());
    trace!("inference_probability - Getting features from backlinks");
    let backlinks = compute_backlinks(storage, &proposition)?;

    for backlink in &backlinks {
        ensure_probabilities_are_stored(storage, &backlink.conjunction)?;
        print_premise_probabilities(storage, &backlink.conjunction)?;
    }

    let features = match features_from_backlinks(storage, &backlinks) {
        Ok(f) => f,
        Err(e) => {
            trace!(
                "inference_probability - Error in features_from_backlinks: {:?}",
                e
            );
            return Err(e);
        }
    };

    let mut potentials = vec![];
    for class_label in CLASS_LABELS {
        let this_features = &features[class_label];
        for (feature, weight) in this_features.iter() {
            trace!("feature {:?} {}", &feature, weight);
        }
    
        trace!("inference_probability - Reading weights");
        let weight_vector = match read_weights(
            storage.get_redis_connection(),
            &this_features.keys().cloned().collect::<Vec<_>>(),
        ) {
            Ok(w) => w,
            Err(e) => {
                trace!("inference_probability - Error in read_weights: {:?}", e);
                return Err(e);
            }
        };
        for (feature, weight) in weight_vector.iter() {
            trace!("weight {:?} {}", &feature, weight);
        }
    
        trace!("inference_probability - Computing probability");
        let potential = compute_potential(&weight_vector, &this_features);
        potentials.push(potential);
        trace!("inference_probability - Computed potential {} {:?}", potential, proposition.search_string());
    }

    let normalization = potentials[0] + potentials[1];
    let probability = potentials[1] / normalization;
    trace!("inference_probability - Computed probability {} {:?}", probability, proposition.search_string());

    storage.store_proposition(proposition, probability)?;

    Ok(probability)
}
