use super::choose::extract_backimplications_from_proposition;
use super::config::ConfigurationOptions;
use super::objects::PredicateFactor;
use super::weights::{negative_feature, positive_feature, ExponentialWeights};
use crate::common::interface::{PropositionDB, PredictStatistics, TrainStatistics};
use crate::common::model::InferenceModel;
use crate::common::model::{FactorContext, FactorModel};
use crate::common::redis::RedisManager;
use crate::common::resources::FactoryResources;
use crate::model::objects::Predicate;
use crate::model::weights::CLASS_LABELS;
use crate::{print_yellow, print_blue};
use redis::Connection;
use std::cell::RefCell;
use std::collections::HashMap;
use std::error::Error;
use std::rc::Rc;
pub struct ExponentialModel {
    config: ConfigurationOptions,
    weights: ExponentialWeights,
}

impl ExponentialModel {
    pub fn new_mutable(resources: &FactoryResources) -> Result<Box<dyn FactorModel>, Box<dyn Error>> {
        let connection = resources.redis.get_connection()?;
        let weights = ExponentialWeights::new(connection);
        Ok(Box::new(ExponentialModel {
            config: resources.config.clone(),
            weights,
        }))
    }
    pub fn new_shared(resources: &FactoryResources) -> Result<Rc<dyn FactorModel>, Box<dyn Error>> {
        let connection = resources.redis.get_connection()?;
        let weights = ExponentialWeights::new(connection);
        Ok(Rc::new(ExponentialModel {
            config: resources.config.clone(),
            weights,
        }))
    }
}

fn dot_product(dict1: &HashMap<String, f64>, dict2: &HashMap<String, f64>) -> f64 {
    let mut result = 0.0;
    for (key, &v1) in dict1 {
        if let Some(&v2) = dict2.get(key) {
            let product = v1 * v2;
            print_blue!("dot_product: key {}, v1 {}, v2 {}, product {}", key, v1, v2, product);
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
        for (i, premise) in factor.factor.iter().enumerate() {
            debug!("Processing backimplication {}", i);
            let feature = premise.inference.unique_key();
            debug!("Generated unique key for feature: {}", feature);
            let probability = factor.probabilities[i];
            debug!(
                "Conjunction probability for backimplication {}: {}",
                i, probability
            );
            let posf = positive_feature(&feature, class_label);
            let negf = negative_feature(&feature, class_label);
            result.insert(posf.clone(), probability);
            result.insert(negf.clone(), 1.0 - probability);
            debug!(
                "Inserted features for backimplication {}: positive - {}, negative - {}",
                i, posf, negf
            );
        }
        vec_result.push(result);
    }
    trace!("features_from_backimplications completed successfully");
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

const LEARNING_RATE: f64 = 0.01;

pub fn do_sgd_update(
    weights: &HashMap<String, f64>,
    gold_features: &HashMap<String, f64>,
    expected_features: &HashMap<String, f64>,
    print_training_loss: bool,
) -> HashMap<String, f64> {
    let mut new_weights = HashMap::new();
    for (feature, &wv) in weights {
        let gv = gold_features.get(feature).unwrap_or(&0.0);
        let ev = expected_features.get(feature).unwrap_or(&0.0);
        let new_weight = wv + LEARNING_RATE * (gv - ev);
        let loss = (gv - ev).abs();
        if print_training_loss {
            info!(
                "feature: {}, gv: {}, ev: {}, loss: {}, old_weight: {}, new_weight: {}",
                feature, gv, ev, loss, wv, new_weight
            );
        }
        new_weights.insert(feature.clone(), new_weight);
    }
    new_weights
}

impl FactorModel for ExponentialModel {
    fn initialize_connection(
        &mut self,
        implication: &PredicateFactor,
    ) -> Result<(), Box<dyn Error>> {
        self.weights.initialize_weights(implication)?;
        Ok(())
    }

    fn train(
        &mut self,
        factor: &FactorContext,
        gold_probability: f64,
    ) -> Result<TrainStatistics, Box<dyn Error>> {
        trace!("train_on_example - Getting features from backimplications");
        let features = match features_from_factor(factor) {
            Ok(f) => f,
            Err(e) => {
                trace!(
                    "train_on_example - Error in features_from_backimplications: {:?}",
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
                1f64 - gold_probability
            } else {
                gold_probability
            };
            let gold = compute_expected_features(this_true_prob, &features[class_label]);
            let expected = compute_expected_features(probability, &features[class_label]);
            trace!("train_on_example - Performing SGD update");
            let new_weight = do_sgd_update(
                &weight_vectors[class_label],
                &gold,
                &expected,
                self.config.print_training_loss,
            );
            trace!("train_on_example - Saving new weights");
            self.weights.save_weights(&new_weight)?;
        }
        trace!("train_on_example - End");
        Ok(TrainStatistics { loss: 1f64 })
    }
    fn predict(&self, factor: &FactorContext) -> Result<PredictStatistics, Box<dyn Error>> {
        let features = match features_from_factor(factor) {
            Ok(f) => f,
            Err(e) => {
                print_yellow!(
                    "inference_probability - Error in features_from_backimplications: {:?}",
                    e
                );
                return Err(e);
            }
        };
        let mut potentials = vec![];
        for class_label in CLASS_LABELS {
            let this_features = &features[class_label];
            for (feature, weight) in this_features.iter() {
                print_yellow!("feature {:?} {}", &feature, weight);
            }
            print_yellow!("inference_probability - Reading weights");
            let weight_vector = match self
                .weights
                .read_weights(&this_features.keys().cloned().collect::<Vec<_>>())
            {
                Ok(w) => w,
                Err(e) => {
                    print_yellow!("inference_probability - Error in read_weights: {:?}", e);
                    return Err(e);
                }
            };
            for (feature, weight) in weight_vector.iter() {
                print_yellow!("weight {:?} {}", &feature, weight);
            }
            let potential = compute_potential(&weight_vector, &this_features);
            print_yellow!("potential for {} {} {:?}", class_label, potential, &factor);
            potentials.push(potential);
        }
        let normalization = potentials[0] + potentials[1];
        let marginal = potentials[1] / normalization;
        print_yellow!("dot_product: normalization {}, marginal {}", normalization, marginal);
        Ok(PredictStatistics { marginal })
    }
}
