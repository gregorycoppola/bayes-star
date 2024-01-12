use std::{error::Error, rc::Rc};

use crate::{
    common::{graph::Graph, redis::RedisManager},
    model::objects::{Proposition, PropositionConjunction},
};

pub struct PropositionGraph {
    predicate_graph: Rc<Graph>,
}

impl PropositionGraph {
    fn new(predicate_graph: Rc<Graph>) -> Result<PropositionGraph, Box<dyn Error>> {
        Ok(PropositionGraph { predicate_graph })
    }
    pub fn find_roots(
        &self,
        proposition: &Proposition,
    ) -> Result<Vec<Proposition>, Box<dyn Error>> {
        todo!()
    }
    pub fn parents_of_proposition(
        &self,
        x: &Proposition,
    ) -> Result<Vec<PropositionConjunction>, Box<dyn Error>> {
        todo!()
    }
    pub fn children_of_proposition(
        &self,
        root: &Proposition,
    ) -> Result<Vec<PropositionConjunction>, Box<dyn Error>> {
        todo!()
    }
    pub fn parents_of_conjunct(
        &self,
        conjunction: &PropositionConjunction,
    ) -> Result<Vec<Proposition>, Box<dyn Error>> {
        Ok(conjunction.terms.clone())
    }
    pub fn children_of_conjunct(
        &self,
        conjunction: &PropositionConjunction,
    ) -> Result<Vec<Proposition>, Box<dyn Error>> {
        todo!()
    }
}

pub fn compute_forward_graph(
    predicate_graph: Rc<Graph>,
    proposition: &Proposition,
) -> Result<PropositionGraph, Box<dyn Error>> {
    todo!()
}

pub fn compute_backward_graph(
    predicate_graph: Rc<Graph>,
    proposition: &Proposition,
) -> Result<PropositionGraph, Box<dyn Error>> {
    todo!()
}