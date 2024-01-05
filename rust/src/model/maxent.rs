use crate::model::objects::{BackLink, Proposition};
use crate::model::storage::{Storage, PropositionProbability};
use crate::model::weights::{read_weights, save_weights, CLASS_LABELS};
use std::error::Error;
use std::collections::HashMap;

use super::choose::compute_backlinks;
use super::config::CONFIG;
use super::conjunction::get_conjunction_probability;
use super::weights::{negative_feature, positive_feature, initialize_weights};

fn dot_product(dict1: &HashMap<String, f64>, dict2: &HashMap<String, f64>) -> f64 {
    let mut result = 0.0;
    for (key, &v1) in dict1 {
        if let Some(&v2) = dict2.get(key) {
            let product = v1 * v2;
            info!("\x1b[33mpotential {} for {:?}\x1b[0m", product, key);
            result += product;
        }
        // In case of null (None), we skip the key as per the original JavaScript logic.
    }
    result
}

pub fn compute_potential(weights: &HashMap<String, f64>, features: &HashMap<String, f64>) -> f64 {
    let dot = dot_product(weights, features);
    dot.exp()
}

pub fn features_from_backlinks<T: PropositionProbability>(
    storage: &mut T,
    backlinks: &[BackLink],
) -> Result<Vec<HashMap<String, f64>>, Box<dyn Error>> {

    trace!("Starting features_from_backlinks with {} backlinks", backlinks.len());
    let mut vec_result = vec![];
    for class_label in CLASS_LABELS {
        let mut result = HashMap::new();

        for (i, backlink) in backlinks.iter().enumerate() {
            debug!("Processing backlink {}", i);
            
            let feature = backlink.implication.unique_key(); 
            debug!("Generated unique key for feature: {}", feature);
    
            match get_conjunction_probability(storage, &backlink.conjunction) {
                Ok(probability) => {
                    debug!("Conjunction probability for backlink {}: {}", i, probability);
                    let posf = positive_feature(&feature, class_label);
                    let negf = negative_feature(&feature, class_label);
    
                    result.insert(posf.clone(), probability);
                    result.insert(negf.clone(), 1.0 - probability);
    
                    debug!("Inserted features for backlink {}: positive - {}, negative - {}", i, posf, negf);
                },
                Err(e) => {
                    error!("Error computing conjunction probability for backlink {}: {}", i, e);
                    return Err(e);
                },
            }
        }

        vec_result.push(result);
    }


    trace!("features_from_backlinks completed successfully");
    Ok(vec_result)
}

pub fn compute_expected_features(
    probability: f64,
    features: &HashMap<String, f64>,
) -> HashMap<String, f64> {
    let mut result = HashMap::new();

    for (key, &value) in features {
        result.insert(key.clone(), value * probability);
    }

    result
}

const LEARNING_RATE: f64 = 0.1;

pub fn do_sgd_update(
    weights: &HashMap<String, f64>,
    gold_features: &HashMap<String, f64>,
    expected_features: &HashMap<String, f64>,
) -> HashMap<String, f64> {
    let mut new_weights = HashMap::new();

    for (feature, &wv) in weights {
        let gv = gold_features.get(feature).unwrap_or(&0.0);
        let ev = expected_features.get(feature).unwrap_or(&0.0);

        let new_weight = wv + LEARNING_RATE * (gv - ev);
        // loss calculation is optional, here for completeness
        let _loss = (gv - ev).abs();

        let config = CONFIG.get().expect("Config not initialized");
        if config.print_training_loss {
            trace!("feature: {}, gv: {}, ev: {}, loss: {}, old_weight: {}, new_weight: {}", feature, gv, ev, _loss, wv, new_weight);

        }

        new_weights.insert(feature.clone(), new_weight);
    }

    new_weights
}

pub fn train_on_example(
    storage: &mut Storage,
    proposition: &Proposition,
    backlinks: &[BackLink],
) -> Result<(), Box<dyn Error>> {
    trace!("train_on_example - Start: {:?}", proposition.search_string());
    trace!("train_on_example - Getting features from backlinks");

    let true_label = storage.get_proposition_probability(proposition)?.expect("True probability should exist");
    let features = match features_from_backlinks(storage, backlinks) {
        Ok(f) => f,
        Err(e) => {
            trace!("train_on_example - Error in features_from_backlinks: {:?}", e);
            return Err(e);
        }
    };

    let mut weight_vectors = vec![];
    let mut potentials = vec![];
    for class_label in CLASS_LABELS {
        for (feature, weight) in &features[class_label] {
            trace!("feature {:?} {}", feature, weight);
    
        }
        trace!("train_on_example - Reading weights for class {}", class_label);
        let weight_vector = match read_weights(
            storage.get_redis_connection(),
            &features[class_label].keys().cloned().collect::<Vec<_>>(),
        ) {
            Ok(w) => w,
            Err(e) => {
                trace!("train_on_example - Error in read_weights: {:?}", e);
                return Err(e);
            }
        };

        trace!("train_on_example - Computing probability");
        let potential = compute_potential(&weight_vector, &features[class_label]);
        trace!("train_on_example - Computed probability: {}", potential);
        potentials.push(potential);
        weight_vectors.push(weight_vector);

    }

    let normalization = potentials[0] + potentials[1];

    for class_label in CLASS_LABELS {
        let probability = potentials[class_label] / normalization;
        trace!("train_on_example - Computing expected features");
        let this_true_prob = if class_label == 0 {
            1f64 - true_label
        } else {
            true_label
        };
        let gold = compute_expected_features(this_true_prob, &features[class_label]);
        let expected = compute_expected_features(probability, &features[class_label]);
    
        trace!("train_on_example - Performing SGD update");
        let new_weight = do_sgd_update(&weight_vectors[class_label], &gold, &expected);
    
        trace!("train_on_example - Saving new weights");
        save_weights(storage.get_redis_connection(), &new_weight)?;
    }

    trace!("train_on_example - End");
    Ok(())
}

pub fn do_training(storage: &mut Storage) -> Result<(), Box<dyn Error>> {
    trace!("do_training - Getting all implications");
    let implications = storage.get_all_implications()?;
    for implication in implications {
        trace!("do_training - Processing implication: {:?}", implication);
        initialize_weights(storage.get_redis_connection(), &implication)?;
    }

    trace!("do_training - Getting all propositions");
    let propositions = storage.get_all_propositions()?;
    trace!("do_training - Processing propositions: {}", propositions.len());

    let mut examples_processed = 0;
    for proposition in propositions {
        trace!("do_training - Processing proposition: {:?}", proposition);
        let backlinks = compute_backlinks(storage, &proposition)?;
        trace!("do_training - Backlinks: {:?}", backlinks);

        match train_on_example(storage, &proposition, &backlinks) {
            Ok(_) => trace!("do_training - Successfully trained on proposition: {:?}", proposition),
            Err(e) => {
                panic!("do_training - Error in train_on_example for proposition {} {:?}: {:?}", examples_processed, proposition, e)
            }

        }

        examples_processed += 1;
    }

    trace!("do_training - Training complete: examples processed {}", examples_processed);
    Ok(())
}