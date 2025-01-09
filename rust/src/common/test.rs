use std::{collections::HashMap, error::Error, io, rc::Rc};

use colored::Colorize;

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
    model::{
        exponential::ExponentialModel,
        objects::{Proposition, unary_existence_function},
    },
    print_blue, print_green, print_red, print_yellow,
};

use super::{interface::BeliefTable, resources::FactoryResources, setup::CommandLineOptions};

pub struct ReplState {
    pub inferencer: Box<Inferencer>,
    pub fact_memory: Rc<HashMapBeliefTable>,
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
    fn do_repl_loop(&mut self) -> Result<(), Box<dyn Error>> {
        loop {
            self.print_menu_options()?;
            let tokens = get_input_tokens_from_user();
            if tokens.len() < 1 {
                continue;
            }
            let function = &tokens[0];
            match function.as_str() {
                "s" => {
                    self.handle_set(&tokens);
                }
                "u" => {
                    self.handle_unset(&tokens);
                }
                "reinit" => {
                    self.inferencer.initialize_chart()?;
                }
                "print" => {
                    self.print_table(&tokens);
                }
                "pv" => {
                    self.inferencer.data.print_table(&function);
                }
                "lv" => {
                    self.inferencer.data.print_table(&function);
                }
                "pm" => {
                    self.inferencer.data.print_table(&function);
                }
                "lm" => {
                    self.inferencer.data.print_table(&function);
                }
                "m" => {
                    self.inferencer.update_marginals()?;
                }
                "p" => {
                    self.inferencer.do_full_forward_and_backward()?;
                    self.inferencer.update_marginals()?;
                }
                "q" => break,
                _ => println!("Command not recognized."),
            };
        }
        Ok(())
    }

    pub fn set_pairs_by_name(&mut self, pairs:&Vec<(&str, f64)>) -> Option<PropositionNode> {
        assert!(pairs.len() <= 1);
        for pair in pairs {
            let key = pair.0.to_string();
            let node = self.proposition_index.get(&key).unwrap();
            let prop = node.extract_single();
            info!("setting {} to {}", &key, pair.1);
            self.fact_memory
                .store_proposition_probability(&prop, pair.1)
                .unwrap();
            self.inferencer.do_fan_out_from_node(&node).unwrap();
            return Some(node.clone());
        }
        None
    }

    fn handle_set(&mut self, tokens: &Vec<String>) {
        let select_index = tokens[1].parse::<u64>().unwrap();
        let new_prob = tokens[2].parse::<f64>().unwrap();
        let node = self.question_index.get(&select_index).unwrap();
        let prop = node.extract_single();
        self.fact_memory
            .store_proposition_probability(&prop, new_prob)
            .unwrap();
        self.inferencer.do_fan_out_from_node(&node).unwrap();
        self.inferencer.update_marginals().unwrap();
    }

    fn handle_unset(&mut self, tokens: &Vec<String>) {
        let select_index = tokens[1].parse::<u64>().unwrap();
        let node = self.question_index.get(&select_index).unwrap();
        self.fact_memory.clear(node);
        self.inferencer.do_fan_out_from_node(&node).unwrap();
    }

    fn print_table(&mut self, tokens: &Vec<String>) {
        let table_name = tokens[1].clone();
        self.inferencer.data.print_table(&table_name);
    }

    fn print_menu_options(&mut self) -> Result<(), Box<dyn Error>> {
        let bfs = self.inferencer.proposition_graph.get_bfs_order();
        self.question_index.clear();
        println!("NODES");
        for (index, node) in bfs.iter().enumerate() {
            if node.is_single() {
                let single = node.extract_single();
                let probability = self.fact_memory.get_proposition_probability(&single)?;
                let probability_string = match &probability {
                    Some(value) => {
                        if *value > 0.5f64 {
                            "Yes".green()
                        } else {
                            "No".green()
                        }
                    }
                    None => "None".yellow(),
                };
                println!("{}\t{}\t{:?}", index, &probability_string, &node);
                self.question_index.insert(index as u64, node.clone());
            } else {
                // trace!("node {} {:?} *", index, &node);
            }
        }
        Ok(())
    }

}

pub fn get_input_tokens_from_user() -> Vec<String> {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let trimmed = input.trim();
    let tokens: Vec<String> = trimmed.split_whitespace().map(|s| s.to_string()).collect();
    tokens
}

pub fn interactive_inference_example(
    config: &CommandLineOptions,
    resources: &FactoryResources,
) -> Result<(), Box<dyn Error>> {
    let plan = TrainingPlan::new(&resources)?;
    let graphical_model = InferenceModel::new_shared(&resources)?;
    let plan = TrainingPlan::new(&resources)?;
    let model = InferenceModel::new_shared(&resources).unwrap();
    let test_questions = plan.get_test_questions().unwrap();
    let target = &test_questions[config.test_example.unwrap() as usize];
    let fact_memory = EmptyBeliefTable::new_shared(&resources.redis)?;
    let proposition_graph = PropositionGraph::new_shared(&model.graph)?;
    proposition_graph.visualize();
    let mut inferencer =
        Inferencer::new_mutable(&config, model.clone(), proposition_graph.clone(), fact_memory)?;
    inferencer.initialize_chart()?;
    let mut repl = ReplState::new(inferencer);
    repl.do_repl_loop()?;
    Ok(())
}

pub fn summarize_examples(
    config: &CommandLineOptions,
    resources: &FactoryResources,
) -> Result<(), Box<dyn Error>> {
    let plan = TrainingPlan::new(&resources)?;
    let graphical_model = InferenceModel::new_shared(&resources)?;
    let model = InferenceModel::new_shared(&resources).unwrap();
    // test
    let test_questions = plan.get_test_questions().unwrap();
    for (index, proposition) in test_questions.iter().enumerate() {
        println!("testing proposition {:?}", &proposition.hash_string());
    }
    Ok(())
}

fn make_proposition_map(graph:&PropositionGraph) -> HashMap<String, PropositionNode> {
    let bfs = graph.get_bfs_order();
    let mut result = HashMap::new();
    for (index, node) in bfs.iter().enumerate() {
        let name = node.debug_string();
        result.insert(name, node.clone());
    }
    result
}