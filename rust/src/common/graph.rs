use super::{
    interface::{PredictStatistics, TrainStatistics},
    redis::{set_value, RedisManager},
    resources::NamespaceBundle,
};
use crate::{
    common::{
        interface::BeliefTable,
        redis::{get_value, is_member, set_add, set_members},
    },
    model::{
        self,
        choose::{
            extract_existence_factor_for_predicate, extract_existence_factor_for_proposition,
        },
        exponential::ExponentialModel,
        objects::{
            Domain, Entity, ImplicationFactor, Predicate, PredicateGroup, Proposition, PropositionGroup, Relation
        },
    },
    print_blue,
};
use redis::{Commands, Connection};
use serde::{Deserialize, Serialize};
use std::{cell::RefCell, error::Error, rc::Rc, sync::{Arc, Mutex}};
pub struct InferenceGraph {
    namespace: String,
}

impl InferenceGraph {
    pub fn new_mutable(namespace: String) -> Result<Box<Self>, Box<dyn Error>> {
        Ok(Box::new(InferenceGraph {
            namespace,
        }))
    }

    pub fn new_shared(namespace: String) -> Result<Arc<Self>, Box<dyn Error>> {
        Ok(Arc::new(InferenceGraph {
            namespace,
        }))
    }

    pub fn new_literal(redis_connection: Arc<Mutex<redis::Connection>>, namespace: String) -> Result<Self, Box<dyn Error>> {
        Ok(InferenceGraph {
            namespace,
        })
    }

    pub fn register_experiment(&mut self, connection: &mut Connection, experiment_name: &str) -> Result<(), Box<dyn Error>> {
        set_add(
            connection,
            &self.namespace,
            &Self::experiment_set_name(),
            experiment_name,
        )?;
        Ok(())
    }

    pub fn get_all_experiments(&self, connection: &mut Connection) -> Result<Vec<String>, Box<dyn Error>> {
        let set_members: Vec<String> = set_members(
            connection,
            &self.namespace,
            &Self::experiment_set_name(),
        )?;
        Ok(set_members)
    }

    pub fn register_relation(&mut self, connection: &mut Connection, relation: &Relation) -> Result<(), Box<dyn Error>> {
        let record = serialize_record(relation)?;
        set_add(
            connection,
            &self.namespace,
            &Self::relation_set_name(),
            &record,
        )?;
        Ok(())
    }

    pub fn check_relation(&mut self, connection: &mut Connection, relation: &Relation) -> Result<(), Box<dyn Error>> {
        // TODO: impelment this
        Ok(())
    }

    pub fn get_all_relations(&self, connection: &mut Connection) -> Result<Vec<Relation>, Box<dyn Error>> {
        let set_members: Vec<String> = set_members(
            connection,
            &self.namespace,
            &Self::relation_set_name(),
        )?;
        set_members
            .into_iter()
            .map(|record| serde_json::from_str(&record).map_err(|e| Box::new(e) as Box<dyn Error>))
            .collect()
    }

    pub fn register_domain(&mut self, connection: &mut Connection, domain: &String) -> Result<(), Box<dyn Error>> {
        set_add(
            connection,
            &self.namespace,
            "domains",
            domain,
        )?;
        Ok(())
    }

    pub fn check_domain(&self, connection: &mut Connection, domain: &String) -> Result<(), Box<dyn Error>> {
        let result = is_member(
             connection,
            &self.namespace,
            "domains",
            domain,
        )?;
        assert!(result);
        Ok(())
    }

    pub fn get_all_domains(&self, connection: &mut Connection) -> Result<Vec<String>, Box<dyn Error>> {
        let result = set_members(
            connection,
            &self.namespace,
            "domains",
        )?;
        Ok(result)
    }

    pub fn register_target(&mut self, connection: &mut Connection, target: &Proposition) -> Result<(), Box<dyn Error>> {
        let record = serialize_record(target)?;
        set_value(
            connection,
            &self.namespace,
            &Self::target_key_name(),
            &record,
        )?;
        Ok(())
    }

