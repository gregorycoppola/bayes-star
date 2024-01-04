use crate::model::objects::{BackLink, Domain, Entity, Implication, Proposition};
use crate::model::storage::Storage;
use crate::model::weights::{read_weights, save_weights};
use std::{error::Error};

fn sigmoid(x: f64) -> f64 {
    1.0 / (1.0 + (-x).exp())
}

use std::collections::HashMap;

use super::choose::compute_backlinks;
use super::config::CONFIG;
use super::conjunction::get_conjunction_probability;
use super::weights::{negative_feature, positive_feature, initialize_weights};

fn dot_product(dict1: &HashMap<String, f64>, dict2: &HashMap<String, f64>) -> f64 {
    let mut result = 0.0;
    for (key, &v1) in dict1 {
        if let Some(&v2) = dict2.get(key) {
            result += v1 * v2;
        }
        // In case of null (None), we skip the key as per the original JavaScript logic.
    }
    result
}

fn compute_probability(weights: &HashMap<String, f64>, features: &HashMap<String, f64>) -> f64 {
    let dot = dot_product(weights, features);
    sigmoid(dot)
}

pub fn features_from_backlinks(
    storage: &mut Storage,
    backlinks: &[BackLink],
) -> Result<HashMap<String, f64>, Box<dyn Error>> {
    let mut result = HashMap::new();

    for backlink in backlinks {
        let feature = backlink.implication.unique_key(); // Assuming Implication has a unique_key method
        let probability = get_conjunction_probability(&backlink.conjunction)?;
        let posf = positive_feature(&feature);
        let negf = negative_feature(&feature);

        result.insert(posf, probability);
        result.insert(negf, 1.0 - probability);
    }

    Ok(result)
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
            println!("feature: {}, gv: {}, ev: {}, loss: {}, new_weight: {}", feature, gv, ev, _loss, new_weight);

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
    trace!("train_on_example - Start: {:?}", proposition);

    trace!("train_on_example - Getting features from backlinks");
    let features = match features_from_backlinks(storage, backlinks) {
        Ok(f) => f,
        Err(e) => {
            trace!("train_on_example - Error in features_from_backlinks: {:?}", e);
            return Err(e);
        }
    };

    trace!("train_on_example - Reading weights");
    let weight_vector = match read_weights(
        storage.get_redis_connection(),
        &features.keys().cloned().collect::<Vec<_>>(),
    ) {
        Ok(w) => w,
        Err(e) => {
            trace!("train_on_example - Error in read_weights: {:?}", e);
            return Err(e);
        }
    };

    trace!("train_on_example - Computing probability");
    let probability = compute_probability(&weight_vector, &features);
    trace!("train_on_example - Computed probability: {}", probability);

    trace!("train_on_example - Computing expected features");
    let expected = compute_expected_features(probability, &features);

    trace!("train_on_example - Performing SGD update");
    let new_weight = do_sgd_update(&weight_vector, &features, &expected);

    trace!("train_on_example - Saving new weights");
    save_weights(storage.get_redis_connection(), &new_weight)?;

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