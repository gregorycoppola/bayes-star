use std::{collections::HashMap, error::Error};
use crate::model::objects::PredicateConjunction;

pub struct MonolithicBayes {
    underlying:HashMap<PredicateConjunction, f64>,
}


impl MonolithicBayes {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        Ok(MonolithicBayes{ underlying: HashMap::new() })
    }
}