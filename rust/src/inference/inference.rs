use super::{
    graph::{PropositionFactor, PropositionGraph},
    table::{FactorProbabilityTable, HashMapBeliefTable, PropositionNode},
};
use crate::{
    common::{
        interface::BeliefTable,
        model::{FactorContext, InferenceModel},
        proposition_db,
        setup::CommandLineOptions,
    },
    inference::table::GenericNodeType,
    model::{
        objects::{Predicate, PredicateGroup, Proposition, PropositionGroup},
        weights::CLASS_LABELS,
    },
    print_blue, print_green, print_red, print_yellow,
};
use colored::*;
use redis::Connection;
use serde::{Deserialize, Serialize};
use std::io::Write;
use std::{
    borrow::Borrow,
    collections::{HashMap, HashSet, VecDeque},
    error::Error,
    fmt::Display,
    fs::OpenOptions,
    rc::Rc,
    sync::Arc,
};

use std::backtrace::Backtrace;

pub struct Inferencer {
    pub model: Arc<InferenceModel>,
    pub fact_memory: Arc<dyn BeliefTable>,
    pub proposition_graph: Arc<PropositionGraph>,
    pub data: HashMapBeliefTable,
    pub bfs_order: Vec<PropositionNode>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MarginalTable {
    entries: Vec<(String, f64)>,
}

impl MarginalTable {
    pub fn render_marginal_table(&self) -> String {
        let mut entries = self.entries.clone();
        // Sort entries by the string key in alphabetical order
        entries.sort_by(|a, b| a.0.cmp(&b.0));

        // Start HTML table
        let mut html_table = String::from("<table><tr><th>Key</th><th>Value</th></tr>");

        // Add rows to the table
        for (key, value) in entries {
            html_table.push_str(&format!("<tr><td>{}</td><td>{}</td></tr>", key, value));
        }

        // Close HTML table
        html_table.push_str("</table>");

        html_table
    }
}

impl Inferencer {
    pub fn new_mutable(
        model: Arc<InferenceModel>,
        proposition_graph: Arc<PropositionGraph>,
        fact_memory: Arc<dyn BeliefTable>,
    ) -> Result<Box<Self>, redis::RedisError> {
        let bfs_order = proposition_graph.get_bfs_order();
        Ok(Box::new(Inferencer {
            model,
            fact_memory,
            proposition_graph,
            data: HashMapBeliefTable::new(bfs_order.clone()),
            bfs_order,
        }))
    }

    pub fn initialize_chart(&mut self, connection: &mut Connection) -> Result<(), Box<dyn Error>> {
        self.initialize_lambda()?;
        self.do_pi_traversal(connection)?;
        Ok(())
    }

    pub fn do_full_forward_and_backward(
        &mut self,
        connection: &mut Connection,
    ) -> Result<(), Box<dyn Error>> {
        self.do_pi_traversal(connection)?;
        self.do_lambda_traversal(connection)?;
        Ok(())
    }

    pub fn do_fan_out_from_node(
        &mut self,
        connection: &mut Connection,
        node: &PropositionNode,
    ) -> Result<(), Box<dyn Error>> {
        let mut backward_order = self.bfs_order.clone();
        backward_order.reverse();
        let mut started = false;
        for visiting in &backward_order {
            if visiting.underlying_hash == node.underlying_hash {
                started = true;
            }
            if started {
                trace!("will visit {:?}", &visiting);
                self.lambda_visit_node(connection, visiting)?;
            } else {
                trace!("wont visit {:?}", &visiting);
            }
        }
        self.do_pi_traversal(connection)?;
        Ok(())
    }

    pub fn update_marginals(&mut self) -> Result<MarginalTable, Box<dyn Error>> {
        println!("\nMARGINALS");
        let mut entries = vec![];
        for node in &self.bfs_order {
            let pi0 = self.data.get_pi_value(node, 0).unwrap();
            let pi1 = self.data.get_pi_value(node, 1).unwrap();
            let lambda0 = self.data.get_lambda_value(node, 0).unwrap();
            let lambda1 = self.data.get_lambda_value(node, 1).unwrap();
            let potential0 = pi0 * lambda0;
            let potential1 = pi1 * lambda1;
            let norm = potential0 + potential1;
            let probability0 = potential0 / norm;
            let probability1 = potential1 / norm;

            let formatted_prob0 = format!("{:.8}", probability0);
            let formatted_prob1 = format!("{:.8}", probability1);
            println!(
                "{:<12} {:<12} {:?}",
                formatted_prob1.green(),
                formatted_prob0.red(),
                node
            );
            let node_string = format!("{:?}", node);
            let probability = probability1;
            entries.push((node_string, probability));
        }

        // self.log_table_to_file(&table)?;
        let table = MarginalTable { entries };
        Ok(table)
    }

    pub fn build_marginal_table(&self) -> Result<MarginalTable, Box<dyn Error>> {
        let mut entries = vec![];
        for node in &self.bfs_order {
            let pi0 = self.data.get_pi_value(node, 0).unwrap();
            let pi1 = self.data.get_pi_value(node, 1).unwrap();
            let lambda0 = self.data.get_lambda_value(node, 0).unwrap();
            let lambda1 = self.data.get_lambda_value(node, 1).unwrap();
            let potential0 = pi0 * lambda0;
            let potential1 = pi1 * lambda1;
            let norm = potential0 + potential1;
            let probability0 = potential0 / norm;
            let probability1 = potential1 / norm;

            let formatted_prob0 = format!("{:.8}", probability0);
            let formatted_prob1 = format!("{:.8}", probability1);
            let node_string = format!("{:?}", node);
            let probability = probability1;
            entries.push((node_string, probability));
        }
        let table = MarginalTable { entries };
        Ok(table)
    }

