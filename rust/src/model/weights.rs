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
    println!("initialize_weights - Start: {:?}", implication);

    let feature = implication.unique_key();
    println!("initialize_weights - Unique key: {}", feature);

    let posf = positive_feature(&feature);
    let negf = negative_feature(&feature);
    println!("initialize_weights - Positive feature: {}, Negative feature: {}", posf, negf);

    let weight1 = random_weight();
    let weight2 = random_weight();
    println!("initialize_weights - Generated weights: {}, {}", weight1, weight2);

    println!("initialize_weights - Setting positive feature weight");
    con.hset("weights", &posf, weight1)
        .map_err(|e| {
            println!("initialize_weights - Error setting positive feature weight: {:?}", e);
            Box::new(e) as Box<dyn Error>
        })?;

    println!("initialize_weights - Setting negative feature weight");
    con.hset("weights", &negf, weight2)
        .map_err(|e| {
            println!("initialize_weights - Error setting negative feature weight: {:?}", e);
            Box::new(e) as Box<dyn Error>
        })?;

    println!("initialize_weights - End");
    Ok(())
}


pub fn read_weights(con: &mut Connection, features: &[String]) -> Result<HashMap<String, f64>, Box<dyn Error>> {
    println!("read_weights - Start");
    let mut weights = HashMap::new();

    for feature in features {
        println!("read_weights - Reading weight for feature: {}", feature);
        match con.hget::<_, _, String>("weights", feature) {
            Ok(record) => {
                println!("read_weights - Retrieved record: {}", record);
                let weight = record.parse::<f64>()
                    .map_err(|e| {
                        println!("read_weights - Error parsing weight: {:?}", e);
                        Box::new(e) as Box<dyn Error>
                    })?;
                weights.insert(feature.clone(), weight);
            }
            Err(e) => {
                println!("read_weights - Error retrieving weight for feature {}: {:?}", feature, e);
                return Err(Box::new(e) as Box<dyn Error>);
            }
        }
    }

    println!("read_weights - End");
    Ok(weights)
}


pub fn save_weights(con: &mut Connection, weights: &HashMap<String, f64>) -> Result<(), Box<dyn Error>> {
    println!("save_weights - Start");
    for (feature, &value) in weights {
        println!("save_weights - Saving weight for feature {}: {}", feature, value);
        con.hset("weights", feature, value)
            .map_err(|e| {
                println!("save_weights - Error saving weight for feature {}: {:?}", feature, e);
                Box::new(e) as Box<dyn Error>
            })?;
    }

    println!("save_weights - End");
    Ok(())
}
