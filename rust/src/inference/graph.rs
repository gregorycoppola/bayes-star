use std::{error::Error, rc::Rc};

use serde::{Serialize, Deserialize};

use crate::{
    common::{graph::InferenceGraph, redis::RedisManager, model::Factor},
    model::{objects::{Proposition, PropositionGroup, PredicateInferenceFactor, GroupRoleMap}, choose::compute_search_predicates},
};

fn proposition_implication_from(implication:&PredicateInferenceFactor, proposition: &Proposition) -> Result<Factor, Box<dyn Error>> {
    todo!()
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PropositionInferenceFactor {
    pub premise: PropositionGroup,
    pub role_maps: GroupRoleMap,
    pub conclusion: Proposition,
}

pub struct PropositionGraph {
    predicate_graph: Rc<InferenceGraph>,
}

impl PropositionGraph {
    pub fn new_mutable(
        predicate_graph: Rc<InferenceGraph>,
    ) -> Result<Box<PropositionGraph>, Box<dyn Error>> {
        Ok(Box::new(PropositionGraph { predicate_graph }))
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
        todo!()
        // let search_predicates = compute_search_predicates(proposition)?;
        // let mut result = vec![];
        // for predicate in &search_predicates {
        //     let predicate_backward = self.predicate_graph.predicate_backward_links(predicate)?;
        //     for inference_link in &predicate_backward {
        //         let proposition_implication = proposition_implication_from(inference_link, proposition)?;
        //         result.push(proposition_implication.conjuncts);
        //     }
        // }
        // Ok(result)
    }
    pub fn proposition_forward_links(
        &self,
        proposition: &Proposition,
    ) -> Result<Vec<PropositionGroup>, Box<dyn Error>> {
        todo!()
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
