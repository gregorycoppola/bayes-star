use redis::{Commands, Client, Connection};
use std::error::Error;
use crate::model::objects::{Domain, Entity, Proposition, Implication};
use rand::Rng;
use std::collections::HashMap;

fn random_weight() -> f64 {
    let mut rng = rand::thread_rng();
    (rng.gen::<f64>() - rng.gen::<f64>()) / 5.0
}

pub fn positive_feature(feature: &str) -> String {
    format!("++{}++", feature)
}

pub fn negative_feature(feature: &str) -> String {
    format!("--{}--", feature)
}

pub fn initialize_weights(con: &mut Connection, implication: &Implication) -> Result<(), Box<dyn Error>> {
    let feature = implication.unique_key(); // Assuming Implication has a unique_key method
    let posf = positive_feature(&feature);
    let negf = negative_feature(&feature);
    let weight1 = random_weight();
    let weight2 = random_weight();

    con.hset("weights", &posf, weight1)
        .map_err(|e| Box::new(e) as Box<dyn Error>)?;
    con.hset("weights", &negf, weight2)
        .map_err(|e| Box::new(e) as Box<dyn Error>)?;

    Ok(())
}

pub fn read_weights(con: &mut Connection, features: &[String]) -> Result<HashMap<String, f64>, Box<dyn Error>> {
    let mut weights = HashMap::new();
    for feature in features {
        let record: String = con.hget("weights", feature)
            .map_err(|e| Box::new(e) as Box<dyn Error>)?;
        
        let weight = record.parse::<f64>()
            .map_err(|e| Box::new(e) as Box<dyn Error>)?;
        weights.insert(feature.clone(), weight);
    }

    Ok(weights)
}

pub fn save_weights(con: &mut Connection, weights: &HashMap<String, f64>) -> Result<(), Box<dyn Error>> {
    for (feature, &value) in weights {
        con.hset("weights", feature, value)
            .map_err(|e| Box::new(e) as Box<dyn Error>)?;
    }

    Ok(())
}