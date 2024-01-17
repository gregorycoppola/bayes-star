use crate::common::interface::PropositionDB;
use crate::common::model::InferenceModel;
use crate::model::{
    choose::extract_backimplications_from_proposition, exponential::compute_potential,
    weights::CLASS_LABELS,
};
use crate::print_red;
use std::{collections::HashMap, error::Error};

use super::objects::{Predicate, PredicateGroup, Proposition, PropositionGroup, EXISTENCE_FUNCTION};
use super::weights::ExponentialWeights;

pub struct MapBackedProbabilityStorage {
    underlying: HashMap<String, bool>,
}

impl PropositionDB for MapBackedProbabilityStorage {
    fn get_proposition_probability(
        &self,
        proposition: &Proposition,
    ) -> Result<Option<f64>, Box<dyn Error>> {
        // NOTE: Should perhaps check set membership here.
        if proposition.predicate.function == EXISTENCE_FUNCTION {
            print_red!("Giving 1.0 probability to existence function {:?}", proposition);
            Ok(Some(1f64))
        } else {
            let search_key = proposition.predicate.hash_string();
            if let Some(&value) = self.underlying.get(&search_key) {
                // Assuming true = 1.0 probability and false = 0.0
                Ok(Some(if value { 1.0 } else { 0.0 }))
            } else {
                panic!("proposition key not found in local map {:?}", &search_key);
            }
        }
    }
    fn store_proposition_probability(
        &mut self,
        proposition: &Proposition,
        probability: f64,
    ) -> Result<(), Box<dyn Error>> {
        panic!("This doesn't exist for this subclass. Consider refactor if you see this.")
    }
}