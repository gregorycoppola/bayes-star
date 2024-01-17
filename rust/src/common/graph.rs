use super::{
    interface::{PredictStatistics, TrainStatistics},
    redis::{seq_get_all, seq_push, RedisManager}, resources::FactoryResources,
};
use crate::{
    common::{
        interface::PropositionDB,
        redis::{set_add, set_members},
    },
    model::{
        self,
        exponential::ExponentialModel,
        objects::{
            Domain, Entity, Predicate, PredicateGroup,
            PredicateInferenceFactor, Proposition, PropositionGroup,
        }, choose::extract_existence_factor_for_predicate,
    },
};
use redis::{Commands, Connection};
use serde::{Deserialize, Serialize};
use std::{cell::RefCell, error::Error, rc::Rc};
pub struct InferenceGraph {
    redis_connection: RefCell<redis::Connection>,
}

impl InferenceGraph {
    pub fn new_mutable(resources: &FactoryResources) -> Result<Box<Self>, Box<dyn Error>> {
        let redis_connection = resources.redis.get_connection()?;
        Ok(Box::new(InferenceGraph { redis_connection }))
    }

    pub fn new_shared(resources: &FactoryResources) -> Result<Rc<Self>, Box<dyn Error>> {
        let redis_connection = resources.redis.get_connection()?;
        Ok(Rc::new(InferenceGraph { redis_connection }))
    }
    
    pub fn store_entity(&mut self, entity: &Entity) -> Result<(), Box<dyn Error>> {
        trace!(
            "Storing entity in domain '{}': {}",
            entity.domain,
            entity.name
        ); // Logging
        set_add(
            &mut *self.redis_connection.borrow_mut(),
            &entity.domain.to_string(),
            &entity.name,
        )?;
        Ok(())
    }

    pub fn get_entities_in_domain(&self, domain: &str) -> Result<Vec<Entity>, Box<dyn Error>> {
        trace!("Getting entities in domain '{}'", domain.clone()); // Logging
        let names: Vec<String> = set_members(&mut *self.redis_connection.borrow_mut(), domain)?;
        Ok(names
            .into_iter()
            .map(|name| Entity {
                domain: Domain::from_str(domain).expect("Domain not recognized."), // Use the provided domain
                name,
            })
            .collect())
    }
    fn predicate_backward_set_name(predicate: &Predicate) -> String {
        format!("predicate_backward:{}", predicate.hash_string())
    }
    fn implication_seq_name() -> String {
        "implications".to_string()
    }
    fn store_implication(
        &mut self,
        implication: &PredicateInferenceFactor,
    ) -> Result<(), Box<dyn Error>> {
        let record = serialize_record(implication)?;
        set_add(
            &mut *self.redis_connection.borrow_mut(),
            &Self::implication_seq_name(),
            &record,
        )?;
        Ok(())
    }

    fn store_existence_backlinks_for_predicate(
        &mut self,
        predicate: &Predicate,
    ) -> Result<(), Box<dyn Error>> {
        let factor = extract_existence_factor_for_predicate(predicate)?;
        todo!()
    }

    fn store_existence_backlinks_for_factor(
        &mut self,
        inference: &PredicateInferenceFactor,
    ) -> Result<(), Box<dyn Error>> {
        for premise in &inference.premise.terms {
            self.store_existence_backlinks_for_predicate(premise)?;
        }
        self.store_existence_backlinks_for_predicate(&inference.conclusion)?;
        Ok(())
    }

    fn store_predicate_backward_link(
        &mut self,
        inference: &PredicateInferenceFactor,
    ) -> Result<(), Box<dyn Error>> {
        let conclusion = &inference.conclusion;
        let record = serialize_record(inference)?;
        set_add(
            &mut *self.redis_connection.borrow_mut(),
            &Self::predicate_backward_set_name(conclusion),
            &record,
        )?;
        Ok(())
    }

    pub fn store_predicate_implication(
        &mut self,
        implication: &PredicateInferenceFactor,
    ) -> Result<(), Box<dyn Error>> {
        self.store_implication(implication)?;
        self.store_predicate_backward_link(&implication)?;
        // self.store_existence_backlinks_for_factor(implication)?;
        Ok(())
    }
    pub fn get_all_implications(&self) -> Result<Vec<PredicateInferenceFactor>, Box<dyn Error>> {
        let set_members: Vec<String> = set_members(
            &mut *self.redis_connection.borrow_mut(),
            &Self::implication_seq_name(),
        )?;
        set_members
            .into_iter()
            .map(|record| serde_json::from_str(&record).map_err(|e| Box::new(e) as Box<dyn Error>))
            .collect()
    }

    pub fn predicate_backward_links(
        &self,
        conclusion: &Predicate,
    ) -> Result<Vec<PredicateInferenceFactor>, Box<dyn Error>> {
        let set_members: Vec<String> = set_members(
            &mut *self.redis_connection.borrow_mut(),
            &Self::predicate_backward_set_name(conclusion),
        )?;
        set_members
            .into_iter()
            .map(|record| serde_json::from_str(&record).map_err(|e| Box::new(e) as Box<dyn Error>))
            .collect()
    }
}

pub fn serialize_record<T>(obj: &T) -> Result<String, Box<dyn Error>>
where
    T: Serialize,
{
    serde_json::to_string(obj).map_err(|e| Box::new(e) as Box<dyn Error>)
}

fn deserialize_record<'a, T>(record: &'a str) -> Result<T, Box<dyn Error>>
where
    T: Deserialize<'a>,
{
    serde_json::from_str(record).map_err(|e| Box::new(e) as Box<dyn Error>)
}
