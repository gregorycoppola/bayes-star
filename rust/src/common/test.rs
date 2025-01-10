use std::{collections::HashMap, error::Error, io, rc::Rc, sync::Arc};

use colored::Colorize;
use redis::Connection;

use crate::{
    common::{
        graph::InferenceGraph,
        model::InferenceModel,
        proposition_db::{EmptyBeliefTable, HashMapBeliefTable, RedisBeliefTable},
        train::TrainingPlan,
    },
    inference::{
        graph::PropositionGraph,
        inference::Inferencer,
        table::{self, PropositionNode},
    },
    model::{exponential::ExponentialModel, objects::Proposition},
    print_blue, print_green, print_red, print_yellow,
};

use super::{interface::BeliefTable, resources::ResourceContext, setup::CommandLineOptions};

pub struct ReplState {
    pub inferencer: Box<Inferencer>,
    pub fact_memory: Arc<HashMapBeliefTable>,
    /// Relative set by the `print_ordering` last time it serialized an ordering.
    pub question_index: HashMap<u64, PropositionNode>,
    pub proposition_index: HashMap<String, PropositionNode>,
}

impl ReplState {
    pub fn new(mut inferencer: Box<Inferencer>) -> ReplState {
        let fact_memory = HashMapBeliefTable::new();
        inferencer.fact_memory = fact_memory.clone();
        let proposition_index = make_proposition_map(&inferencer.proposition_graph);
        ReplState {
            inferencer,
            fact_memory,
            question_index: HashMap::new(),
            proposition_index,
        }
    }

    pub fn set_pairs_by_name(
        &mut self,
        connection: &mut Connection,
        pairs: &Vec<(&str, f64)>,
    ) -> Option<PropositionNode> {
        assert!(pairs.len() <= 1);
        for pair in pairs {
            let key = pair.0.to_string();
            trace!("key {key}");
            let node = self.proposition_index.get(&key).unwrap();
            let prop = node.extract_single();
            trace!("setting {} to {}", &key, pair.1);
            self.fact_memory
                .store_proposition_probability(connection, &prop, pair.1)
                .unwrap();
            self.inferencer
                .do_fan_out_from_node(connection, &node)
                .unwrap();
            return Some(node.clone());
        }
        None
    }
}

fn make_proposition_map(graph: &PropositionGraph) -> HashMap<String, PropositionNode> {
    let bfs = graph.get_bfs_order();
    let mut result = HashMap::new();
    for (index, node) in bfs.iter().enumerate() {
        let name = node.debug_string();
        warn!("name_key: {}", &name);
        result.insert(name, node.clone());
    }
    result
}
