use super::{
    graph::PropositionGraph,
    table::{HashMapBeliefTable, InferenceResult, PropositionNode},
};
use crate::{
    common::{interface::PropositionDB, model::InferenceModel},
    inference::table::HashMapInferenceResult,
    model::{
        objects::{Predicate, PredicateGroup, Proposition, PropositionGroup},
        weights::CLASS_LABELS,
    },
};
use redis::Connection;
use std::{borrow::Borrow, collections::HashMap, error::Error, rc::Rc};

struct Inferencer {
    model: Rc<InferenceModel>,
    proposition_graph: Rc<PropositionGraph>,
    pub data: HashMapBeliefTable,
}

fn inference_proposition_probability(
    proposition_db: &dyn PropositionDB,
    proposition: &Proposition,
) -> Result<f64, Box<dyn Error>> {
    Ok(proposition_db
        .get_proposition_probability(proposition)
        .unwrap()
        .unwrap())
}

fn inference_conjoined_probability(
    proposition_db: &dyn PropositionDB,
    group: &PropositionGroup,
) -> Result<f64, Box<dyn Error>> {
    todo!()
}

#[macro_export]
macro_rules! print_red {
    ($($arg:tt)*) => {
        use colored::*;
        info!("{}", format!($($arg)*).red());
    };
}
#[macro_export]
macro_rules! print_green {
    ($($arg:tt)*) => {
        use colored::*;
        info!("{}", format!($($arg)*).green());
    };
}
#[macro_export]
macro_rules! print_yellow {
    ($($arg:tt)*) => {
        use colored::*;
        info!("{}", format!($($arg)*).yellow());
    };
}
#[macro_export]
macro_rules! print_blue {
    ($($arg:tt)*) => {
        use colored::*;
        info!("{}", format!($($arg)*).blue());
    };
}

impl Inferencer {
    // Initialize new Storage with a Redis connection
    pub fn new_mutable(
        model: Rc<InferenceModel>,
        proposition_graph: Rc<PropositionGraph>,
    ) -> Result<Box<Self>, redis::RedisError> {
        Ok(Box::new(Inferencer {
            model,
            proposition_graph,
            data: HashMapBeliefTable::new(),
        }))
    }

    pub fn initialize(&mut self, proposition: &Proposition) -> Result<(), Box<dyn Error>> {
        print_red!("initialize: proposition {:?}", proposition.hash_string());
        self.initialize_pi()?;
        self.initialize_lambda(proposition)?;
        Ok(())
    }

    pub fn initialize_pi(&mut self) -> Result<(), Box<dyn Error>> {
        print_red!("initialize_pi: proposition");
        for node in &self.proposition_graph.all_nodes {
            print_red!("initializing: {}", node.debug_string());
            for outcome in CLASS_LABELS {
                self.data.set_pi_value(node, outcome, 1f64);
            }
            for parent in &self.proposition_graph.get_all_backward(node) {
                print_red!(
                    "initializing pi link from {} to {}",
                    node.debug_string(),
                    parent.debug_string()
                );
                for outcome in CLASS_LABELS {
                    self.data.set_pi_message(node, parent, outcome, 1f64);
                }
            }
        }
        Ok(())
    }

    pub fn initialize_pi_proposition(
        &mut self,
        node: &Proposition,
        is_root: bool,
    ) -> Result<(), Box<dyn Error>> {
        print_yellow!(
            "initialize_pi_proposition: is_root {} node {}",
            is_root,
            node.hash_string()
        );
        let children = self.proposition_graph.get_single_forward(node);
        for child in children {
            print_yellow!("found child {}", child.hash_string());
            self.initialize_pi_conjunct(&child, false)?;
        }
        if is_root {
            let prior_prob =
                inference_proposition_probability(self.model.proposition_db.borrow(), node)?;
            self.data
                .set_pi_value(&PropositionNode::from_proposition(node), 1, prior_prob);
            self.data.set_pi_value(
                &PropositionNode::from_proposition(node),
                0,
                1f64 - prior_prob,
            );
        }
        for outcome in CLASS_LABELS {
            let children = self.proposition_graph.get_single_forward(node);
            for child in children {
                self.data.set_lambda_message(
                    &PropositionNode::from_proposition(node),
                    &PropositionNode::from_group(&child),
                    outcome,
                    1f64,
                );
            }
        }
        Ok(())
    }

    pub fn initialize_pi_conjunct(
        &mut self,
        group: &PropositionGroup,
        is_root: bool,
    ) -> Result<(), Box<dyn Error>> {
        print_green!(
            "initialize_pi_conjunct: starts; is_root {} group {}",
            is_root,
            group.hash_string()
        );
        let children = self.proposition_graph.get_group_forward(group);
        for child in children {
            print_green!("found child: single {}", child.hash_string());
            self.initialize_pi_proposition(&child, false)?;
        }
        for outcome in CLASS_LABELS {
            info!(
                "initialize_pi_conjunct: outcome {} is_root {} group {}",
                outcome,
                is_root,
                group.hash_string()
            );
            let children = self.proposition_graph.get_group_forward(group);
            for child in children {
                self.data.set_lambda_message(
                    &PropositionNode::from_group(group),
                    &PropositionNode::from_proposition(&child),
                    outcome,
                    1f64,
                );
            }
        }
        Ok(())
    }

    pub fn initialize_lambda(&mut self, proposition: &Proposition) -> Result<(), Box<dyn Error>> {
        let roots = self.proposition_graph.get_roots();
        for root in roots {
            self.initialize_lambda_proposition(&root)?;
        }
        Ok(())
    }

    pub fn initialize_lambda_proposition(
        &mut self,
        node: &Proposition,
    ) -> Result<(), Box<dyn Error>> {
        for outcome in CLASS_LABELS {
            self.data
                .set_lambda_value(&PropositionNode::from_proposition(node), outcome, 1f64);
            let parents = self.proposition_graph.get_single_backward(node);
            for parent in parents {
                self.data.set_lambda_message(
                    &PropositionNode::from_proposition(node),
                    &PropositionNode::from_group(&parent),
                    outcome,
                    1f64,
                );
            }
        }
        let children = self.proposition_graph.get_single_forward(node);
        for child in children {
            self.initialize_lambda_conjunct(&child)?;
        }
        Ok(())
    }

    pub fn initialize_lambda_conjunct(
        &mut self,
        group: &PropositionGroup,
    ) -> Result<(), Box<dyn Error>> {
        for outcome in CLASS_LABELS {
            self.data
                .set_lambda_value(&PropositionNode::from_group(group), outcome, 1f64);
            let parents = self.proposition_graph.get_group_backward(group);
            for parent in &parents {
                self.data.set_lambda_message(
                    &PropositionNode::from_group(group),
                    &PropositionNode::from_proposition(parent),
                    outcome,
                    1f64,
                );
            }
        }
        let children = self.proposition_graph.get_group_forward(group);
        for child in children {
            self.initialize_lambda_proposition(&child)?;
        }
        Ok(())
    }
}

// Note: GraphicalModel contains PropositionDB, which contains the "evidence".
pub fn inference_compute_marginals(
    model: Rc<InferenceModel>,
    target: &Proposition,
) -> Result<Rc<dyn InferenceResult>, Box<dyn Error>> {
    let proposition_graph = PropositionGraph::new_shared(model.graph.clone(), target)?;
    let mut inferencer = Inferencer::new_mutable(model.clone(), proposition_graph.clone())?;
    inferencer.initialize(target)?;
    inferencer.data.print_debug();
    HashMapInferenceResult::new_shared(inferencer.data)
}
