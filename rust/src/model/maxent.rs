use crate::model::objects::{Domain, Entity, Proposition, Implication, BackLink};
use crate::model::storage::{Storage};
use std::{error::Error, sync::Arc};


fn sigmoid(x: f64) -> f64 {
    1.0 / (1.0 + (-x).exp())
}

use std::collections::HashMap;

use super::weights::{positive_feature, negative_feature};

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

pub fn features_from_backlinks(storage: &Storage, backlinks: &[BackLink]) -> Result<HashMap<String, f64>, Box<dyn Error>> {
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

pub fn compute_expected_features(probability: f64, features: &HashMap<String, f64>) -> HashMap<String, f64> {
    let mut result = HashMap::new();

    for (key, &value) in features {
        result.insert(key.clone(), value * probability);
    }

    result
}