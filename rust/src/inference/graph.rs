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

fn proposition_implication_from(
    implication: &PredicateInferenceFactor,
    proposition: &Proposition,
) -> Result<PropositionInferenceFactor, Box<dyn Error>> {
    todo!()
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PropositionInferenceFactor {
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
    println!("Initializing visit for proposition: {:?}", single);
    
    let inference_factors =
        extract_backimplications_from_proposition(&graph.predicate_graph, single)?;
    println!("Inference factors count: {}", inference_factors.len());

    if inference_factors.is_empty() {
        println!("No inference factors. Adding to roots.");
        graph.roots.insert(single.clone());
    } else {
        for inference_factor in &inference_factors {
            println!("Processing inference factor: {:?}", inference_factor);

            println!("Updating single_backward for conclusion: {:?}", inference_factor.conclusion);
            graph
                .single_backward
                .entry(inference_factor.conclusion.clone())
                .or_insert_with(Vec::new)
                .push(inference_factor.premise.clone());

            println!("Updating group_forward for premise: {:?}", inference_factor.premise);
            graph
                .group_forward
                .entry(inference_factor.premise.clone())
                .or_insert_with(Vec::new)
                .push(inference_factor.conclusion.clone());

            for term in &inference_factor.premise.terms {
                println!("Processing term: {:?}", term);
                graph
                    .single_forward
                    .entry(term.clone())
                    .or_insert_with(Vec::new)
                    .push(inference_factor.premise.clone());
                println!("Recursively initializing visit for term: {:?}", term);
                initialize_visit_single(graph, term)?;
            }
        }
    }

    println!("Finished initializing visit for proposition: {:?}", single);
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

pub fn compute_forward_graph(
    predicate_graph: Rc<InferenceGraph>,
    proposition: &Proposition,
) -> Result<PropositionGraph, Box<dyn Error>> {
    todo!()
}

pub fn compute_backward_graph(
    predicate_graph: Rc<InferenceGraph>,
    proposition: &Proposition,
) -> Result<PropositionGraph, Box<dyn Error>> {
    todo!()
}
