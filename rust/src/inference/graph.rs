use std::{error::Error, rc::Rc};

use crate::{
    common::{graph::InferenceGraph, redis::RedisManager},
    model::objects::{Proposition, PropositionConjunction},
};

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
        x: &Proposition,
    ) -> Result<Vec<PropositionConjunction>, Box<dyn Error>> {
        todo!()
    }
    pub fn proposition_forward_links(
        &self,
        root: &Proposition,
    ) -> Result<Vec<PropositionConjunction>, Box<dyn Error>> {
        todo!()
    }
    pub fn conjoined_backward_links(
        &self,
        conjunction: &PropositionConjunction,
    ) -> Result<Vec<Proposition>, Box<dyn Error>> {
        Ok(conjunction.terms.clone())
    }
    pub fn conjoined_forward_links(
        &self,
        conjunction: &PropositionConjunction,
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
