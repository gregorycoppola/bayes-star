use std::{
    collections::{HashMap, HashSet, VecDeque},
    error::Error,
    rc::Rc,
};

use env_logger::init;
use serde::{Deserialize, Serialize};

use crate::{
    common::{graph::InferenceGraph, redis::RedisManager},
    model::{
        choose::{compute_search_predicates, extract_backimplications_from_proposition},
        objects::{GroupRoleMap, ImplicationFactor, Proposition, PropositionGroup},
    }, print_yellow,
};

use super::table::{GenericNodeType, PropositionNode};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PropositionFactor {
    pub premise: PropositionGroup,
    pub conclusion: Proposition,
    pub inference: ImplicationFactor,
}

impl PropositionFactor {
    pub fn debug_string(&self) -> String {
        format!(
            "{} -> {}",
            self.premise.hash_string(),
            self.conclusion.hash_string()
        )
    }
}

pub struct PropositionGraph {
    pub predicate_graph: Rc<InferenceGraph>,
    pub single_forward: HashMap<Proposition, HashSet<PropositionGroup>>,
    pub single_backward: HashMap<Proposition, HashSet<PropositionGroup>>,
    pub group_forward: HashMap<PropositionGroup, HashSet<Proposition>>,
    pub inference_used: HashMap<(PropositionGroup, Proposition), ImplicationFactor>,
    pub roots: HashSet<Proposition>,
    pub all_nodes: HashSet<PropositionNode>,
    pub target: Proposition,
}

fn initialize_visit_single(
    graph: &mut PropositionGraph,
    single: &Proposition,
) -> Result<(), Box<dyn Error>> {
    trace!(
        "\x1b[32mInitializing visit for proposition: {:?}\x1b[0m",
        single.hash_string()
    );
    graph
        .all_nodes
        .insert(PropositionNode::from_single(single));
    let inference_factors =
        extract_backimplications_from_proposition(&graph.predicate_graph, single)?;
    trace!(
        "\x1b[33mInference factors count: {}\x1b[0m",
        inference_factors.len()
    );

    if inference_factors.is_empty() {
        trace!("\x1b[34mNo inference factors. Adding to roots.\x1b[0m");
        graph.roots.insert(single.clone());
    } else {
        for inference_factor in &inference_factors {
            trace!(
                "\x1b[36mProcessing inference factor: {:?}\x1b[0m",
                inference_factor.debug_string()
            );
            let inference_used_key = (inference_factor.premise.clone(), inference_factor.conclusion.clone());
            graph.inference_used.insert(inference_used_key, inference_factor.inference.clone());

            trace!(
                "\x1b[36mUpdating single_backward for conclusion: {:?}\x1b[0m",
                inference_factor.conclusion.hash_string()
            );
            graph
                .single_backward
                .entry(inference_factor.conclusion.clone())
                .or_insert_with(HashSet::new)
                .insert(inference_factor.premise.clone());

            trace!(
                "\x1b[36mUpdating group_forward for premise: {:?}\x1b[0m",
                inference_factor.premise.hash_string()
            );
            graph
                .group_forward
                .entry(inference_factor.premise.clone())
                .or_insert_with(HashSet::new)
                .insert(inference_factor.conclusion.clone());

            graph
                .all_nodes
                .insert(PropositionNode::from_group(&inference_factor.premise));

            for term in &inference_factor.premise.terms {
                trace!("\x1b[35mProcessing term: {:?}\x1b[0m", term.hash_string());
                graph
                    .single_forward
                    .entry(term.clone())
                    .or_insert_with(HashSet::new)
                    .insert(inference_factor.premise.clone());
                trace!(
                    "\x1b[35mRecursively initializing visit for term: {:?}\x1b[0m",
                    term.hash_string()
                );
                initialize_visit_single(graph, term)?;
            }
        }
    }
    trace!(
        "\x1b[32mFinished initializing visit for proposition: {:?}\x1b[0m",
        single.hash_string()
    );
    Ok(())
}

impl PropositionGraph {
    pub fn new_shared(
        predicate_graph: Rc<InferenceGraph>,
        target: &Proposition,
    ) -> Result<Rc<PropositionGraph>, Box<dyn Error>> {
        let mut graph = PropositionGraph {
            predicate_graph,
            single_forward: HashMap::new(),
            single_backward: HashMap::new(),
            group_forward: HashMap::new(),
            inference_used: HashMap::new(),
            roots: HashSet::new(),
            all_nodes: HashSet::new(),
            target: target.clone(),
        };
        initialize_visit_single(&mut graph, target)?;
        Ok(Rc::new(graph))
    }

    pub fn get_inference_used(&self, premise:&PropositionGroup, conclusion: &Proposition) -> ImplicationFactor {
        let key = (premise.clone(), conclusion.clone());
        self.inference_used
            .get(&key).unwrap().clone()
    }

