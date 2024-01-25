use super::{
    graph::{PropositionFactor, PropositionGraph},
    table::{HashMapBeliefTable, PropositionNode},
};
use crate::{
    common::{
        interface::BeliefTable,
        model::{FactorContext, InferenceModel},
        proposition_db,
        setup::ConfigurationOptions,
    },
    inference::table::GenericNodeType,
    model::{
        objects::{Predicate, PredicateGroup, Proposition, PropositionGroup, EXISTENCE_FUNCTION},
        weights::CLASS_LABELS,
    },
    print_blue, print_green, print_red, print_yellow,
};
use colored::*;
use redis::Connection;
use std::{
    borrow::Borrow,
    collections::{HashMap, HashSet, VecDeque},
    error::Error,
    rc::Rc,
};

use std::backtrace::Backtrace;

pub struct Inferencer {
    pub config: ConfigurationOptions,
    pub model: Rc<InferenceModel>,
    pub fact_memory: Rc<dyn BeliefTable>,
    pub proposition_graph: Rc<PropositionGraph>,
    pub data: HashMapBeliefTable,
    pub bfs_order: Vec<PropositionNode>,
}

impl Inferencer {
    pub fn new_mutable(
        config: &ConfigurationOptions,
        model: Rc<InferenceModel>,
        proposition_graph: Rc<PropositionGraph>,
        fact_memory: Rc<dyn BeliefTable>,
    ) -> Result<Box<Self>, redis::RedisError> {
        let bfs_order = proposition_graph.get_bfs_order();
        Ok(Box::new(Inferencer {
            config: config.clone(),
            model,
            fact_memory,
            proposition_graph,
            data: HashMapBeliefTable::new(bfs_order.clone()),
            bfs_order,
        }))
    }

    pub fn initialize_chart(&mut self) -> Result<(), Box<dyn Error>> {
        self.initialize_lambda()?;
        self.do_pi_traversal()?;
        Ok(())
    }

    pub fn do_full_forward_and_backward(&mut self) -> Result<(), Box<dyn Error>> {
        self.do_pi_traversal()?;
        self.do_lambda_traversal()?;
        Ok(())
    }

    pub fn do_fan_out_from_node(&mut self, node: &PropositionNode) -> Result<(), Box<dyn Error>> {
        let mut backward_order = self.bfs_order.clone();
        backward_order.reverse();
        let mut started = false;
        for visiting in &backward_order {
            if visiting.underlying_hash == node.underlying_hash {
                started = true;
            }
            if started {
                trace!("will visit {:?}", &visiting);
                self.lambda_visit_node(visiting)?;
            } else {
                trace!("wont visit {:?}", &visiting);
            }
        }
        self.do_pi_traversal()?;
        Ok(())
    }

    pub fn update_marginals(&mut self) -> Result<(), Box<dyn Error>> {
        let backtrace = Backtrace::capture();
        println!("{:?}", backtrace);
        trace!("update_marginals over {:?}", &self.bfs_order);
        println!("\nMARGINALS");
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
        }
        Ok(())
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

    pub fn is_observed(&self, node: &PropositionNode) -> Result<bool, Box<dyn Error>> {
        if node.is_single() {
            let as_single = node.extract_single();
            let has_evidence = self
                .fact_memory
                .get_proposition_probability(&as_single)?
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
        premises: &Vec<PropositionNode>,
        premise_assignment: &HashMap<PropositionNode, bool>,
        conclusion: &PropositionNode,
    ) -> Result<f64, Box<dyn Error>> {
        if conclusion.is_single() {
            self.score_factor_assignment_disjunction(premises, premise_assignment, conclusion)
        } else {
            self.score_factor_assignment_conjunction(premises, premise_assignment, conclusion)
        }
    }

    pub fn score_factor_assignment_disjunction(
        &self,
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
        let statistics = self.model.model.predict(&context)?;
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
