use std::{collections::HashMap, error::Error};

use redis::Connection;

use crate::model::{objects::Proposition, weights::CLASS_LABELS};

pub struct BeliefPropagator {
    redis_connection: redis::Connection,
}

impl Drop for BeliefPropagator {
    fn drop(&mut self) {
        // The Drop trait for Arc<Client> will automatically be called here,
        // reducing the reference count. If this Storage instance holds the last
        // reference to the client, the client will be dropped and its resources
        // (like network connections) will be cleaned up.
    }
}

impl BeliefPropagator {
    // Initialize new Storage with a Redis connection
    pub fn new(connection: Connection) -> Result<Self, redis::RedisError> {
        Ok(BeliefPropagator {
            redis_connection: connection,
        })
    }

    // have to depend on graph, and Propositions -> values
    pub fn initialize(&self) -> Result<(), Box<dyn Error>> {
        let mut lambda_values:HashMap<(String, usize), f64> = HashMap::new();
        let mut pi_values:HashMap<String, f64> = HashMap::new();

        // Assuming each Proposition is an enum or struct that can be iterated over its values
        let all_propositions = self.get_all_propositions();
        for x in all_propositions.iter() {
            for x_value in CLASS_LABELS {
                lambda_values.insert((x.search_string(), x_value.clone()), 1.0);
            }
            // Assuming you have a way to find the parent of a Proposition `x`
            if let Some(parent) = self.find_parent(&x) {
                for z_value in CLASS_LABELS {
                    lambda_values.insert((parent.search_string(), z_value.clone()), 1.0);
                }
            }
        }

        // Assuming `root` is a special Proposition that is the root of the Bayesian Network
        let root = self.find_root();
        for r_value in CLASS_LABELS {
            let probability = self.get_proposition_probability(&root); // This would need to be defined
            pi_values.insert(root.search_string(), probability);
        }

        // Assuming `send_pi_msg` is defined to handle sending the \(\pi\) message
        for child in self.find_children(&root) {
            self.send_pi_msg(&root, &child, &pi_values, &mut lambda_values);
        }
        Ok(())
    }

    // have to depend on graph, and Propositions -> values
    pub fn update_tree(&self) -> Result<(), Box<dyn Error>> {
        todo!()
    }

    pub fn send_pi_msg(
        &self,
        from: &Proposition,
        to: &Proposition,
        pi_values: &HashMap<String, f64>,
        lambda_values: &mut HashMap<(String, usize), f64>,
    ) -> Result<(), Box<dyn Error>> {
        todo!()
    }

    pub fn send_lambda_message(
        &self,
        from: &Proposition,
        to: &Proposition,
    ) -> Result<(), Box<dyn Error>> {
        todo!()
    }

    pub fn get_all_propositions(&self) -> Vec<Proposition> {
        todo!()
    }
    pub fn get_proposition_probability(&self, proposition: &Proposition) -> f64 {
        todo!()
    }
    fn find_parent(&self, x: &Proposition) -> Option<Proposition> {
        // Implementation goes here
        None
    }

    fn find_root(&self) -> Proposition {
        todo!()
    }

    fn find_children(&self, root: &Proposition) -> Vec<Proposition> {
        // Implementation goes here
        Vec::new() // Placeholder
    }
}