    pub fn log_table_to_file(&self) -> Result<MarginalTable, Box<dyn Error>> {
        let table = self.build_marginal_table()?;
        Ok(table)
    }

    pub fn is_root(&self, node: &PropositionNode) -> bool {
        if node.is_single() {
            let as_single = node.extract_single();
            let is_root = self.proposition_graph.roots.contains(&as_single);
            is_root
        } else {
            false
        }
    }

    pub fn is_leaf(&self, node: &PropositionNode) -> bool {
        if node.is_single() {
            let as_single = node.extract_single();
            let forward_links = self
                .proposition_graph
                .single_forward
                .get(&as_single)
                .unwrap();
            forward_links.is_empty()
        } else {
            false
        }
    }

    pub fn is_observed(
        &self,
        connection: &mut Connection,
        node: &PropositionNode,
    ) -> Result<bool, Box<dyn Error>> {
        if node.is_single() {
            let as_single = node.extract_single();
            let has_evidence = self
                .fact_memory
                .get_proposition_probability(connection, &as_single)?
                .is_some();
            trace!(
                "is_observed? node {:?}, has_evidence {}",
                &as_single,
                has_evidence
            );
            Ok(has_evidence)
        } else {
            Ok(false)
        }
    }

    pub fn score_factor_assignment(
        &self,
        connection: &mut Connection,
        premises: &Vec<PropositionNode>,
        premise_assignment: &HashMap<PropositionNode, bool>,
        conclusion: &PropositionNode,
    ) -> Result<f64, Box<dyn Error>> {
        if conclusion.is_single() {
            self.score_factor_assignment_disjunction(
                connection,
                premises,
                premise_assignment,
                conclusion,
            )
        } else {
            self.score_factor_assignment_conjunction(premises, premise_assignment, conclusion)
        }
    }

    pub fn score_factor_assignment_disjunction(
        &self,
        connection: &mut Connection,
        premises: &Vec<PropositionNode>,
        premise_assignment: &HashMap<PropositionNode, bool>,
        conclusion: &PropositionNode,
    ) -> Result<f64, Box<dyn Error>> {
        let mut proposition_premises = vec![];
        for node_premise in premises {
            proposition_premises.push(node_premise.extract_group());
        }
        let proposition_conclusion = conclusion.extract_single();
        let context = build_factor_context_for_assignment(
            &self.proposition_graph,
            &proposition_premises,
            premise_assignment,
            &proposition_conclusion,
        );
        let statistics = self.model.model.predict(connection, &context)?;
        trace!("score_factor_assignment_disjunction; premises: {:?}, assignment: {:?}, conclusion {:?}, probability {}", premises, premise_assignment, conclusion, statistics.probability);
        Ok(statistics.probability)
    }

    pub fn score_factor_assignment_conjunction(
        &self,
        premises: &Vec<PropositionNode>,
        premise_assignment: &HashMap<PropositionNode, bool>,
        conclusion: &PropositionNode,
    ) -> Result<f64, Box<dyn Error>> {
        let mut and_result = true;
        for (_node, value) in premise_assignment {
            and_result &= *value;
        }
        let result = if and_result { 1f64 } else { 0f64 };
        Ok(result)
    }
}

pub fn build_factor_context_for_assignment(
    proposition_graph: &PropositionGraph,
    premises: &Vec<PropositionGroup>,
    premise_assignment: &HashMap<PropositionNode, bool>,
    conclusion: &Proposition,
) -> FactorContext {
    let mut probabilities = vec![];
    let mut factors = vec![];
    for proposition_group in premises {
        let node = PropositionNode::from_group(proposition_group);
        let assignment = *premise_assignment.get(&node).unwrap();
        if assignment {
            probabilities.push(1f64);
        } else {
            probabilities.push(0f64);
        }
        let inference = proposition_graph.get_inference_used(proposition_group, conclusion);
        let factor = PropositionFactor {
            premise: proposition_group.clone(),
            conclusion: conclusion.clone(),
            inference,
        };
        factors.push(factor);
    }
    let context = FactorContext {
        factor: factors,
        probabilities,
    };
    context
}

pub fn compute_each_combination(
    propositions: &Vec<PropositionNode>,
) -> Vec<HashMap<PropositionNode, bool>> {
    trace!("compute_each_combination: propositions={:?}", &propositions);
    let n = propositions.len();
    let mut all_combinations = Vec::new();
    for i in 0..(1 << n) {
        let mut current_combination = HashMap::new();
        for j in 0..n {
            let prop = &propositions[j];
            let state = i & (1 << j) != 0;
            current_combination.insert(prop.clone(), state);
        }
        all_combinations.push(current_combination);
    }
    all_combinations
}

pub fn groups_from_backlinks(backlinks: &Vec<PropositionNode>) -> Vec<PropositionGroup> {
    let mut result = vec![];
    for backlink in backlinks {
        let group = backlink.extract_group();
        result.push(group);
    }
    result
}

pub fn compute_factor_probability_table(
    connection: &mut Connection,
    inferencer: &Inferencer,
    node: &PropositionNode,
) -> Result<FactorProbabilityTable, Box<dyn Error>> {
    let is_observed = inferencer.is_observed(connection, node)?;
    assert!(!is_observed);
    let parent_nodes = inferencer.proposition_graph.get_all_backward(node);
    let all_combinations = compute_each_combination(&parent_nodes);
    let mut buffer = vec![];
    for combination in &all_combinations {
        let true_prob =
            inferencer.score_factor_assignment(connection, &parent_nodes, combination, node)?;
        buffer.push((combination.clone(), true_prob));
    }
    Ok(FactorProbabilityTable{ table: buffer })
}
