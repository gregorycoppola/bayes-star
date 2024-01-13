use std::{collections::HashMap, error::Error};
use crate::model::objects::PredicateGroup;

pub struct MonolithicBayes {
    underlying:HashMap<PredicateGroup, f64>,
}


impl MonolithicBayes {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        Ok(MonolithicBayes{ underlying: HashMap::new() })
    }
}