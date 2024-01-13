use std::{collections::HashMap, error::Error, rc::Rc};

use serde::{Deserialize, Serialize};

use crate::{
    common::{graph::InferenceGraph, redis::RedisManager},
    model::{
        choose::compute_search_predicates,
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
    pub single_forward: HashMap<Proposition, PropositionGroup>,
    pub single_backward: HashMap<Proposition, PropositionGroup>,
    pub group_forward: HashMap<PropositionGroup, Proposition>,
}

fn initialize_visit_group(
    graph: &mut PropositionGraph,
    single: &Proposition,
) -> Result<(), Box<dyn Error>> {
    todo!()
}

fn initialize_visit_single(
    graph: &mut PropositionGraph,
    single: &Proposition,
) -> Result<(), Box<dyn Error>> {
    todo!()
}

impl PropositionGraph {
    pub fn new_mutable(
        predicate_graph: Rc<InferenceGraph>,
        target:&Proposition,
    ) -> Result<Box<PropositionGraph>, Box<dyn Error>> {
        let mut graph = PropositionGraph {
            predicate_graph,
            single_forward: HashMap::new(),
            single_backward: HashMap::new(),
            group_forward: HashMap::new(),
        };
        initialize_visit_single(&mut graph, target)?;
        Ok(Box::new(graph))
    }
    pub fn find_roots(
        &self,
        proposition: &Proposition,
    ) -> Result<Vec<Proposition>, Box<dyn Error>> {
        todo!()
    }
    pub fn proposition_backward_links(
        &self,
        proposition: &Proposition,
    ) -> Result<Vec<PropositionGroup>, Box<dyn Error>> {
        let search_predicates = compute_search_predicates(proposition)?;
        let mut result = vec![];
        for predicate in &search_predicates {
            let predicate_backward = self.predicate_graph.predicate_backward_links(predicate)?;
            for inference_link in &predicate_backward {
                let proposition_implication =
                    proposition_implication_from(inference_link, proposition)?;
                result.push(proposition_implication.premise);
            }
        }
        Ok(result)
    }
    pub fn proposition_forward_links(
        &self,
        proposition: &Proposition,
    ) -> Result<Vec<PropositionGroup>, Box<dyn Error>> {
        todo!()
        // let search_predicates = compute_search_predicates(proposition)?;
        // let mut result = vec![];
        // for predicate in &search_predicates {
        //     let predicate_backward = self.predicate_graph.predicate_forward_links(predicate)?;
        //     for inference_link in &predicate_backward {
        //         result.push(inference_link);
        //     }
        // }
        // Ok(result)
    }
    pub fn conjoined_backward_links(
        &self,
        conjoined: &PropositionGroup,
    ) -> Result<Vec<Proposition>, Box<dyn Error>> {
        Ok(conjoined.terms.clone())
    }
    pub fn conjoined_forward_links(
        &self,
        conjoined: &PropositionGroup,
    ) -> Result<Vec<Proposition>, Box<dyn Error>> {
        todo!()
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
