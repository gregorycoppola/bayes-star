use crate::model::objects::{BackLink, Domain, Entity, Implication, Proposition};
use crate::model::storage::Storage;
use crate::model::weights::{read_weights, save_weights};
use std::{error::Error, sync::Arc};

use super::ops::{convert_to_quantified, convert_to_proposition};

fn combine(input_array: &[usize], k: usize) -> Vec<Vec<usize>> {
    let mut result = vec![];
    let mut temp_vec = vec![];

    fn run(input_array: &[usize], k: usize, start: usize, temp_vec: &mut Vec<usize>, result: &mut Vec<Vec<usize>>) {
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
    roles.iter().enumerate()
        .filter_map(|(i, role)| {
            if index_set.contains(&i) { Some(role.clone()) } else { None }
        })
        .collect()
}

pub fn compute_search_keys(proposition: &Proposition) -> Result<Vec<String>, Box<dyn Error>> {
    if !proposition.is_fact() {
        return Err("Proposition is not a fact".into());
    }

    let num_roles = proposition.roles.len();
    let configurations1 = compute_choose_configurations(num_roles, 1);
    let configurations2 = compute_choose_configurations(num_roles, 2);
    let roles = proposition.role_names();

    let mut result = Vec::new();
    for configuration in configurations1.into_iter().chain(configurations2) {
        let quantified_roles = extract_roles_from_indices(&roles, &configuration);
        let quantified = convert_to_quantified(proposition, &quantified_roles); // Assuming this function exists
        let search_string = quantified.search_string(); // Assuming this method exists
        result.push(search_string);
    }

    Ok(result)
}

pub fn compute_backlinks(storage: &Storage, proposition: &Proposition) -> Result<Vec<BackLink>, Box<dyn Error>> {
    if !proposition.is_fact() {
        return Err("Proposition is not a fact".into());
    }

    let search_keys = compute_search_keys(proposition)?;
    let mut backlinks = Vec::new();

    for search_key in search_keys {
        let implications = storage.find_premises(&search_key)?; // Assuming this method exists in Storage
        for implication in implications {
            let extracted_mapping = extract_premise_role_map(proposition, &implication.role_map); // Assuming this function exists
            let quantified_premise = &implication.premise;
            let extracted_proposition = convert_to_proposition(quantified_premise, &extracted_mapping)?; // Assuming this function exists
            backlinks.push(BackLink::new(implication, extracted_proposition)); // Assuming a constructor for BackLink exists
        }
    }

    Ok(backlinks)
}