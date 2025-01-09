use redis::Connection;

use super::{
    inference::{compute_each_combination, groups_from_backlinks, Inferencer},
    table::{GenericNodeType, PropositionNode},
};
use crate::{
    inference::inference::build_factor_context_for_assignment,
    model::{objects::unary_existence_function, weights::CLASS_LABELS},
    print_blue, print_green, print_red, print_yellow,
};
use std::error::Error;

impl Inferencer {
    pub fn initialize_lambda(&mut self) -> Result<(), Box<dyn Error>> {
        trace!("initialize_lambda: proposition");
        for node in &self.proposition_graph.all_nodes {
            trace!("initializing: {}", node.debug_string());
            for outcome in CLASS_LABELS {
                self.data.set_lambda_value(node, outcome, 1f64);
            }
            for parent in &self.proposition_graph.get_all_backward(node) {
                trace!(
                    "initializing lambda link from {} to {}",
                    node.debug_string(),
                    parent.debug_string()
                );
                for outcome in CLASS_LABELS {
                    self.data.set_lambda_message(node, parent, outcome, 1f64);
                }
            }
        }
        Ok(())
    }

    pub fn do_lambda_traversal(
        &mut self,
        connection: &mut Connection,
    ) -> Result<(), Box<dyn Error>> {
        let mut bfs_order = self.bfs_order.clone();
        bfs_order.reverse();
        trace!("send_lambda_messages bfs_order: {:?}", &bfs_order);
        for node in &bfs_order {
            trace!("send pi bfs selects {:?}", node);
            self.lambda_visit_node(connection, node)?;
        }
        Ok(())
    }

    pub fn lambda_visit_node(
        &mut self,
        connection: &mut Connection,
        from_node: &PropositionNode,
    ) -> Result<(), Box<dyn Error>> {
        self.lambda_send_messages(connection, from_node)?;
        let is_observed = self.is_observed(connection, from_node)?;
        trace!(
            "lambda_visit_node {:?} is_observed {}",
            from_node,
            is_observed
        );
        if is_observed {
            self.lambda_set_from_evidence(connection, from_node)?;
        } else {
            self.lambda_compute_value(connection, &from_node)?;
        }
        Ok(())
    }

    pub fn lambda_set_from_evidence(
        &mut self,
        connection: &mut Connection,
        node: &PropositionNode,
    ) -> Result<(), Box<dyn Error>> {
        let as_single = node.extract_single();
        let probability = self
            .fact_memory
            .get_proposition_probability(connection, &as_single)?
            .unwrap();
        trace!("set from evidence {:?} {}", node, probability);
        self.data.set_lambda_value(node, 1, probability);
        self.data.set_lambda_value(node, 0, 1f64 - probability);
        Ok(())
    }

    pub fn lambda_compute_value(
        &mut self,
        connection: &mut Connection,
        node: &PropositionNode,
    ) -> Result<(), Box<dyn Error>> {
        let is_observed = self.is_observed(connection, node)?;
        assert!(!is_observed);
        let children = self.proposition_graph.get_all_forward(node);
        for class_label in &CLASS_LABELS {
            let mut product = 1f64;
            for (_child_index, child_node) in children.iter().enumerate() {
                let child_lambda = self
                    .data
                    .get_lambda_message(&child_node, node, *class_label)
                    .unwrap();
                product *= child_lambda;
            }
            self.data.set_lambda_value(&node, *class_label, product);
        }
        Ok(())
    }

    pub fn lambda_send_messages(
        &mut self,
        connection: &mut Connection,
        node: &PropositionNode,
    ) -> Result<(), Box<dyn Error>> {
        let parent_nodes = self.proposition_graph.get_all_backward(node);
        trace!(
            "lambda_send_generic for node {:?} with parents {:?}",
            node,
            &parent_nodes
        );
        let all_combinations = compute_each_combination(&parent_nodes);
        let lambda_true = self.data.get_lambda_value(node, 1).unwrap();
        let lambda_false = self.data.get_lambda_value(node, 0).unwrap();
        for (to_index, to_parent) in parent_nodes.iter().enumerate() {
            trace!("to_index {} to_parent {:?}", to_index, to_parent);
            let mut sum_true = 0f64;
            let mut sum_false = 0f64;
            for combination in &all_combinations {
                let mut pi_product = 1f64;
                for (other_index, other_parent) in parent_nodes.iter().enumerate() {
                    if other_index != to_index {
                        let class_bool = combination.get(other_parent).unwrap();
                        let class_label = if *class_bool { 1 } else { 0 };
                        let this_pi = self
                            .data
                            .get_pi_message(&other_parent, node, class_label)
                            .unwrap();
                        trace!(
                            "using pi message parent {:?}, node {:?}, label {}: pi={}",
                            &other_parent,
                            node,
                            class_label,
                            this_pi
                        );
                        pi_product *= this_pi;
                    }
                }
                let probability_true =
                    self.score_factor_assignment(connection, &parent_nodes, combination, node)?;
                let probability_false = 1f64 - probability_true;
                trace!(
                    "probability {} for {:?} on assignment {:?}",
                    probability_true,
                    node,
                    combination
                );
                let parent_assignment = combination.get(to_parent).unwrap();
                let true_factor = probability_true * pi_product * lambda_true;
                let false_factor = probability_false * pi_product * lambda_false;
                if *parent_assignment {
                    sum_true += true_factor + false_factor;
                } else {
                    sum_false += true_factor + false_factor;
                }
            }
            trace!(
                "final 1 lambda message {} from {:?} to {:?}",
                sum_true,
                node,
                to_parent
            );
            trace!(
                "final 0 lambda message {} from {:?} to {:?}",
                sum_false,
                node,
                to_parent
            );
            self.data.set_lambda_message(node, to_parent, 1, sum_true);
            self.data.set_lambda_message(node, to_parent, 0, sum_false);
        }
        Ok(())
    }
}
