use super::{
    inference::{compute_each_combination, groups_from_backlinks, Inferencer},
    table::{GenericNodeType, PropositionNode},
};
use crate::{
    inference::inference::build_factor_context_for_assignment,
    model::{objects::EXISTENCE_FUNCTION, weights::CLASS_LABELS},
    print_blue, print_green, print_red,
};
use std::error::Error;

impl Inferencer {
    pub fn do_pi_traversal(&mut self) -> Result<(), Box<dyn Error>> {
        let bfs_order = self.bfs_order.clone();
        trace!("send_pi_messages bfs_order: {:?}", &bfs_order);
        for node in &bfs_order {
            trace!("send pi bfs selects {:?}", node);
            self.pi_visit_node(node)?;
        }
        Ok(())
    }

    fn pi_compute_root(&mut self, node: &PropositionNode) -> Result<(), Box<dyn Error>> {
        let root = node.extract_single();
        assert_eq!(root.predicate.function, EXISTENCE_FUNCTION.to_string());
        self.data
            .set_pi_value(&PropositionNode::from_single(&root), 1, 1.0f64);
        self.data
            .set_pi_value(&PropositionNode::from_single(&root), 0, 0.0f64);

        Ok(())
    }

    pub fn pi_set_from_evidence(&mut self, node: &PropositionNode) -> Result<(), Box<dyn Error>> {
        let as_single = node.extract_single();
        let probability = self
            .fact_memory
            .get_proposition_probability(&as_single)?
            .unwrap();
        self.data.set_pi_value(node, 1, probability);
        self.data.set_pi_value(node, 0, 1f64 - probability);
        Ok(())
    }

    pub fn pi_send_messages(&mut self, node: &PropositionNode) -> Result<(), Box<dyn Error>> {
        // Part 2: For each value of z, compute pi_X(z)
        let forward_groups = self.proposition_graph.get_all_forward(node);
        for (this_index, to_node) in forward_groups.iter().enumerate() {
            for class_label in &CLASS_LABELS {
                let mut lambda_part = 1f64;
                for (other_index, other_child) in forward_groups.iter().enumerate() {
                    if other_index != this_index {
                        // This should be a message.
                        let this_lambda = self
                            .data
                            .get_lambda_message(&other_child, node, *class_label)
                            .unwrap();
                        lambda_part *= this_lambda;
                    }
                }
                let pi_part = self.data.get_pi_value(&node, *class_label).unwrap();
                let message = pi_part * lambda_part;
                self.data
                    .set_pi_message(&node, &to_node, *class_label, message);
            }
        }
        Ok(())
    }
    pub fn pi_visit_node(&mut self, from_node: &PropositionNode) -> Result<(), Box<dyn Error>> {
        // Compute pi's.
        if !self.is_root(from_node) {
            let is_observed = self.is_observed(from_node)?;
            if is_observed {
                self.pi_set_from_evidence(from_node)?;
            } else {
                self.pi_compute_generic(&from_node)?;
            }
        } else {
            self.pi_compute_root(from_node)?;
        }
        // Compute messages.
        self.pi_send_messages(from_node)?;
        Ok(())
    }

    pub fn pi_compute_generic(&mut self, node: &PropositionNode) -> Result<(), Box<dyn Error>> {
        let parent_nodes = self.proposition_graph.get_all_backward(node);
        let all_combinations = compute_each_combination(&parent_nodes);
        let mut sum_true = 0f64;
        let mut sum_false = 0f64;
        for combination in &all_combinations {
            let mut product = 1f64;
            for (index, parent_node) in parent_nodes.iter().enumerate() {
                let boolean_outcome = combination.get(parent_node).unwrap();
                let usize_outcome = if *boolean_outcome { 1 } else { 0 };
                let pi_x_z = self
                    .data
                    .get_pi_message(parent_node, node, usize_outcome)
                    .unwrap();
                trace!(
                    "getting pi message parent_node {:?}, node {:?}, usize_outcome {}, pi_x_z {}",
                    &parent_node,
                    &node,
                    usize_outcome,
                    pi_x_z,
                );
                product *= pi_x_z;
            }
            let true_marginal =
                self.score_factor_assignment(&parent_nodes, combination, node)?;
            let false_marginal = 1f64 - true_marginal;
            sum_true += true_marginal * product;
            sum_false += false_marginal * product;
        }
        self.data.set_pi_value(node, 1, sum_true);
        self.data.set_pi_value(node, 0, sum_false);
        Ok(())
    }
}
