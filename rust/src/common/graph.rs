use crate::{
    common::interface::FactDB,
    model::{
        self,
        maxent::ExponentialModel,
        objects::{Conjunction, Domain, Entity, Implication, Proposition, ImplicationInstance},
    },
};
use redis::{Commands, Connection};
use std::{cell::RefCell, error::Error};

use super::{
    interface::{PredictStatistics, TrainStatistics},
    redis::RedisClient,
};

pub struct Graph {
    redis_connection: RefCell<redis::Connection>,
}

impl Graph {
    // Initialize new GraphicalModel with a Redis connection
    pub fn new(redis: &RedisClient) -> Result<Self, Box<dyn Error>> {
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
        self.redis_connection
            .borrow_mut()
            .sadd(&entity.domain.to_string(), &entity.name)
            .map_err(|e| Box::new(e) as Box<dyn Error>)?;
        Ok(())
    }

    pub fn get_entities_in_domain(&self, domain: &str) -> Result<Vec<Entity>, Box<dyn Error>> {
        trace!("Getting entities in domain '{}'", domain.clone()); // Logging

        let names: Vec<String> = self
            .redis_connection
            .borrow_mut()
            .smembers(domain)
            .map_err(|e| Box::new(e) as Box<dyn Error>)?;
        Ok(names
            .into_iter()
            .map(|name| Entity {
                domain: Domain::from_str(domain).expect("Domain not recognized."), // Use the provided domain
                name,
            })
            .collect())
    }

    pub fn store_link(&mut self, implication: &Implication) -> Result<(), Box<dyn Error>> {
        let record =
            serde_json::to_string(implication).map_err(|e| Box::new(e) as Box<dyn Error>)?;

        self.redis_connection
            .borrow_mut()
            .sadd("links", &record)
            .map_err(|e| Box::new(e) as Box<dyn Error>)?;

        self.store_links(implication)
    }

    pub fn store_links(&mut self, implication: &Implication) -> Result<(), Box<dyn Error>> {
        let search_string = implication.conclusion_string();
        let record =
            serde_json::to_string(implication).map_err(|e| Box::new(e) as Box<dyn Error>)?;

        self.redis_connection
            .borrow_mut()
            .sadd(&search_string, &record)
            .map_err(|e| Box::new(e) as Box<dyn Error>)?;

        Ok(())
    }

    // Get all Implications
    pub fn get_all_links(&self) -> Result<Vec<Implication>, Box<dyn Error>> {
        let all_values: Vec<String> = self
            .redis_connection
            .borrow_mut()
            .smembers("links")
            .map_err(|e| Box::new(e) as Box<dyn Error>)?;

        all_values
            .into_iter()
            .map(|record| serde_json::from_str(&record).map_err(|e| Box::new(e) as Box<dyn Error>))
            .collect()
    }

    pub fn find_premises(&self, search_string: &str) -> Result<Vec<Implication>, Box<dyn Error>> {
        trace!("find_premises: {:?}", &search_string);
        let set_members: Vec<String> = self
            .redis_connection
            .borrow_mut()
            .smembers(search_string)
            .map_err(|e| Box::new(e) as Box<dyn Error>)?;

        set_members
            .into_iter()
            .map(|record| serde_json::from_str(&record).map_err(|e| Box::new(e) as Box<dyn Error>))
            .collect()
    }


    pub fn find_roots(&self) -> Result<Vec<Proposition>, Box<dyn Error>> {
        todo!()
    }

    pub fn parents_of_proposition(&self, x: &Proposition) -> Result<Vec<Conjunction>, Box<dyn Error>> {
        todo!()
    }

    pub fn children_of_proposition(&self, root: &Proposition) -> Result<Vec<Conjunction>, Box<dyn Error>> {
        todo!()
    }

    pub fn parents_of_conjunct(&self, x: &Conjunction) -> Result<Vec<Proposition>, Box<dyn Error>> {
        todo!()
    }

    pub fn children_of_conjunct(&self, root: &Conjunction) -> Result<Vec<Proposition>, Box<dyn Error>> {
        todo!()
    }
}