use crate::model::objects::{RoleMap, Entity, Proposition, FilledRole};
use std::{error::Error, collections::HashMap};

use super::objects::FirstOrderArgument;

pub fn convert_to_quantified(proposition: &Proposition, roles: &[String]) -> Proposition {
    let role_set: std::collections::HashSet<String> = roles.iter().cloned().collect();
    let result: Vec<FilledRole> = proposition.roles.iter().map(|crole| {
        if role_set.contains(&crole.role_name) {
            crole.convert_to_quantified()
        } else {
            crole.clone()
        }
    }).collect();

    Proposition::new(result)
}

pub fn convert_to_proposition(
    predicate: &Proposition, 
    role_map: &HashMap<String, FirstOrderArgument>
) -> Result<Proposition, Box<dyn Error>> {
    debug!("Converting to proposition: {:?}", predicate);

    let mut result_roles = Vec::new();

    for role in &predicate.roles {
        debug!("Processing role: {:?}", role);

        if role.argument.is_variable() {
            debug!("Role is a variable, attempting substitution.");

            match role_map.get(&role.role_name) {
                Some(substitute) => {
                    debug!("Substitution found for role: {}", role.role_name);
                    let new_role = role.do_substitution(substitute.clone()); // Assuming this method exists in FilledRole
                    assert!(new_role.argument.is_constant(), "arg must be a constant here");
                    result_roles.push(new_role);
                },
                None => {
                    error!("Substitution not found for role: {}", role.role_name);
                    return Err(format!("Substitution not found for role: {}", role.role_name).into());
                }
            }
        } else {
            debug!("Role is not a variable, pushing as is.");
            result_roles.push(role.clone());
        }
    }

    debug!("Conversion to proposition completed successfully.");
    Ok(Proposition { roles: result_roles })
}

pub fn extract_premise_role_map(proposition: &Proposition, role_map: &RoleMap) -> HashMap<String, FirstOrderArgument> {
    let mut result = std::collections::HashMap::new();
    for crole in &proposition.roles {
        let role_name = &crole.role_name;
        if let Some(premise_role_name) = role_map.get(role_name) {
            result.insert(premise_role_name.clone(), crole.argument.clone());
        }
    }
    result
}
