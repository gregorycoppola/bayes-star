use crate::{
    common::{
        redis::{map_get, map_insert},
        resources::ResourceContext,
    },
    model::objects::ImplicationFactor,
};
use rand::Rng;
use redis::{Commands, Connection};
use std::{cell::RefCell, error::Error};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

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
    namespace: String,
}

impl ExponentialWeights {
    pub fn new(namespace: String) -> Result<ExponentialWeights, Box<dyn Error>> {
        Ok(ExponentialWeights { namespace })
    }
}

impl ExponentialWeights {
    pub const WEIGHTS_KEY: &'static str = "weights";

    pub fn initialize_weights(
        &mut self,
        connection: &mut Connection,
        implication: &ImplicationFactor,
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
            map_insert(
                connection,
                &self.namespace,
                Self::WEIGHTS_KEY,
                &posf,
                &weight1.to_string(),
            )?;
            map_insert(
                connection,
                &self.namespace,
                Self::WEIGHTS_KEY,
                &negf,
                &weight2.to_string(),
            )?;
        }
        trace!("initialize_weights - End");
        Ok(())
    }

    pub fn read_single_weight(
        &self,
        connection: &mut Connection,
        feature: &str,
    ) -> Result<f64, Box<dyn Error>> {
        trace!("read_weights - Start");
        warn!("read_weights - Reading weight for feature: {}", feature);
        let weight_record = map_get(connection, &self.namespace, Self::WEIGHTS_KEY, &feature)?
        .unwrap_or("0.0".to_string());
            // .expect("should be there");
        let weight = weight_record.parse::<f64>().map_err(|e| {
            trace!("read_weights - Error parsing weight: {:?}", e);
            Box::new(e) as Box<dyn Error>
        })?;
        trace!("read_weights - End");
        Ok(weight)
    }

    pub fn read_weight_vector(
        &self,
        connection: &mut Connection,
        features: &[String],
    ) -> Result<HashMap<String, f64>, Box<dyn Error>> {
        trace!("read_weights - Start");
        let mut weights = HashMap::new();
        for feature in features {
            trace!("read_weights - Reading weight for feature: {}", feature);
            let weight_record = map_get(connection, &self.namespace, Self::WEIGHTS_KEY, &feature)?
                .expect("should be there");
            let weight = weight_record.parse::<f64>().map_err(|e| {
                trace!("read_weights - Error parsing weight: {:?}", e);
                Box::new(e) as Box<dyn Error>
            })?;
            weights.insert(feature.clone(), weight);
        }
        trace!("read_weights - End");
        Ok(weights)
    }

    pub fn save_weight_vector(
        &mut self,
        connection: &mut Connection,
        weights: &HashMap<String, f64>,
    ) -> Result<(), Box<dyn Error>> {
        trace!("save_weights - Start");
        for (feature, &value) in weights {
            trace!(
                "save_weights - Saving weight for feature {}: {}",
                feature,
                value
            );
            map_insert(
                connection,
                &self.namespace,
                Self::WEIGHTS_KEY,
                &feature,
                &value.to_string(),
            )?;
        }
        trace!("save_weights - End");
        Ok(())
    }
}
