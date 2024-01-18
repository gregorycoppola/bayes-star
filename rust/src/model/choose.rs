use super::objects::{PredicateFactor, Proposition};
use super::ops::{convert_to_proposition, convert_to_quantified, extract_premise_role_map};
use crate::common::graph::InferenceGraph;
use crate::common::model::{FactorContext, InferenceModel};
use crate::inference::graph::PropositionFactor;
use crate::model::objects::{GroupRoleMap, PropositionGroup, RoleMap, EXISTENCE_FUNCTION};
use crate::{
    common::interface::PropositionDB,
    model::objects::{Predicate, PredicateGroup},
};
use crate::{print_green, print_red, print_yellow};
use std::collections::{HashMap, HashSet};
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
    let num_roles = proposition.predicate.roles().len();
    let configurations1 = compute_choose_configurations(num_roles, 1);
    let configurations2 = compute_choose_configurations(num_roles, 2);
    let roles = proposition.predicate.role_names();
    let mut result = Vec::new();
    for configuration in configurations1.into_iter().chain(configurations2) {
        let quantified_roles = extract_roles_from_indices(&roles, &configuration);
        let quantified = convert_to_quantified(proposition, &quantified_roles);
        result.push(quantified);
    }
    Ok(result)
}

pub fn extract_backimplications_from_proposition(
    graph: &InferenceGraph,
    conclusion: &Proposition,
) -> Result<Vec<PropositionFactor>, Box<dyn Error>> {
    print_yellow!(
        "Computing backimplications for proposition {:?}",
        conclusion
    );
    let search_keys = compute_search_predicates(conclusion)?;
    print_yellow!("Computed search_keys {:?}", &search_keys);
    let mut backimplications = Vec::new();
    for predicate in &search_keys {
        print_yellow!("Processing search_key {:?}", &predicate.hash_string());
        let implications = graph.predicate_backward_links(&predicate)?;
        print_yellow!("Found implications {:?}", &implications);
        for implication in &implications {
            let mut terms = Vec::new();
            for (index, proposition) in implication.premise.terms.iter().enumerate() {
                print_yellow!("Processing term {}: {:?}", index, proposition);
                let extracted_mapping =
                    extract_premise_role_map(&conclusion, &implication.role_maps.role_maps[index]);
                print_yellow!(
                    "Extracted mapping for term {}: {:?}",
                    index,
                    &extracted_mapping
                );
                let extracted_proposition =
                    convert_to_proposition(&proposition, &extracted_mapping)?;
                print_yellow!(
                    "Converted to proposition for term {}: {:?}",
                    index,
                    extracted_proposition
                );
                terms.push(extracted_proposition);
            }
            backimplications.push(PropositionFactor {
                premise: PropositionGroup { terms },
                conclusion: conclusion.clone(),
                inference: implication.clone(),
            });
        }
    }
    trace!("Returning backimplications {:?}", &backimplications);
    debug!(
        "Completed computing backimplications, total count: {}",
        backimplications.len()
    );
    Ok(backimplications)
}

pub fn extract_existence_factor_for_predicate(
    conclusion: &Predicate,
) -> Result<PredicateFactor, Box<dyn Error>> {
    let mut new_roles = vec![];
    let mut mapping = HashMap::new();
    for old_role in &conclusion.roles() {
        new_roles.push(old_role.convert_to_quantified());
        mapping.insert(old_role.role_name.clone(), old_role.role_name.clone());
    }
    let premise = Predicate {
        function: EXISTENCE_FUNCTION.to_string(),
        roles: new_roles,
    };
    let role_map = RoleMap::new(mapping);
    let premise_group = PredicateGroup::new(vec![premise]);
    let mapping_group = GroupRoleMap::new(vec![role_map]);
    let factor = PredicateFactor {
        premise: premise_group,
        role_maps: mapping_group,
        conclusion: conclusion.clone(),
    };
    print_green!("extracted existence predicate {:?}", &factor);
    Ok(factor)
}

pub fn extract_existence_factor_for_proposition(
    basis: &Proposition,
) -> Result<PredicateFactor, Box<dyn Error>> {
    let mut new_roles = vec![];
    let mut mapping = HashMap::new();
    for old_role in &basis.predicate.roles() {
        new_roles.push(old_role.convert_to_quantified());
        mapping.insert(old_role.role_name.clone(), old_role.role_name.clone());
    }
    let premise = Predicate {
        function: EXISTENCE_FUNCTION.to_string(),
        roles: new_roles.clone(),
    };
    let role_map = RoleMap::new(mapping);
    let premise_group = PredicateGroup::new(vec![premise]);
    let mapping_group = GroupRoleMap::new(vec![role_map]);
    let conclusion = Predicate {
        function: basis.predicate.function.clone(),
        roles: new_roles.clone(),
    };
    let factor = PredicateFactor {
        premise: premise_group,
        role_maps: mapping_group,
        conclusion,
    };
    print_green!("extracted existence predicate {:?}", &factor);
    Ok(factor)
}
