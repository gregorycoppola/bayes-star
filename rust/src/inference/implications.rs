use std::error::Error;

use crate::{common::{graph::Graph, redis::RedisManager}, model::objects::{Proposition, PropositionConjunction}};

pub struct PropositionGraph {
    predicate_graph: Graph,
}

impl PropositionGraph {
    fn new(redis: &RedisManager) -> Result<PropositionGraph, Box<dyn Error>> {
        todo!()
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
