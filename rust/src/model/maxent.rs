use super::choose::compute_backlinks;
use super::config::CONFIG;
use super::conjunction::get_conjunction_probability;
use super::objects::Implication;
use super::weights::{negative_feature, positive_feature, ExponentialWeights};
use crate::common::interface::{FactDB, PredictStatistics, TrainStatistics};
use crate::common::model::{Factor, GraphicalModel};
use crate::common::model::{FactorContext, FactorModel};
use crate::model::inference::MapBackedProbabilityStorage;
use crate::model::objects::{ConjunctLink, Proposition};
use crate::model::weights::CLASS_LABELS;
use redis::Connection;
use std::cell::RefCell;
use std::collections::HashMap;
use std::error::Error;
pub struct ExponentialModel {
    weights: ExponentialWeights,
}

impl ExponentialModel {
    pub fn new(connection: RefCell<Connection>) -> Result<Box<dyn FactorModel>, Box<dyn Error>> {
        let weights = ExponentialWeights::new(connection);
        Ok(Box::new(ExponentialModel { weights }))
    }
}

fn dot_product(dict1: &HashMap<String, f64>, dict2: &HashMap<String, f64>) -> f64 {
    let mut result = 0.0;
    for (key, &v1) in dict1 {
        if let Some(&v2) = dict2.get(key) {
            let product = v1 * v2;
            trace!("\x1b[33mpotential {} for {:?}\x1b[0m", product, key);
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

pub fn features_from_factor(
    factor: &FactorContext,
) -> Result<Vec<HashMap<String, f64>>, Box<dyn Error>> {
    let mut vec_result = vec![];
    for class_label in CLASS_LABELS {
        let mut result = HashMap::new();
        for (i, backlink) in factor.factor.conjuncts.iter().enumerate() {
            debug!("Processing backlink {}", i);
            let feature = backlink.implication.unique_key();
            debug!("Generated unique key for feature: {}", feature);
            let probability = 0f64;
            debug!(
                "Conjunction probability for backlink {}: {}",
                i, probability
            );
            let posf = positive_feature(&feature, class_label);
            let negf = negative_feature(&feature, class_label);
            result.insert(posf.clone(), probability);
            result.insert(negf.clone(), 1.0 - probability);
            debug!(
                "Inserted features for backlink {}: positive - {}, negative - {}",
                i, posf, negf
            );
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
        let loss = (gv - ev).abs();
        let config = CONFIG.get().expect("Config not initialized");
        if config.print_training_loss {
            trace!(
                "feature: {}, gv: {}, ev: {}, loss: {}, old_weight: {}, new_weight: {}",
                feature,
                gv,
                ev,
                loss,
                wv,
                new_weight
            );
        }
        new_weights.insert(feature.clone(), new_weight);
    }
    new_weights
}

impl FactorModel for ExponentialModel {
    fn initialize_connection(&mut self, implication: &Implication) -> Result<(), Box<dyn Error>> {
        self.weights.initialize_weights(implication)?;
        Ok(())
    }

    fn train(
        &mut self,
        factor: &FactorContext,
        probability: f64,
    ) -> Result<TrainStatistics, Box<dyn Error>> {
        trace!("train_on_example - Getting features from backlinks");
        let features = match features_from_factor(factor) {
            Ok(f) => f,
            Err(e) => {
                trace!(
                    "train_on_example - Error in features_from_backlinks: {:?}",
                    e
                );
                return Err(e);
            }
        };
        let mut weight_vectors = vec![];
        let mut potentials = vec![];
        for class_label in CLASS_LABELS {
            for (feature, weight) in &features[class_label] {
                trace!("feature {:?} {}", feature, weight);
            }
            trace!(
                "train_on_example - Reading weights for class {}",
                class_label
            );
            let weight_vector = match self
                .weights
                .read_weights(&features[class_label].keys().cloned().collect::<Vec<_>>())
            {
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
                1f64 - probability
            } else {
                probability
            };
            let gold = compute_expected_features(this_true_prob, &features[class_label]);
            let expected = compute_expected_features(probability, &features[class_label]);
            trace!("train_on_example - Performing SGD update");
            let new_weight = do_sgd_update(&weight_vectors[class_label], &gold, &expected);

            trace!("train_on_example - Saving new weights");
            self.weights.save_weights(&new_weight)?;
        }
        trace!("train_on_example - End");
        Ok(TrainStatistics { loss: 1f64 })
    }
    fn predict(&self, factor: &Factor) -> Result<PredictStatistics, Box<dyn Error>> {
        let features = match features_from_factor(factor) {
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
            let weight_vector = match self
                .weights
                .read_weights(&this_features.keys().cloned().collect::<Vec<_>>())
            {
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
        }
        let normalization = potentials[0] + potentials[1];
        let marginal = potentials[1] / normalization;
        Ok(PredictStatistics { marginal })
    }
}
