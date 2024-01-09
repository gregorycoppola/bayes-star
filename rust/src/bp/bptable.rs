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
        let mut lambda_values: HashMap<(String, usize), f64> = HashMap::new();
        let mut pi_values: HashMap<String, f64> = HashMap::new();

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

    // A stub implementation for `send_pi_msg`.
    pub fn send_pi_msg(
        &self,
        from: &Proposition,
        to: &Proposition,
        pi_values: &HashMap<String, f64>,
        lambda_values: &mut HashMap<(String, usize), f64>,
    ) -> Result<(), Box<dyn Error>> {
        // Get the pi value for the 'from' Proposition.
        let from_pi = pi_values
            .get(&from.search_string())
            .ok_or_else(|| "Pi value for 'from' Proposition not found")?;

        // Get the conditional probability of 'to' given 'from'.
        // This function `get_conditional_probability` is assumed to be defined elsewhere.
        let conditional_probability = self.get_conditional_probability(from, to);

        // Calculate the new pi value for 'to'.
        // In a real scenario, this should be more complex, taking into account all possible values of 'from'.
        // For simplicity, we assume binary propositions.
        let to_pi = from_pi * conditional_probability;

        // Update the pi value for 'to' in `pi_values`.
        // Since `pi_values` is not mutable, we cannot update it directly.
        // If it needs to be updated, consider changing the function signature or using another method.

        // Update lambda values for 'to'. This involves combining the new pi value with existing lambda values.
        for value_index in CLASS_LABELS.iter() {
            let lambda_key = (to.search_string(), *value_index);
            if let Some(lambda) = lambda_values.get_mut(&lambda_key) {
                // Combine the existing lambda value with the new pi value.
                // This is a placeholder for the actual combination logic, which will depend on your specific use case.
                *lambda *= to_pi;
            } else {
                // If there is no lambda value for 'to', insert a new one.
                lambda_values.insert(lambda_key, to_pi);
            }
        }

        Ok(())
    }

    // A stub implementation for `send_lambda_message`.
    pub fn send_lambda_message(
        &self,
        from: &Proposition,
        to: &Proposition,
        lambda_values: &mut HashMap<(String, usize), f64>,
        pi_values: &mut HashMap<String, f64>,
    ) -> Result<(), Box<dyn Error>> {
        // Get the lambda value for the 'from' Proposition.
        let mut from_lambda = 1.0;
        for value_index in CLASS_LABELS.iter() {
            let lambda_key = (from.search_string(), *value_index);
            from_lambda *= lambda_values
                .get(&lambda_key)
                .ok_or_else(|| "Lambda value for 'from' Proposition not found")?;
        }

        // Get the conditional probability of 'from' given 'to'.
        // This function `get_conditional_probability` is assumed to be defined elsewhere.
        let conditional_probability = self.get_conditional_probability(to, from);

        // Calculate the new lambda value for 'to'.
        // The lambda value is a product of the lambda value from 'from' and the conditional probability.
        let to_lambda = from_lambda * conditional_probability;

        // Update the lambda value for 'to' in `lambda_values`.
        // This is a simplified version, assuming binary propositions.
        for value_index in CLASS_LABELS.iter() {
            let lambda_key = (to.search_string(), *value_index);
            if let Some(lambda) = lambda_values.get_mut(&lambda_key) {
                // Combine the existing lambda value with the new lambda value.
                // This is a placeholder for the actual combination logic, which will depend on your specific use case.
                *lambda *= to_lambda;
            } else {
                // If there is no lambda value for 'to', insert a new one.
                lambda_values.insert(lambda_key, to_lambda);
            }
        }

        // Assuming pi_values need to be updated with new lambda values.
        // Here we adjust the pi_values for 'to' Proposition, though the specific update rule will depend on your use case.
        let to_pi = pi_values
            .get(&to.search_string())
            .ok_or_else(|| "Pi value for 'to' Proposition not found")?;
        pi_values.insert(to.search_string(), to_pi * to_lambda);

        Ok(())
    }

    pub fn get_all_propositions(&self) -> Vec<Proposition> {
        todo!()
    }
    pub fn get_proposition_probability(&self, proposition: &Proposition) -> f64 {
        todo!()
    }
    pub fn get_conditional_probability(
        &self,
        conclusion: &Proposition,
        premise: &Proposition,
    ) -> f64 {
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