    pub fn get_target(&self, connection: &mut Connection) -> Result<Proposition, Box<dyn Error>> {
        let record = get_value(
            connection,
            &self.namespace,
            &Self::target_key_name(),
        )?.unwrap();
        serde_json::from_str(&record).map_err(|e| Box::new(e) as Box<dyn Error>)
    }

    pub fn store_entity(&mut self, connection: &mut Connection, entity: &Entity) -> Result<(), Box<dyn Error>> {
        trace!(
            "Storing entity in domain '{}': {}",
            entity.domain,
            entity.name
        );
        self.check_domain(connection, &entity.domain)?;
        // NOTE: this is a "set" named after the "domain", with each "entity.name" inside of it.
        set_add(
            connection,
            &self.namespace,
            &entity.domain.to_string(),
            &entity.name,
        )?;
        Ok(())
    }

    pub fn get_entities_in_domain(&self, connection: &mut Connection, domain: &String) -> Result<Vec<Entity>, Box<dyn Error>> {
        let domain_string = domain.to_string();
        let names: Vec<String> = set_members(
            connection,
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

    fn relation_set_name() -> String {
        "relations".to_string()
    }

    fn experiment_set_name() -> String {
        "experiments".to_string()
    }

    fn target_key_name() -> String {
        "target".to_string()
    }

    fn store_implication(&mut self, connection: &mut Connection, implication: &ImplicationFactor) -> Result<(), Box<dyn Error>> {
        let record = serialize_record(implication)?;
        set_add(
            connection,
            &self.namespace,
            &Self::implication_seq_name(),
            &record,
        )?;
        Ok(())
    }

    // TODO: I feel like this should not be public.
    pub fn ensure_existence_backlinks_for_proposition(
        &mut self, connection: &mut Connection,
        proposition: &Proposition,
    ) -> Result<(), Box<dyn Error>> {
        let implication = extract_existence_factor_for_proposition(proposition)?;
        self.store_predicate_implication(connection, &implication)?;
        Ok(())
    }

    fn store_predicate_backward_link(
        &mut self, connection: &mut Connection,
        inference: &ImplicationFactor,
    ) -> Result<(), Box<dyn Error>> {
        let conclusion = &inference.conclusion;
        let record = serialize_record(inference)?;
        set_add(
            connection,
            &self.namespace,
            &Self::predicate_backward_set_name(conclusion),
            &record,
        )?;
        Ok(())
    }

    pub fn store_predicate_implication(
        &mut self, connection: &mut Connection,
        implication: &ImplicationFactor,
    ) -> Result<(), Box<dyn Error>> {
        self.store_implication(connection, implication)?;
        self.store_predicate_backward_link(connection, implication)?;
        Ok(())
    }

    pub fn store_predicate_implications(
        &mut self, connection: &mut Connection,
        implications: &Vec<ImplicationFactor>,
    ) -> Result<(), Box<dyn Error>> {
        for implication in implications {
            self.store_predicate_implication(connection, implication)?;
        }
        Ok(())
    }

    pub fn get_all_implications(&self, connection: &mut Connection) -> Result<Vec<ImplicationFactor>, Box<dyn Error>> {
        let set_members: Vec<String> = set_members(
            connection,
            &self.namespace,
            &Self::implication_seq_name(),
        )?;
    
        set_members
            .into_iter()
            .map(|record| {
                warn!("Deserializing record: {}", record);  // Log each record before deserialization
                serde_json::from_str::<ImplicationFactor>(&record)
                    .map_err(|e| {
                        warn!("Failed to deserialize record: {}, Error: {}", record, e);  // Log if deserialization fails
                        Box::new(e) as Box<dyn Error>
                    })
            })
            .collect()
    }
    

    pub fn predicate_backward_links(
        &self, connection: &mut Connection,
        conclusion: &Predicate,
    ) -> Result<Vec<ImplicationFactor>, Box<dyn Error>> {
        let set_members: Vec<String> = set_members(
            connection,
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
