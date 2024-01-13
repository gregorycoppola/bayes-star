use super::conjunction::get_conjunction_probability;
use super::objects::Proposition;
use super::{
    conjunction,
    ops::{convert_to_proposition, convert_to_quantified, extract_premise_role_map},
};
use crate::common::graph::InferenceGraph;
use crate::common::model::{FactorContext, GraphicalModel};
use crate::inference::graph::PropositionInferenceFactor;
use crate::model::objects::PropositionGroup;
use crate::{
    common::{interface::FactDB, model::Factor},
    model::objects::{ Predicate, PredicateGroup},
};
use std::{borrow::Borrow, error::Error};

fn combine(input_array: &[usize], k: usize) -> Vec<Vec<usize>> {
    let mut result = vec![];
    let mut temp_vec = vec![];
    fn run(
        input_array: &[usize],
        k: usize,
        start: usize,
        temp_vec: &mut Vec<usize>,
        result: &mut Vec<Vec<usize>>,
    ) {
        if temp_vec.len() == k {
            result.push(temp_vec.clone());
            return;
        }
        for i in start..input_array.len() {
            temp_vec.push(input_array[i]);
            run(input_array, k, i + 1, temp_vec, result);
            temp_vec.pop();
        }
    }
    run(input_array, k, 0, &mut temp_vec, &mut result);
    result
}

fn compute_choose_configurations(n: usize, k: usize) -> Vec<Vec<usize>> {
    let input_array: Vec<usize> = (0..n).collect();
    combine(&input_array, k)
}

fn extract_roles_from_indices(roles: &[String], indices: &[usize]) -> Vec<String> {
    let index_set: std::collections::HashSet<usize> = indices.iter().cloned().collect();
    roles
        .iter()
        .enumerate()
        .filter_map(|(i, role)| {
            if index_set.contains(&i) {
                Some(role.clone())
            } else {
                None
            }
        })
        .collect()
}

pub fn compute_search_predicates(
    proposition: &Proposition,
) -> Result<Vec<Predicate>, Box<dyn Error>> {
    let num_roles = proposition.predicate.roles.len();
    let configurations1 = compute_choose_configurations(num_roles, 1);
    let configurations2 = compute_choose_configurations(num_roles, 2);
    let roles = proposition.predicate.role_names();
    let mut result = Vec::new();
    for configuration in configurations1.into_iter().chain(configurations2) {
        let quantified_roles = extract_roles_from_indices(&roles, &configuration);
        let quantified = convert_to_quantified(proposition, &quantified_roles); // Assuming this function exists
                                                                                // let hash_string = quantified.hash_string(); // Assuming this method exists
        result.push(quantified);
    }
    Ok(result)
}

pub fn extract_backimplications_from_proposition(
    graph: &InferenceGraph,
    conclusion: &Proposition,
) -> Result<Vec<PropositionInferenceFactor>, Box<dyn Error>> {
    todo!()
    // debug!(
    //     "Computing backimplications for proposition {:?}",
    //     conclusion
    // );
    // let search_keys = compute_search_predicates(conclusion)?;
    // trace!("Computed search_keys {:?}", &search_keys);
    // let mut backimplications = Vec::new();
    // for predicate in &search_keys {
    //     trace!("Processing search_key {:?}", &predicate.hash_string());
    //     let implications = graph.predicate_backward_links(&predicate)?;
    //     trace!("Found implications {:?}", &implications);
    //     for implication in &implications {
    //         let mut terms = Vec::new();
    //         for (index, proposition) in implication.premise.terms.iter().enumerate() {
    //             trace!("Processing term {}: {:?}", index, proposition);
    //             let extracted_mapping =
    //                 extract_premise_role_map(&conclusion, &implication.role_maps.role_maps[index]); // Assuming this function exists
    //             trace!(
    //                 "Extracted mapping for term {}: {:?}",
    //                 index,
    //                 &extracted_mapping
    //             );
    //             let extracted_proposition =
    //                 convert_to_proposition(&proposition, &extracted_mapping)?; // Assuming this function exists
    //             trace!(
    //                 "Converted to proposition for term {}: {:?}",
    //                 index,
    //                 extracted_proposition
    //             );
    //             terms.push(extracted_proposition);
    //         }
    //         backimplications.push(ImplicationInstance::new(
    //             implication.clone(),
    //             PropositionGroup { terms },
    //         ));
    //     }
    // }
    // trace!("Returning backimplications {:?}", &backimplications);
    // debug!(
    //     "Completed computing backimplications, total count: {}",
    //     backimplications.len()
    // );
    // Ok(backimplications)
}

pub fn extract_factor_for_proposition(
    graph: &InferenceGraph,
    conclusion: Proposition,
) -> Result<Factor, Box<dyn Error>> {
    let implications = extract_backimplications_from_proposition(graph, &conclusion)?;
    let factor = Factor {
        conclusion,
        conjuncts: implications,
    };
    Ok(factor)
}

pub fn extract_factor_context_for_proposition(
    fact_db: &Box<dyn FactDB>,
    graph: &InferenceGraph,
    conclusion: Proposition,
) -> Result<FactorContext, Box<dyn Error>> {
    let factor = extract_factor_for_proposition(graph, conclusion)?;
    let mut conjoined_probabilities = vec![];
    for conjoined_implication in &factor.conjuncts {
        let conjoined_probability =
            get_conjunction_probability(fact_db.borrow(), &conjoined_implication.conjunction)?;
        conjoined_probabilities.push(conjoined_probability);
    }
    Ok(FactorContext {
        factor,
        conjoined_probabilities,
    })
}
