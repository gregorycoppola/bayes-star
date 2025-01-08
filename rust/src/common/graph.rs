use super::{
    interface::{PredictStatistics, TrainStatistics},
    redis::RedisManager,
    resources::FactoryResources,
};
use crate::{
    common::{
        interface::BeliefTable,
        redis::{is_member, set_add, set_members},
    },
    model::{
        self,
        choose::{
            extract_existence_factor_for_predicate, extract_existence_factor_for_proposition,
        },
        exponential::ExponentialModel,
        objects::{
            Domain, Entity, Predicate, PredicateFactor, PredicateGroup, Proposition,
            PropositionGroup, Relation,
        },
    },
    print_blue,
};
use redis::{Commands, Connection};
use serde::{Deserialize, Serialize};
use std::{cell::RefCell, error::Error, rc::Rc};
pub struct InferenceGraph {
    redis_connection: RefCell<redis::Connection>,
    namespace: String,
}

impl InferenceGraph {
    pub fn new_mutable(resources: &FactoryResources) -> Result<Box<Self>, Box<dyn Error>> {
        let redis_connection = resources.redis.get_connection()?;
        Ok(Box::new(InferenceGraph {
            redis_connection,
            namespace: resources.config.scenario_name.clone(),
        }))
    }

    pub fn new_shared(resources: &FactoryResources) -> Result<Rc<Self>, Box<dyn Error>> {
        let redis_connection = resources.redis.get_connection()?;
        Ok(Rc::new(InferenceGraph {
            redis_connection,
            namespace: resources.config.scenario_name.clone(),
        }))
    }

    pub fn register_relation(&mut self, relation: &Relation) -> Result<(), Box<dyn Error>> {
        todo!()
    }

    pub fn check_relation(&mut self, relation: &Relation) -> Result<(), Box<dyn Error>> {
        todo!()
    }

    pub fn get_all_relations(&self) -> Result<Vec<Relation>, Box<dyn Error>> {
        todo!()
    }

    pub fn register_domain(&mut self, domain: &String) -> Result<(), Box<dyn Error>> {
        set_add(
            &mut *self.redis_connection.borrow_mut(),
            &self.namespace,
            "domains",
            domain,
        )?;
        Ok(())
    }

    /// Return a "substantive" iff it's ok, else panic.
    pub fn check_domain(&self, domain: &String) -> Result<(), Box<dyn Error>> {
        let result = is_member(
            &mut *self.redis_connection.borrow_mut(),
            &self.namespace,
            "domains",
            domain,
        )?;
        assert!(result);
        Ok(())
    }

    pub fn get_all_domains(&self) -> Result<Vec<String>, Box<dyn Error>> {
        let result = set_members(
            &mut *self.redis_connection.borrow_mut(),
            &self.namespace,
            "domains",
        )?;
        Ok(result)
    }

    pub fn store_entity(&mut self, entity: &Entity) -> Result<(), Box<dyn Error>> {
        trace!(
            "Storing entity in domain '{}': {}",
            entity.domain,
            entity.name
        );
        self.check_domain(&entity.domain)?;
        // NOTE: this is a "set" named after the "domain", with each "entity.name" inside of it.
        set_add(
            &mut *self.redis_connection.borrow_mut(),
            &self.namespace,
            &entity.domain.to_string(),
            &entity.name,
        )?;
        Ok(())
    }

    pub fn get_entities_in_domain(&self, domain: &String) -> Result<Vec<Entity>, Box<dyn Error>> {
        let domain_string = domain.to_string();
        let names: Vec<String> = set_members(
            &mut *self.redis_connection.borrow_mut(),
            &self.namespace,
            &domain_string,
        )?;
        Ok(names
            .into_iter()
            .map(|name| Entity {
                domain: domain.clone(),
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

    fn store_implication(&mut self, implication: &PredicateFactor) -> Result<(), Box<dyn Error>> {
        let record = serialize_record(implication)?;
        set_add(
            &mut *self.redis_connection.borrow_mut(),
            &self.namespace,
            &Self::implication_seq_name(),
            &record,
        )?;
        Ok(())
    }

    // TODO: I feel like this should not be public.
    pub fn ensure_existence_backlinks_for_proposition(
        &mut self,
        proposition: &Proposition,
    ) -> Result<(), Box<dyn Error>> {
        let implication = extract_existence_factor_for_proposition(proposition)?;
        self.store_predicate_implication(&implication)?;
        Ok(())
    }

    fn store_predicate_backward_link(
        &mut self,
        inference: &PredicateFactor,
    ) -> Result<(), Box<dyn Error>> {
        let conclusion = &inference.conclusion;
        let record = serialize_record(inference)?;
        set_add(
            &mut *self.redis_connection.borrow_mut(),
            &self.namespace,
            &Self::predicate_backward_set_name(conclusion),
            &record,
        )?;
        Ok(())
    }

    pub fn store_predicate_implication(
        &mut self,
        implication: &PredicateFactor,
    ) -> Result<(), Box<dyn Error>> {
        self.store_implication(implication)?;
        self.store_predicate_backward_link(implication)?;
        Ok(())
    }

    pub fn store_predicate_implications(
        &mut self,
        implications: &Vec<PredicateFactor>,
    ) -> Result<(), Box<dyn Error>> {
        for implication in implications {
            self.store_predicate_implication(implication)?;
        }
        Ok(())
    }

    pub fn get_all_implications(&self) -> Result<Vec<PredicateFactor>, Box<dyn Error>> {
        let set_members: Vec<String> = set_members(
            &mut *self.redis_connection.borrow_mut(),
            &self.namespace,
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
    ) -> Result<Vec<PredicateFactor>, Box<dyn Error>> {
        let set_members: Vec<String> = set_members(
            &mut *self.redis_connection.borrow_mut(),
            &self.namespace,
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
