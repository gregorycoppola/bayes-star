use std::error::Error;
use bayes_star::common::model::InferenceModel;
use bayes_star::common::proposition_db::EmptyBeliefTable;
use bayes_star::common::setup::{parse_configuration_options, CommandLineOptions};
use bayes_star::common::resources::NamespaceBundle;
use bayes_star::common::test::ReplState;
use bayes_star::common::train::TrainingPlan;
use bayes_star::inference::graph::PropositionGraph;
use bayes_star::inference::inference::Inferencer;
use bayes_star::inference::table::PropositionNode;

extern crate log;

fn setup_test_scenario(scenario_name:&str, test_scenario:&str, repl_state:&mut ReplState) -> Result<Option<PropositionNode>, Box<dyn Error>> {
    let pairs = match (scenario_name, test_scenario) {
        ("dating_simple", "prior") => vec![],
        ("dating_simple", "jack_lonely") => vec![("lonely[sub=test_Man0]", 1f64)],
        ("dating_simple", "they_date") => vec![("date[obj=test_Woman0,sub=test_Man0]", 1f64)],
        ("dating_simple", "jack_likes") => vec![("like[obj=test_Woman0,sub=test_Man0]", 1f64)],
        ("dating_simple", "jill_likes") => vec![("like[obj=test_Man0,sub=test_Woman0]", 1f64)],
        ("dating_triangle", "prior") => vec![("charming[sub=test_Man0]", 1f64)],
        ("dating_triangle", "charming") => vec![("charming[sub=test_Man0]", 1f64)],
        ("dating_triangle", "baller") => vec![("baller[sub=test_Man0]", 1f64)],
        ("long_chain", "prior") => vec![],
        ("long_chain", "set_0_1") => vec![("alpha0[sub=test_Man0]", 1f64)],
        ("long_chain", "set_n_1") => vec![("alpha10[sub=test_Man0]", 1f64)],
        ("mid_chain", "set_0_1") => vec![("alpha0[sub=test_Man0]", 1f64)],
        ("mid_chain", "set_n_1") => vec![("alpha4[sub=test_Man0]", 1f64)],
        _ => panic!("Case name not recognized"),
    };
    let r = repl_state.set_pairs_by_name(&pairs);
    Ok(r)
}

pub fn run_inference_rounds(
    config: &CommandLineOptions,
    resources: &NamespaceBundle,
) -> Result<(), Box<dyn Error>> {
    let model = InferenceModel::new_shared(&resources).unwrap();
    let fact_memory = EmptyBeliefTable::new_shared(&resources)?;
    let proposition_graph = PropositionGraph::new_shared(&model.graph)?;
    proposition_graph.visualize();
    let mut inferencer = Inferencer::new_mutable(model.clone(), proposition_graph.clone(), fact_memory)?;
    inferencer.initialize_chart()?;
    let mut repl = ReplState::new(inferencer);
    if config.test_scenario.clone().unwrap() == "show".to_string() {
        for (i, x) in repl.inferencer.bfs_order.iter().enumerate() {
            println!("{} {:?}", i , x);
        }
    } else {
        repl.inferencer.clear_marginal_output_file()?;
        repl.inferencer.log_table_to_file()?;
        let focus = setup_test_scenario(&config.scenario_name,&config.test_scenario.as_ref().unwrap(), &mut repl)?;
        if focus.is_some() {
            for _i in 0..50 {
                repl.inferencer.do_fan_out_from_node(&focus.clone().unwrap())?;
                repl.inferencer.log_table_to_file()?;
            }
        } else {
            for _i in 0..50 {
                repl.inferencer.do_full_forward_and_backward()?;
                repl.inferencer.log_table_to_file()?;
            }
        }
    }
    Ok(())
}

fn main() {
    let config: bayes_star::common::setup::CommandLineOptions = parse_configuration_options();
    let resources = NamespaceBundle::new(&config).expect("Couldn't create resources.");
    run_inference_rounds(&config, &resources).expect("Testing failed.");
    println!("main finishes");
}
