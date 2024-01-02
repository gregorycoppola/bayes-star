use crate::model::objects::{BackLink, Domain, Entity, Implication, Proposition};
use crate::model::storage::Storage;
use crate::model::weights::{read_weights, save_weights};
use std::{error::Error};

fn sigmoid(x: f64) -> f64 {
    1.0 / (1.0 + (-x).exp())
}

use std::collections::HashMap;

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
    storage: &Storage,
    backlinks: &[BackLink],
) -> Result<HashMap<String, f64>, Box<dyn Error>> {
    let mut result = HashMap::new();

    for backlink in backlinks {
        let feature = backlink.implication.unique_key(); // Assuming Implication has a unique_key method
        let probability = storage.get_proposition_probability(&backlink.proposition)?;
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

        new_weights.insert(feature.clone(), new_weight);
    }

    new_weights
}

pub fn train_on_example(
    storage: &Storage,
    proposition: &Proposition,
    backlinks: &[BackLink],
) -> Result<(), Box<dyn Error>> {
    let features = features_from_backlinks(storage, backlinks)?;
    let weight_vector = read_weights(
        storage.get_redis_client(),
        &features.keys().cloned().collect::<Vec<_>>(),
    )?;
    let probability = compute_probability(&weight_vector, &features);
    let expected = compute_expected_features(probability, &features);
    let new_weight = do_sgd_update(&weight_vector, &features, &expected);

    save_weights(storage.get_redis_client(), &new_weight)
}

pub fn do_training(storage: &Storage) -> Result<(), Box<dyn Error>> {
    let redis_client = storage.get_redis_client();

    // Assuming storage has a method to get all implications
    let implications = storage.get_all_implications()?;
    for implication in implications {
        initialize_weights(redis_client, &implication)?;
    }

    // // Assuming storage has a method to get all propositions
    // let propositions = storage.get_all_propositions()?;
    // for proposition in propositions {
    //     let backlinks = compute_backlinks(storage, &proposition)?;
    //     train_on_example(storage, &proposition, &backlinks)?;
    // }

    Ok(())
}