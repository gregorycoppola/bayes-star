use redis::{Commands, Connection};
use std::{error::Error, cell::RefCell};
use crate::model::objects::Implication;
use rand::Rng;
use std::collections::HashMap;

pub const CLASS_LABELS: [usize; 2] = [0, 1];

fn random_weight() -> f64 {
    let mut rng = rand::thread_rng();
    (rng.gen::<f64>() - rng.gen::<f64>()) / 5.0
}

pub fn positive_feature(feature: &str, class_label:usize) -> String {
    format!("+>{} {}", class_label, feature)
}

pub fn negative_feature(feature: &str, class_label : usize) -> String {
    format!("->{} {}", class_label, feature)
}

pub struct ExponentialWeights {
    connection:RefCell<Connection>,
}

impl ExponentialWeights {
    pub fn new(connection: RefCell<Connection>) -> ExponentialWeights {
        ExponentialWeights { connection }
    }
}

impl ExponentialWeights {
    pub fn initialize_weights(&mut self, link: &Implication) -> Result<(), Box<dyn Error>> {
        trace!("initialize_weights - Start: {:?}", link);
        let feature = link.unique_key();
        trace!("initialize_weights - Unique key: {}", feature);
        for class_label in CLASS_LABELS {
            let posf = positive_feature(&feature, class_label);
            let negf = negative_feature(&feature, class_label);
            trace!("initialize_weights - Positive feature: {}, Negative feature: {}", posf, negf);
            let weight1 = random_weight();
            let weight2 = random_weight();
            trace!("initialize_weights - Generated weights: {}, {}", weight1, weight2);
            trace!("initialize_weights - Setting positive feature weight");
            self.connection.borrow_mut().hset("weights", &posf, weight1)
                .map_err(|e| {
                    trace!("initialize_weights - Error setting positive feature weight: {:?}", e);
                    Box::new(e) as Box<dyn Error>
                })?;
            trace!("initialize_weights - Setting negative feature weight");
            self.connection.borrow_mut().hset("weights", &negf, weight2)
                .map_err(|e| {
                    trace!("initialize_weights - Error setting negative feature weight: {:?}", e);
                    Box::new(e) as Box<dyn Error>
                })?;
        }
        trace!("initialize_weights - End");
        Ok(())
    }
    
    
    pub fn read_weights(&self, features: &[String]) -> Result<HashMap<String, f64>, Box<dyn Error>> {
        trace!("read_weights - Start");
        let mut weights = HashMap::new();
        for feature in features {
            trace!("read_weights - Reading weight for feature: {}", feature);
            match self.connection.borrow_mut().hget::<_, _, String>("weights", feature) {
                Ok(record) => {
                    trace!("read_weights - Retrieved record: {}", record);
                    let weight = record.parse::<f64>()
                        .map_err(|e| {
                            trace!("read_weights - Error parsing weight: {:?}", e);
                            Box::new(e) as Box<dyn Error>
                        })?;
                    weights.insert(feature.clone(), weight);
                }
                Err(e) => {
                    trace!("read_weights - Error retrieving weight for feature {}: {:?}", feature, e);
                    return Err(Box::new(e) as Box<dyn Error>);
                }
            }
        }
        trace!("read_weights - End");
        Ok(weights)
    }
    
    pub fn save_weights(&mut self, weights: &HashMap<String, f64>) -> Result<(), Box<dyn Error>> {
        trace!("save_weights - Start");
        for (feature, &value) in weights {
            trace!("save_weights - Saving weight for feature {}: {}", feature, value);
            self.connection.borrow_mut().hset("weights", feature, value)
                .map_err(|e| {
                    trace!("save_weights - Error saving weight for feature {}: {:?}", feature, e);
                    Box::new(e) as Box<dyn Error>
                })?;
        }
        trace!("save_weights - End");
        Ok(())
    }
}


