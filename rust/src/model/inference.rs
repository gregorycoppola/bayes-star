use std::{collections::HashMap, error::Error};
use crate::common::interface::FactDB;
use crate::common::model::GraphicalModel;
use crate::model::{
    choose::extract_backlinks_from_proposition,
    maxent::compute_potential,
    weights::{CLASS_LABELS},
};

use super::weights::ExponentialWeights;
use super::objects::{ConjunctLink, Conjunction, Proposition};

fn read_in_parent_probabilities(
    storage: &mut GraphicalModel,
    conjunction: &Conjunction,
) -> Result<HashMap<String, f64>, Box<dyn Error>> {
    todo!()
    // let mut probabilities = HashMap::new();

    // for (i, term) in conjunction.terms.iter().enumerate() {
    //     assert!(term.is_fact());
    //     info!(
    //         "Getting proposition probability for term {}: {:?}",
    //         i,
    //         term.search_string()
    //     );

    //     match storage.fact_db.get_proposition_probability(term) {
    //         Ok(term_prob_opt) => {
    //             match term_prob_opt {
    //                 Some(term_prob) => {
    //                     // Insert into the hashmap
    //                     probabilities.insert(term.search_string(), term_prob);
    //                 }
    //                 None => {
    //                     // doesn't exist.. recursively compute and insert
    //                     let computed_prob = marginalized_inference_probability(storage, &term)?;
    //                     probabilities.insert(term.search_string(), computed_prob);
    //                 }
    //             }
    //         }
    //         Err(e) => {
    //             error!(
    //                 "Error getting proposition probability for term {}: {}",
    //                 i, e
    //             );
    //             return Err(e);
    //         }
    //     }
    // }

    // Ok(probabilities)
}

fn print_premise_probabilities(
    storage: &mut GraphicalModel,
    conjunction: &Conjunction,
) -> Result<(), Box<dyn Error>> {
    for (i, term) in conjunction.terms.iter().enumerate() {
        assert!(term.is_fact());
        match storage.fact_db.get_proposition_probability(term) {
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

pub struct MapBackedProbabilityStorage {
    underlying: HashMap<String, bool>,
}

impl FactDB for MapBackedProbabilityStorage {
    fn get_proposition_probability(
        &self,
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
    fn store_proposition_probability(
        &mut self,
        proposition: &Proposition,
        probability: f64,
    ) -> Result<(), Box<dyn Error>> {
        panic!("This doesn't exist for this subclass. Consider refactor if you see this.")
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
