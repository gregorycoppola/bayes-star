use std::{
    collections::{HashMap, HashSet},
    error::Error,
    rc::Rc,
};

use serde::{Deserialize, Serialize};

use crate::{
    common::{graph::InferenceGraph, redis::RedisManager},
    model::{
        choose::{compute_search_predicates, extract_backimplications_from_proposition},
        objects::{GroupRoleMap, PredicateInferenceFactor, Proposition, PropositionGroup},
    },
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PropositionFactor {
    pub premise: PropositionGroup,
    pub conclusion: Proposition,
    pub inference: PredicateInferenceFactor,
}

pub struct PropositionGraph {
    pub predicate_graph: Rc<InferenceGraph>,
    pub single_forward: HashMap<Proposition, Vec<PropositionGroup>>,
    pub single_backward: HashMap<Proposition, Vec<PropositionGroup>>,
    pub group_forward: HashMap<PropositionGroup, Vec<Proposition>>,
    pub roots: HashSet<Proposition>,
}

fn initialize_visit_single(
    graph: &mut PropositionGraph,
    single: &Proposition,
) -> Result<(), Box<dyn Error>> {
    // Green for starting a new operation
    println!("\x1b[32mInitializing visit for proposition: {:?}\x1b[0m", single.hash_string());
    
    let inference_factors =
        extract_backimplications_from_proposition(&graph.predicate_graph, single)?;
    // Yellow for showing counts or lengths
    println!("\x1b[33mInference factors count: {}\x1b[0m", inference_factors.len());

    if inference_factors.is_empty() {
        // Blue for specific condition-related messages
        println!("\x1b[34mNo inference factors. Adding to roots.\x1b[0m");
        graph.roots.insert(single.clone());
    } else {
        for inference_factor in &inference_factors {
            // Cyan for loop iteration
            println!("\x1b[36mProcessing inference factor: {:?}\x1b[0m", inference_factor);

            println!("\x1b[36mUpdating single_backward for conclusion: {:?}\x1b[0m", inference_factor.conclusion.hash_string());
            graph
                .single_backward
                .entry(inference_factor.conclusion.clone())
                .or_insert_with(Vec::new)
                .push(inference_factor.premise.clone());

            println!("\x1b[36mUpdating group_forward for premise: {:?}\x1b[0m", inference_factor.premise.hash_string());
            graph
                .group_forward
                .entry(inference_factor.premise.clone())
                .or_insert_with(Vec::new)
                .push(inference_factor.conclusion.clone());

            for term in &inference_factor.premise.terms {
                println!("\x1b[35mProcessing term: {:?}\x1b[0m", term);
                graph
                    .single_forward
                    .entry(term.clone())
                    .or_insert_with(Vec::new)
                    .push(inference_factor.premise.clone());
                println!("\x1b[35mRecursively initializing visit for term: {:?}\x1b[0m", term);
                initialize_visit_single(graph, term)?;
            }
        }
    }

    // Green for completion messages
    println!("\x1b[32mFinished initializing visit for proposition: {:?}\x1b[0m", single);
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
            roots: HashSet::new(),
        };
        initialize_visit_single(&mut graph, target)?;
        Ok(Rc::new(graph))
    }

    pub fn get_single_forward(&self, key: &Proposition) -> Vec<PropositionGroup> {
        self.single_forward.get(key).cloned().unwrap_or_else(Vec::new)
    }

    pub fn get_single_backward(&self, key: &Proposition) -> Vec<PropositionGroup> {
        self.single_backward.get(key).cloned().unwrap_or_else(Vec::new)
    }

    pub fn get_group_forward(&self, key: &PropositionGroup) -> Vec<Proposition> {
        self.group_forward.get(key).unwrap().clone()
    }

    pub fn get_group_backward(&self, key: &PropositionGroup) -> Vec<Proposition> {
        key.terms.clone()
    }

    pub fn get_roots(&self) -> HashSet<Proposition> {
        self.roots.clone()
    }
}