    pub fn get_single_forward(&self, key: &Proposition) -> HashSet<PropositionGroup> {
        self.single_forward
            .get(key)
            .cloned()
            .unwrap_or_else(HashSet::new)
    }

    pub fn get_single_backward(&self, key: &Proposition) -> HashSet<PropositionGroup> {
        self.single_backward
            .get(key)
            .cloned()
            .unwrap_or_else(HashSet::new)
    }

    pub fn get_group_forward(&self, key: &PropositionGroup) -> HashSet<Proposition> {
        self.group_forward.get(key).unwrap().clone()
    }

    pub fn get_group_backward(&self, key: &PropositionGroup) -> Vec<Proposition> {
        key.terms.clone()
    }

    pub fn get_all_backward(&self, node: &PropositionNode) -> Vec<PropositionNode> {
        trace!("get_all_backward called for node: {:?}", node.debug_string());
        let mut r = vec![];
        match &node.node {
            GenericNodeType::Single(proposition) => {
                trace!("Processing as Single: {:?}", proposition.debug_string());
                let initial = self.get_single_backward(proposition);
                trace!("Initial singles: {}", initial.len());
                for group in &initial {
                    trace!("Adding group from initial singles: {:?}", group.debug_string());
                    r.push(PropositionNode::from_group(group));
                }
            }
            GenericNodeType::Group(group) => {
                trace!("Processing as Group: {:?}", group.debug_string());
                let initial = self.get_group_backward(group);
                trace!("Initial groups: {}", initial.len());
                for single in &initial {
                    trace!("Adding single from initial groups: {:?}", single.debug_string());
                    r.push(PropositionNode::from_single(single));
                }
            }
        }
        trace!("Resulting vector: {:?}", r);
        r
    }

    pub fn get_all_forward(&self, node: &PropositionNode) -> Vec<PropositionNode> {
        trace!("get_all_backward called for node: {:?}", node.debug_string());
        let mut r = vec![];
        match &node.node {
            GenericNodeType::Single(proposition) => {
                trace!("Processing as Single: {:?}", proposition.debug_string());
                let initial = self.get_single_forward(proposition);
                trace!("Initial singles: {}", initial.len());
                for group in &initial {
                    trace!("Adding group from initial singles: {:?}", group.debug_string());
                    r.push(PropositionNode::from_group(group));
                }
            }
            GenericNodeType::Group(group) => {
                trace!("Processing as Group: {:?}", group.debug_string());
                let initial = self.get_group_forward(group);
                trace!("Initial groups: {}", initial.len());
                for single in &initial {
                    trace!("Adding single from initial groups: {:?}", single.debug_string());
                    r.push(PropositionNode::from_single(single));
                }
            }
        }
        trace!("Resulting vector: {:?}", r);
        r
    }

    pub fn get_roots(&self) -> HashSet<Proposition> {
        self.roots.clone()
    }

    pub fn get_bfs_order(&self) -> Vec<PropositionNode> {
        create_bfs_order(&self)
    }
}

impl PropositionGraph {
    pub fn visualize(&self) {
        trace!("Single Forward:");
        for (key, value) in self.single_forward.iter() {
            trace!("  {:?}: {:?}", key, value);
        }

        trace!("Single Backward:");
        for (key, value) in self.single_backward.iter() {
            trace!("  {:?}: {:?}", key, value);
        }

        trace!("Group Forward:");
        for (key, value) in self.group_forward.iter() {
            trace!("  {:?}: {:?}", key, value);
        }

        trace!("Inference Used:");
        for (key, value) in self.inference_used.iter() {
            trace!("  ({:?}, {:?}): {:?}", key.0, key.1, value);
        }

        trace!("Roots: {:?}", self.roots);
        trace!("All Nodes: {:?}", self.all_nodes);
    }
}

fn reverse_prune_duplicates(raw_order: &Vec<(i32, PropositionNode)>) -> Vec<PropositionNode> {
    let mut seen = HashSet::new();
    let mut result = vec![];
    for (depth, node) in raw_order.iter().rev() {
        if !seen.contains(node) {
            result.push(node.clone());
        }
        seen.insert(node);
    }
    result.reverse();
    result
}

fn create_bfs_order(proposition_graph: &PropositionGraph) -> Vec<PropositionNode> {
    let mut queue = VecDeque::new();
    let mut buffer = vec![];
    for root in &proposition_graph.roots {
        queue.push_back((0, PropositionNode::from_single(&root)));
    }
    while let Some((depth, node)) = queue.pop_front() {
        buffer.push((depth, node.clone()));
        let forward = proposition_graph.get_all_forward(&node);
        for child in &forward {
            queue.push_back((depth + 1, child.clone()));
        }
    }
    let result = reverse_prune_duplicates(&buffer);
    result
}
