use std::{collections::HashMap, error::Error};
use crate::model::objects::Conjunct;

pub struct MonolithicBayes {
    underlying:HashMap<Conjunct, f64>,
}


impl MonolithicBayes {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        Ok(MonolithicBayes{ underlying: HashMap::new() })
    }
}