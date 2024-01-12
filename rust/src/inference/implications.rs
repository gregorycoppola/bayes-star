use std::error::Error;

use crate::common::{graph::Graph, redis::RedisManager};

struct PropositionGraph {
    predicate_graph: Graph,
}

impl PropositionGraph {
    fn new(redis: &RedisManager) -> Result<PropositionGraph, Box<dyn Error>> {
        todo!()
    }
}
