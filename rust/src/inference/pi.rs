use std::error::Error;
use crate::{print_red, print_yellow, model::{objects::EXISTENCE_FUNCTION, weights::CLASS_LABELS}};
use super::{inference::Inferencer, table::PropositionNode};

impl Inferencer {
    pub fn send_pi_messages(&mut self) -> Result<(), Box<dyn Error>> {
        let bfs_order = self.bfs_order.clone();
        print_red!("send_pi_messages bfs_order: {:?}", &bfs_order);
        for node in &bfs_order {
            print_yellow!("send pi bfs selects {:?}", node);
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
            .model
            .proposition_db
            .get_proposition_probability(&as_single)?
            .unwrap();
        self.data
            .set_pi_value(node, 1, probability);
        self.data
            .set_pi_value(node, 0, 1f64 - probability);
        Ok(())
    }

    pub fn pi_visit_node(&mut self, from_node: &PropositionNode) -> Result<(), Box<dyn Error>> {
        // Part 1: Compute pi for this node.
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
        // Part 2: For each value of z, compute pi_X(z)
        let forward_groups = self.proposition_graph.get_all_forward(from_node);
        for (this_index, to_node) in forward_groups.iter().enumerate() {
            for class_label in &CLASS_LABELS {
                let mut lambda_part = 1f64;
                for (other_index, other_node) in forward_groups.iter().enumerate() {
                    if other_index != this_index {
                        let this_lambda = self
                            .data
                            .get_lambda_value(&other_node, *class_label)
                            .unwrap();
                        lambda_part *= this_lambda;
                    }
                }
                let pi_part = self.data.get_pi_value(&from_node, *class_label).unwrap();
                let message = pi_part * lambda_part;
                self.data
                    .set_pi_message(&from_node, &to_node, *class_label, message);
            }
        }
        // Success.
        Ok(())
    }
}