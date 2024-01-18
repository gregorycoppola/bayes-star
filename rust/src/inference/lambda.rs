use std::error::Error;
use crate::{print_red, print_yellow, model::{objects::EXISTENCE_FUNCTION, weights::CLASS_LABELS}, print_green, print_blue};
use super::{inference::{Inferencer, groups_from_backlinks, compute_each_combination}, table::{PropositionNode, GenericNodeType}};

impl Inferencer {
    pub fn send_lambda_messages(&mut self) -> Result<(), Box<dyn Error>> {
        let mut bfs_order = self.bfs_order.clone();
        bfs_order.reverse();
        print_red!("send_lambda_messages bfs_order: {:?}", &bfs_order);
        for node in &bfs_order {
            print_yellow!("send pi bfs selects {:?}", node);
            self.lambda_visit_node(node)?;
        }
        Ok(())
    }

    pub fn lambda_set_from_evidence(&mut self, node: &PropositionNode) -> Result<(), Box<dyn Error>> {
        let as_single = node.extract_single();
        let probability = self
            .model
            .proposition_db
            .get_proposition_probability(&as_single)?
            .unwrap();
        self.data
            .set_lambda_value(node, 1, probability);
        self.data
            .set_lambda_value(node, 0, 1f64 - probability);
        Ok(())
    }

    pub fn lambda_send_messages(&mut self, from_node: &PropositionNode) -> Result<(), Box<dyn Error>> {
        // Part 2: For each value of z, compute lambda_X(z)
        let backward_groups = self.proposition_graph.get_all_backward(from_node);
        for (this_index, to_node) in backward_groups.iter().enumerate() {
            for class_label in &CLASS_LABELS {
                let mut lambda_part = 1f64;
                for (other_index, other_node) in backward_groups.iter().enumerate() {
                    if other_index != this_index {
                        let this_lambda = self
                            .data
                            .get_lambda_value(&other_node, *class_label)
                            .unwrap();
                        lambda_part *= this_lambda;
                    }
                }
                let lambda_part = self.data.get_lambda_value(&from_node, *class_label).unwrap();
                let message = lambda_part * lambda_part;
                self.data
                    .set_lambda_message(&from_node, &to_node, *class_label, message);
            }
        }
        Ok(())
    }
    pub fn lambda_visit_node(&mut self, from_node: &PropositionNode) -> Result<(), Box<dyn Error>> {
        let is_observed = self.is_observed(from_node)?;
        if is_observed {
            self.lambda_set_from_evidence(from_node)?;
        } else {
            self.lambda_compute_generic(&from_node)?;
        }
        // Success.
        self.lambda_send_messages(from_node)?;
        Ok(())
    }

    pub fn lambda_compute_generic(&mut self, node: &PropositionNode) -> Result<(), Box<dyn Error>> {
        match &node.node {
            GenericNodeType::Single(proposition) => {
                self.lambda_compute_single(node)?;
            }
            GenericNodeType::Group(group) => {
                self.lambda_compute_group(node)?;
            }
        }
        Ok(())
    }

    // from_node is a single.. compute it from the group
    pub fn lambda_compute_single(&mut self, node: &PropositionNode) -> Result<(), Box<dyn Error>> {
        let conclusion = node.extract_single();
        let parent_nodes = self.proposition_graph.get_all_backward(node);
        let premise_groups = groups_from_backlinks(&parent_nodes);
        let all_combinations = compute_each_combination(&parent_nodes);
        let mut sum_true = 0f64;
        let mut sum_false = 0f64;
        for combination in &all_combinations {
            let mut product = 1f64;
            for (index, parent_node) in parent_nodes.iter().enumerate() {
                let boolean_outcome = combination.get(parent_node).unwrap();
                let usize_outcome = if *boolean_outcome { 1 } else { 0 };
                let lambda_x_z = self
                    .data
                    .get_lambda_message(parent_node, node, usize_outcome)
                    .unwrap();
                print_red!(
                    "getting pi message parent_node {:?}, node {:?}, usize_outcome {}, lambda_x_z {}",
                    &parent_node,
                    &node,
                    usize_outcome,
                    lambda_x_z,
                );
                product *= lambda_x_z;
            }
            let factor =
                self.build_factor_context_for_assignment(&premise_groups, combination, &conclusion);
            let prediction = self.model.model.predict(&factor)?;
            print_yellow!("local probability {}  for factor {:?}", &prediction.marginal, &factor);
            let true_marginal = &prediction.marginal;
            let false_marginal = 1f64 - true_marginal;
            sum_true += true_marginal * product;
            sum_false += false_marginal * product;
        }
        self.data.set_lambda_value(node, 1, sum_true);
        self.data.set_lambda_value(node, 0, sum_false);
        Ok(())
    }

    pub fn lambda_compute_group(&mut self, node: &PropositionNode) -> Result<(), Box<dyn Error>> {
        let parent_nodes = self.proposition_graph.get_all_backward(node);
        print_yellow!("lambda_compute_group {:?}", &parent_nodes);
        let all_combinations = compute_each_combination(&parent_nodes);
        let mut sum_true = 0f64;
        let mut sum_false = 0f64;
        for combination in &all_combinations {
            let mut product = 1f64;
            let mut condition = true;
            for (index, parent_node) in parent_nodes.iter().enumerate() {
                let boolean_outcome = combination.get(parent_node).unwrap();
                let usize_outcome = if *boolean_outcome { 1 } else { 0 };
                print_green!(
                    "get pi message: parent_node {:?}, node {:?}, outcome: {}",
                    parent_node,
                    node,
                    usize_outcome
                );
                let lambda_x_z = self
                    .data
                    .get_lambda_message(parent_node, node, usize_outcome)
                    .unwrap();
                print_yellow!(
                    "boolean_outcome {} usize_outcome {} lambda_x_z {}",
                    boolean_outcome,
                    usize_outcome,
                    lambda_x_z
                );
                product *= lambda_x_z;
                let combination_val = combination[parent_node];
                condition = condition && combination_val;
                print_yellow!(
                    "combination_val {} condition {}",
                    combination_val,
                    condition
                );
            }
            if condition {
                print_blue!("true combination: {:?}, product {}", &combination, product);
                sum_true += product;
            } else {
                print_blue!("false combination: {:?}, product {}", &combination, product);
                sum_false += product;
            }
        }
        self.data.set_lambda_value(node, 1, sum_true);
        self.data.set_lambda_value(node, 0, sum_false);
        Ok(())
    }
}