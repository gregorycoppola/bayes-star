use redis::{Commands, Client};
use std::error::Error;
use crate::model::objects::{Domain, Entity, Proposition, Implication};
use rand::Rng;

fn random_weight() -> f64 {
    let mut rng = rand::thread_rng();
    (rng.gen::<f64>() - rng.gen::<f64>()) / 5.0
}

fn positive_feature(feature: &str) -> String {
    format!("++{}++", feature)
}

fn negative_feature(feature: &str) -> String {
    format!("--{}--", feature)
}

pub fn initialize_weights(redis_client: &Client, implication: &Implication) -> Result<(), Box<dyn Error>> {
    let mut con = redis_client.get_connection()
        .map_err(|e| Box::new(e) as Box<dyn Error>)?;
    
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