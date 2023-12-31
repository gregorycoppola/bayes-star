use std::{collections::HashMap, error::Error};

use crate::model::{
    choose::compute_backlinks,
    maxent::{compute_potential, features_from_backlinks},
    storage::PropositionProbability,
    weights::{read_weights, CLASS_LABELS},
};

use super::{
    objects::{BackLink, Conjunction, Proposition},
    storage::Storage,
};

fn read_in_parent_probabilities(
    storage: &mut Storage,
    conjunction: &Conjunction,
) -> Result<HashMap<String, f64>, Box<dyn Error>> {
    let mut probabilities = HashMap::new();

    for (i, term) in conjunction.terms.iter().enumerate() {
        assert!(term.is_fact());
        info!(
            "Getting proposition probability for term {}: {:?}",
            i,
            term.search_string()
        );

        match storage.get_proposition_probability(term) {
            Ok(term_prob_opt) => {
                match term_prob_opt {
                    Some(term_prob) => {
                        // Insert into the hashmap
                        probabilities.insert(term.search_string(), term_prob);
                    }
                    None => {
                        // doesn't exist.. recursively compute and insert
                        let computed_prob = marginalized_inference_probability(storage, &term)?;
                        probabilities.insert(term.search_string(), computed_prob);
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

    Ok(probabilities)
}

fn print_premise_probabilities(
    storage: &mut Storage,
    conjunction: &Conjunction,
) -> Result<(), Box<dyn Error>> {
    for (i, term) in conjunction.terms.iter().enumerate() {
        assert!(term.is_fact());
        match storage.get_proposition_probability(term) {
            Ok(term_prob_opt) => match term_prob_opt {
                Some(term_prob) => {
                    info!(
                        "\x1b[32mactivation: {} {}\x1b[0m",
                        term.search_string(),
                        term_prob
                    );
                }
                None => {
                    panic!("Should have the probability by now");
                }
            },
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

struct MapBackedProbabilityStorage {
    underlying: HashMap<String, bool>,
}

impl PropositionProbability for MapBackedProbabilityStorage {
    fn get_proposition_probability(
        &mut self,
        proposition: &Proposition,
    ) -> Result<Option<f64>, Box<dyn Error>> {
        let search_key = proposition.search_string();

        if let Some(&value) = self.underlying.get(&search_key) {
            // Assuming true = 1.0 probability and false = 0.0
            Ok(Some(if value { 1.0 } else { 0.0 }))
        } else {
            panic!("proposition key not found in local map {:?}", &search_key);
        }
    }
}

pub fn compute_joint_probability(
    boolean_assignment: &HashMap<String, bool>,
    assumed_probabilities: &HashMap<String, f64>,
) -> Result<f64, Box<dyn Error>> {
    let mut joint_probability = 1.0;
    info!("\x1b[94mStarting computation of joint probability\x1b[0m {:?}", assumed_probabilities);

    for (event, &is_true) in boolean_assignment {
        info!("\x1b[94mProcessing event: {}\x1b[0m", event);
        match assumed_probabilities.get(event) {
            Some(&prob_true) => {
                let prob = if is_true { prob_true } else { 1.0 - prob_true };
                joint_probability *= prob;
                info!("\x1b[94mEvent: {}, Probability: {}, Cumulative Probability: {}\x1b[0m", event, prob, joint_probability);
            },
            None => {
                error!("\x1b[94mError: Probability not found for event: {}\x1b[0m", event);
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    format!("Probability not found for event: {}", event),
                )));
            }
        }
    }

    info!("\x1b[94mFinal joint probability: {}\x1b[0m", joint_probability);
    Ok(joint_probability)
}


pub fn local_inference_probability(
    storage: &mut Storage,
    proposition: &Proposition,
    backlinks: &[BackLink],
    assumed_probabilities: HashMap<String, bool>,
) -> Result<f64, Box<dyn Error>> {
    info!(
        "\x1b[31mlocal_inference_probability - Start: {:?}\x1b[0m",
        proposition.search_string()
    );

    let mut map_storage = MapBackedProbabilityStorage {
        underlying: assumed_probabilities,
    };
    let features = match features_from_backlinks(&mut map_storage, &backlinks) {
        Ok(f) => f,
        Err(e) => {
            info!(
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
                info!("inference_probability - Error in read_weights: {:?}", e);
                return Err(e);
            }
        };
        for (feature, weight) in weight_vector.iter() {
            trace!("weight {:?} {}", &feature, weight);
        }

        trace!("inference_probability - Computing probability");
        let potential = compute_potential(&weight_vector, &this_features);
        potentials.push(potential);
        info!(
            "inference_probability - Computed potential {} {:?}",
            potential,
            proposition.search_string()
        );
    }

    let normalization = potentials[0] + potentials[1];
    let probability = potentials[1] / normalization;
    info!(
        "\x1b[33mlocal_inference_probability - Computed probability {} {:?}\x1b[0m",
        probability,
        proposition.search_string()
    );

    Ok(probability)
}

fn each_combination(propositions: &Vec<Proposition>) -> Vec<HashMap<String, bool>> {
    let n = propositions.len();
    let mut all_combinations = Vec::new();

    for i in 0..(1 << n) {
        let mut current_combination = HashMap::new();

        for j in 0..n {
            let prop = &propositions[j];
            let state = i & (1 << j) != 0;
            current_combination.insert(prop.search_string(), state);
        }

        all_combinations.push(current_combination);
    }

    all_combinations
}

pub fn marginalized_inference_probability(
    storage: &mut Storage,
    proposition: &Proposition,
) -> Result<f64, Box<dyn Error>> {
    info!(
        "inference_probability - Start: {:?}",
        proposition.search_string()
    );
    info!("inference_probability - Getting features from backlinks");
    let backlinks = compute_backlinks(storage, &proposition)?;

    let mut direct_parents = vec![];
    let mut parent_probabilities: HashMap<String, f64> = HashMap::new();
    for backlink in &backlinks {
        let part_map = read_in_parent_probabilities(storage, &backlink.conjunction)?;
        parent_probabilities.extend(part_map);
        print_premise_probabilities(storage, &backlink.conjunction)?;
        for term in &backlink.conjunction.terms {
            direct_parents.push(term.clone());
            info!(
                "\x1b[34mdirect dependency {:?}\x1b[0m",
                term.search_string()
            );
        }
    }

    let combinations = each_combination(&direct_parents);
    let mut cumulative_probability = 0f64;
    for combination in &combinations {
        info!("\x1b[35mdirect dependency {:?}\x1b[0m", &combination);
        let local_probability =
            local_inference_probability(storage, proposition, &backlinks, combination.clone())?;
        info!(
            "\x1b[31mdirect probability {} {:?}, {:?}\x1b[0m",
            local_probability,
            proposition.search_string(),
            &combination
        );

        let joint_parent_probability = compute_joint_probability(combination, &parent_probabilities)?;
        info!(
            "\x1b[31mjoint parent probability {} {:?}\x1b[0m",
            joint_parent_probability,
            combination,
        );

        let combined = local_probability * joint_parent_probability;
        cumulative_probability += combined;
    }

    storage.store_proposition(proposition, cumulative_probability)?;

    Ok(cumulative_probability)
}
