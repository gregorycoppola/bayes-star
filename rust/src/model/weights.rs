use crate::{
    common::{redis::{map_get, map_insert}, resources::FactoryResources},
    model::objects::PredicateFactor,
};
use rand::Rng;
use redis::{Commands, Connection};
use std::collections::HashMap;
use std::{cell::RefCell, error::Error};

pub const CLASS_LABELS: [usize; 2] = [0, 1];

fn random_weight() -> f64 {
    let mut rng = rand::thread_rng();
    (rng.gen::<f64>() - rng.gen::<f64>()) / 5.0
}

fn sign_char(value: usize) -> String {
    if value == 0 {
        '-'.to_string()
    } else {
        "+".to_string()
    }
}

pub fn positive_feature(feature: &str, class_label: usize) -> String {
    format!("+>{} {}", sign_char(class_label), feature)
}

pub fn negative_feature(feature: &str, class_label: usize) -> String {
    format!("->{} {}", sign_char(class_label), feature)
}

pub struct ExponentialWeights {
    connection: RefCell<Connection>,
    namespace: String,
}

impl ExponentialWeights {
    pub fn new(resources: &FactoryResources) -> Result<ExponentialWeights, Box<dyn Error>> {
        let connection = resources.redis.get_connection()?;
        Ok(ExponentialWeights {
            connection,
            namespace: resources.config.scenario_name.clone(),
        })
    }
}

impl ExponentialWeights {
    pub const WEIGHTS_KEY: &'static str = "weights";

    pub fn initialize_weights(
        &mut self,
        implication: &PredicateFactor,
    ) -> Result<(), Box<dyn Error>> {
        trace!("initialize_weights - Start: {:?}", implication);
        let feature = implication.unique_key();
        trace!("initialize_weights - Unique key: {}", feature);
        for class_label in CLASS_LABELS {
            let posf = positive_feature(&feature, class_label);
            let negf = negative_feature(&feature, class_label);
            trace!(
                "initialize_weights - Positive feature: {}, Negative feature: {}",
                posf,
                negf
            );
            let weight1 = random_weight();
            let weight2 = random_weight();
            trace!(
                "initialize_weights - Generated weights: {}, {}",
                weight1,
                weight2
            );
            // trace!("initialize_weights - Setting positive feature weight");
            map_insert(
                &mut self.connection.borrow_mut(),
                &self.namespace,
                Self::WEIGHTS_KEY,
                &posf,
                &weight1.to_string(),
            )?;
            map_insert(
                &mut self.connection.borrow_mut(),
                &self.namespace,
                Self::WEIGHTS_KEY,
                &negf,
                &weight2.to_string(),
            )?;
            // self.connection.borrow_mut().hset("weights", &posf, weight1)
            //     .map_err(|e| {
            //         trace!("initialize_weights - Error setting positive feature weight: {:?}", e);
            //         Box::new(e) as Box<dyn Error>
            //     })?;
            // trace!("initialize_weights - Setting negative feature weight");
            // self.connection.borrow_mut().hset("weights", &negf, weight2)
            //     .map_err(|e| {
            //         trace!("initialize_weights - Error setting negative feature weight: {:?}", e);
            //         Box::new(e) as Box<dyn Error>
            //     })?;
        }
        trace!("initialize_weights - End");
        Ok(())
    }

    pub fn read_weights(
        &self,
        features: &[String],
    ) -> Result<HashMap<String, f64>, Box<dyn Error>> {
        trace!("read_weights - Start");
        let mut weights = HashMap::new();
        for feature in features {
            trace!("read_weights - Reading weight for feature: {}", feature);
            let weight_record = map_get(
                &mut self.connection.borrow_mut(),
                &self.namespace,
                Self::WEIGHTS_KEY,
                &feature,
            )?.expect("should be there");
            let weight = weight_record.parse::<f64>().map_err(|e| {
                trace!("read_weights - Error parsing weight: {:?}", e);
                Box::new(e) as Box<dyn Error>
            })?;
            weights.insert(feature.clone(), weight);
        }
        trace!("read_weights - End");
        Ok(weights)
    }

    pub fn save_weights(&mut self, weights: &HashMap<String, f64>) -> Result<(), Box<dyn Error>> {
        trace!("save_weights - Start");
        for (feature, &value) in weights {
            trace!(
                "save_weights - Saving weight for feature {}: {}",
                feature,
                value
            );
            map_insert(
                &mut self.connection.borrow_mut(),
                &self.namespace,
                Self::WEIGHTS_KEY,
                &feature,
                &value.to_string(),
            )?;
            // self.connection.borrow_mut().hset("weights", feature, value)
            //     .map_err(|e| {
            //         trace!("save_weights - Error saving weight for feature {}: {:?}", feature, e);
            //         Box::new(e) as Box<dyn Error>
            //     })?;
        }
        trace!("save_weights - End");
        Ok(())
    }
}
