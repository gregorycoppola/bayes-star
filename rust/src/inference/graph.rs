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
        objects::{GroupRoleMap, PredicateFactor, Proposition, PropositionGroup},
    }, print_green, print_yellow,
};

use super::table::{GenericNodeType, PropositionNode};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PropositionFactor {
    pub premise: PropositionGroup,
    pub conclusion: Proposition,
    pub inference: PredicateFactor,
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
    pub inference_used: HashMap<(PropositionGroup, Proposition), PredicateFactor>,
    pub roots: HashSet<Proposition>,
    pub all_nodes: HashSet<PropositionNode>,
}

fn initialize_visit_single(
    graph: &mut PropositionGraph,
    single: &Proposition,
) -> Result<(), Box<dyn Error>> {
    // Green for starting a new operation
    info!(
        "\x1b[32mInitializing visit for proposition: {:?}\x1b[0m",
        single.hash_string()
    );
    graph
        .all_nodes
        .insert(PropositionNode::from_single(single));
    let inference_factors =
        extract_backimplications_from_proposition(&graph.predicate_graph, single)?;
    // Yellow for showing counts or lengths
    info!(
        "\x1b[33mInference factors count: {}\x1b[0m",
        inference_factors.len()
    );

    if inference_factors.is_empty() {
        // Blue for specific condition-related messages
        info!("\x1b[34mNo inference factors. Adding to roots.\x1b[0m");
        graph.roots.insert(single.clone());
    } else {
        for inference_factor in &inference_factors {
            // Cyan for loop iteration
            info!(
                "\x1b[36mProcessing inference factor: {:?}\x1b[0m",
                inference_factor.debug_string()
            );

            let inference_used_key = (inference_factor.premise.clone(), inference_factor.conclusion.clone());
            graph.inference_used.insert(inference_used_key, inference_factor.inference.clone());

            info!(
                "\x1b[36mUpdating single_backward for conclusion: {:?}\x1b[0m",
                inference_factor.conclusion.hash_string()
            );
            graph
                .single_backward
                .entry(inference_factor.conclusion.clone())
                .or_insert_with(HashSet::new)
                .insert(inference_factor.premise.clone());

            info!(
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
                info!("\x1b[35mProcessing term: {:?}\x1b[0m", term.hash_string());
                graph
                    .single_forward
                    .entry(term.clone())
                    .or_insert_with(HashSet::new)
                    .insert(inference_factor.premise.clone());
                info!(
                    "\x1b[35mRecursively initializing visit for term: {:?}\x1b[0m",
                    term.hash_string()
                );
                initialize_visit_single(graph, term)?;
            }
        }
    }

    // Green for completion messages
    info!(
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
        };
        initialize_visit_single(&mut graph, target)?;
        Ok(Rc::new(graph))
    }

    pub fn get_inference_used(&self, premise:&PropositionGroup, conclusion: &Proposition) -> PredicateFactor {
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
        print_green!("get_all_backward called for node: {:?}", node.debug_string());
        let mut r = vec![];
        match &node.node {
            GenericNodeType::Single(proposition) => {
                print_green!("Processing as Single: {:?}", proposition.debug_string());
                let initial = self.get_single_backward(proposition);
                print_green!("Initial singles: {}", initial.len());
                for group in &initial {
                    print_green!("Adding group from initial singles: {:?}", group.debug_string());
                    r.push(PropositionNode::from_group(group));
                }
            }
            GenericNodeType::Group(group) => {
                print_green!("Processing as Group: {:?}", group.debug_string());
                let initial = self.get_group_backward(group);
                print_green!("Initial groups: {}", initial.len());
                for single in &initial {
                    print_green!("Adding single from initial groups: {:?}", single.debug_string());
                    r.push(PropositionNode::from_single(single));
                }
            }
        }
        info!("Resulting vector: {:?}", r);
        r
    }

    pub fn get_all_forward(&self, node: &PropositionNode) -> Vec<PropositionNode> {
        print_green!("get_all_backward called for node: {:?}", node.debug_string());
        let mut r = vec![];
        match &node.node {
            GenericNodeType::Single(proposition) => {
                print_green!("Processing as Single: {:?}", proposition.debug_string());
                let initial = self.get_single_forward(proposition);
                print_green!("Initial singles: {}", initial.len());
                for group in &initial {
                    print_green!("Adding group from initial singles: {:?}", group.debug_string());
                    r.push(PropositionNode::from_group(group));
                }
            }
            GenericNodeType::Group(group) => {
                print_green!("Processing as Group: {:?}", group.debug_string());
                let initial = self.get_group_forward(group);
                print_green!("Initial groups: {}", initial.len());
                for single in &initial {
                    print_green!("Adding single from initial groups: {:?}", single.debug_string());
                    r.push(PropositionNode::from_single(single));
                }
            }
        }
        info!("Resulting vector: {:?}", r);
        r
    }

    pub fn get_roots(&self) -> HashSet<Proposition> {
        self.roots.clone()
    }

    pub fn get_bfs_order(&self) -> Vec<PropositionNode> {
        todo!()
    }
}

impl PropositionGraph {
    pub fn visualize(&self) {
        info!("Single Forward:");
        for (key, value) in self.single_forward.iter() {
            info!("  {:?}: {:?}", key, value);
        }

        info!("Single Backward:");
        for (key, value) in self.single_backward.iter() {
            info!("  {:?}: {:?}", key, value);
        }

        info!("Group Forward:");
        for (key, value) in self.group_forward.iter() {
            info!("  {:?}: {:?}", key, value);
        }

        info!("Inference Used:");
        for (key, value) in self.inference_used.iter() {
            info!("  ({:?}, {:?}): {:?}", key.0, key.1, value);
        }

        info!("Roots: {:?}", self.roots);
        info!("All Nodes: {:?}", self.all_nodes);
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

    print_yellow!("create_bfs_order initial: queue {:?}", &queue);

    while let Some((depth, node)) = queue.pop_front() {
        buffer.push((depth, node.clone()));
        let forward = proposition_graph.get_all_forward(&node);
        for child in &forward {
            queue.push_back((depth + 1, child.clone()));
        }

        print_yellow!("create_bfs_order initial: queue {:?}", &queue);
        print_yellow!("create_bfs_order initial: buffer {:?}", &buffer);
    }

    let result = reverse_prune_duplicates(&buffer);
    print_yellow!("create_bfs_order result: {:?}", &result);
    result
}
