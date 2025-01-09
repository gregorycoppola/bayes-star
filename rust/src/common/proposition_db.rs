use crate::{
    common::{interface::BeliefTable, redis::map_insert},
    inference::table::PropositionNode,
    model::{
        self,
        exponential::ExponentialModel,
        objects::{
            Domain, Entity, Predicate, ImplicationFactor, PredicateGroup, Proposition,
            unary_existence_function,
        },
    },
};
use redis::{Commands, Connection};
use std::{cell::RefCell, collections::HashMap, error::Error, io::Empty, rc::Rc, sync::{Arc, Mutex}};

use super::{
    graph::InferenceGraph,
    interface::{PredictStatistics, TrainStatistics},
    redis::{map_get, RedisManager}, resources::NamespaceBundle,
};

pub struct RedisBeliefTable {
    redis_connection: Arc<Mutex<redis::Connection>>,
    namespace: String,
}

impl RedisBeliefTable {
    pub fn new_mutable(resources: &NamespaceBundle) -> Result<Box<dyn BeliefTable>, Box<dyn Error>> {
        let redis_connection = resources.connection.clone();
        let namespace = resources.namespace.clone();
        Ok(Box::new(RedisBeliefTable { redis_connection, namespace }))
    }
    pub fn new_shared(resources: &NamespaceBundle) -> Result<Rc<dyn BeliefTable>, Box<dyn Error>> {
        let redis_connection = resources.connection.clone();
        let namespace = resources.namespace.clone();
        Ok(Rc::new(RedisBeliefTable { redis_connection, namespace }))
    }
    pub const PROBABILITIES_KEY: &'static str = "probabilities";
}

impl BeliefTable for RedisBeliefTable {
    // Return Some if the probability exists in the table, or else None.
    fn get_proposition_probability(
        &self,
        proposition: &Proposition,
    ) -> Result<Option<f64>, Box<dyn Error>> {
        if proposition.predicate.relation == unary_existence_function() {
            return Ok(Some(1f64));
        }
        let hash_string = proposition.predicate.hash_string();

        let probability_record = map_get(
            &mut self.redis_connection.borrow_mut(),
            &self.namespace,
            Self::PROBABILITIES_KEY,
            &hash_string,
        )?
        .expect("should be there");
        let probability = probability_record
            .parse::<f64>()
            .map_err(|e| Box::new(e) as Box<dyn Error>)?;
        Ok(Some(probability))

        // // Use a match statement to handle the different outcomes
        // match self
        //     .redis_connection
        //     .borrow_mut()
        //     .hget::<_, _, String>("probs", &hash_string)
        // {
        //     Ok(probability_str) => {
        //         // Found the entry, parse it
        //         let probability = probability_str
        //             .parse::<f64>()
        //             .map_err(|e| Box::new(e) as Box<dyn Error>)?;
        //         Ok(Some(probability))
        //     }
        //     Err(e) => {
        //         // Handle specific "not found" error
        //         if e.kind() == redis::ErrorKind::TypeError {
        //             // Entry not found in Redis
        //             Ok(None)
        //         } else {
        //             // Other Redis errors
        //             Err(Box::new(e) as Box<dyn Error>)
        //         }
        //     }
        // }
    }

    fn store_proposition_probability(
        &self,
        proposition: &Proposition,
        probability: f64,
    ) -> Result<(), Box<dyn Error>> {
        trace!("GraphicalModel::store_proposition_probability - Start. Input proposition: {:?}, probability: {}", proposition, probability);

        let hash_string = proposition.predicate.hash_string();
        trace!(
            "GraphicalModel::store_proposition_probability - Computed hash_string: {}",
            hash_string
        );

        map_insert(
            &mut self.redis_connection.borrow_mut(),
            &self.namespace,
            Self::PROBABILITIES_KEY,
            &hash_string,
            &probability.to_string(),
        )?;

        // if let Err(e) = self
        //     .redis_connection
        //     .borrow_mut()
        //     .hset::<&str, &str, String, bool>("probs", &hash_string, probability.to_string())
        // {
        //     trace!(
        //         "GraphicalModel::store_proposition_probability - Error storing probability in Redis: {}",
        //         e
        //     );
        //     return Err(Box::new(e));
        // }

        // trace!("GraphicalModel::store_proposition_probability - Completed successfully");
        Ok(())
    }
}

pub struct EmptyBeliefTable;

impl EmptyBeliefTable {
    pub fn new_shared(client: &RedisManager) -> Result<Rc<dyn BeliefTable>, Box<dyn Error>> {
        let redis_connection = client.get_connection()?;
        Ok(Rc::new(EmptyBeliefTable {}))
    }
}

impl BeliefTable for EmptyBeliefTable {
    // Return Some if the probability exists in the table, or else None.
    fn get_proposition_probability(
        &self,
        proposition: &Proposition,
    ) -> Result<Option<f64>, Box<dyn Error>> {
        if proposition.predicate.relation == unary_existence_function() {
            return Ok(Some(1f64));
        }
        Ok(None)
    }

    fn store_proposition_probability(
        &self,
        proposition: &Proposition,
        probability: f64,
    ) -> Result<(), Box<dyn Error>> {
        panic!("Shouldn't call this.")
    }
}

pub struct HashMapBeliefTable {
    evidence: RefCell<HashMap<PropositionNode, f64>>,
}

impl HashMapBeliefTable {
    pub fn new() -> Rc<HashMapBeliefTable> {
        Rc::new(HashMapBeliefTable {
            evidence: RefCell::new(HashMap::new()),
        })
    }

    pub fn clear(&self, node: &PropositionNode) -> () {
        self.evidence.borrow_mut().remove(node);
    }
}

impl BeliefTable for HashMapBeliefTable {
    fn get_proposition_probability(
        &self,
        proposition: &Proposition,
    ) -> Result<Option<f64>, Box<dyn Error>> {
        if proposition.predicate.relation == unary_existence_function() {
            return Ok(Some(1f64));
        }
        let node = PropositionNode::from_single(proposition);
        let map = self.evidence.borrow();
        let result = map.get(&node);
        Ok(result.copied())
    }

    fn store_proposition_probability(
        &self,
        proposition: &Proposition,
        probability: f64,
    ) -> Result<(), Box<dyn Error>> {
        let node = PropositionNode::from_single(proposition);
        // Use `borrow_mut` to get a mutable reference to the HashMap
        self.evidence.borrow_mut().insert(node, probability);
        Ok(())
    }
}
