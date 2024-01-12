use super::{
    interface::{PredictStatistics, TrainStatistics},
    redis::{RedisManager, seq_push},
};
use crate::{
    common::{
        interface::FactDB,
        redis::{set_add, set_members},
    },
    model::{
        self,
        maxent::ExponentialModel,
        objects::{
            Domain, Entity, ImplicationInstance, Predicate, PredicateConjunction,
            PredicateImplication, Proposition, PropositionConjunction,
        },
    },
};
use redis::{Commands, Connection};
use serde::{Deserialize, Serialize};
use std::{cell::RefCell, error::Error};
pub struct Graph {
    redis_connection: RefCell<redis::Connection>,
}

impl Graph {
    // Initialize new GraphicalModel with a Redis connection
    pub fn new(redis: &RedisManager) -> Result<Self, Box<dyn Error>> {
        let redis_connection = redis.get_connection()?;
        Ok(Graph { redis_connection })
    }
    // Store an entity
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
    fn predicate_forward_set_name(predicate:&Predicate) -> String {
        format!("predicate_forward:{}", predicate.hash_string())
    }
    fn predicate_backward_set_name(predicate:&Predicate) -> String {
        format!("predicate_backward:{}", predicate.hash_string())
    }
    fn conjunction_forward_set_name(predicate:&Predicate) -> String {
        format!("conjunction_forward:{}", predicate.hash_string())
    }
    fn implication_seq_name() -> String {
        "implications".to_string()
    }
    fn store_implication(&mut self, implication: &PredicateImplication) -> Result<(), Box<dyn Error>> {
        let record = serialize_record(implication)?;
        seq_push(
            &mut *self.redis_connection.borrow_mut(),
            &Self::implication_seq_name(),
            &record,
        )?;
        Ok(())
    }
    fn store_predicate_forward_links(&mut self, conjunction: &PredicateConjunction) -> Result<(), Box<dyn Error>> {
        for predicate in &conjunction.terms {
            let record = serialize_record(conjunction)?;
            set_add(
                &mut *self.redis_connection.borrow_mut(),
                &Self::predicate_forward_set_name(predicate),
                &record,
            )?;
        }
        Ok(())
    }
    fn store_predicate_backward_link(&mut self, conclusion: &Predicate, premise: &PredicateConjunction) -> Result<(), Box<dyn Error>> {
        let record = serialize_record(premise)?;
        set_add(
            &mut *self.redis_connection.borrow_mut(),
            &Self::predicate_backward_set_name(conclusion),
            &record,
        )?;
        Ok(())
    }
    fn store_conjunction_forward_link(&mut self, premise: &PredicateConjunction, conclusion: &Predicate) -> Result<(), Box<dyn Error>> {
        let record = serialize_record(premise)?;
        set_add(
            &mut *self.redis_connection.borrow_mut(),
            &Self::predicate_backward_set_name(conclusion),
            &record,
        )?;
        Ok(())
    }
    pub fn store_predicate_implication(
        &mut self,
        implication: &PredicateImplication,
    ) -> Result<(), Box<dyn Error>> {
        self.store_implication(implication)?;
        self.store_predicate_backward_link(&implication.conclusion, &implication.premise)?;
        self.store_conjunction_forward_link(&implication.premise, &implication.conclusion)?;
        self.store_predicate_forward_links( &implication.premise)?;
        Ok(())
    }
    pub fn get_all_implications(&self) -> Result<Vec<PredicateImplication>, Box<dyn Error>> {
        todo!()
    }
    pub fn parents_of_predicate(
        &self,
        predicate: &Predicate,
    ) -> Result<Vec<PredicateImplication>, Box<dyn Error>> {
        let hash_string = predicate.hash_string();
        trace!("find_premises: {:?}", &hash_string);
        let set_members: Vec<String> =
            set_members(&mut *self.redis_connection.borrow_mut(), &hash_string)?;
        set_members
            .into_iter()
            .map(|record| serde_json::from_str(&record).map_err(|e| Box::new(e) as Box<dyn Error>))
            .collect()
    }
    pub fn children_of_predicate(
        &self,
        root: &Predicate,
    ) -> Result<Vec<PredicateConjunction>, Box<dyn Error>> {
        todo!()
    }
    pub fn parents_of_conjunct(
        &self,
        conjunction: &PredicateConjunction,
    ) -> Result<Vec<Predicate>, Box<dyn Error>> {
        Ok(conjunction.terms.clone())
    }
    pub fn children_of_conjunct(
        &self,
        conjunction: &PredicateConjunction,
    ) -> Result<Vec<Predicate>, Box<dyn Error>> {
        todo!()
    }
}

fn serialize_record<T>(obj: &T) -> Result<String, Box<dyn Error>>
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