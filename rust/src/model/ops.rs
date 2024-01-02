use crate::model::objects::{RoleMap, Entity, Proposition, FilledRole};
use std::{error::Error};

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

pub fn convert_to_proposition(predicate: &Proposition, role_map: &RoleMap) -> Result<Proposition, Box<dyn Error>> {
    let mut result_roles = Vec::new();

    for role in &predicate.roles {
        if role.argument.is_variable() {
            if let Some(substitute) = role_map.get(&role.role_name) {
                let new_role = role.do_substitution(substitute); // Assuming this method exists in FilledRole
                result_roles.push(new_role);
            } else {
                // Handle the error if the substitution is not found
                return Err(format!("Substitution not found for role: {}", role.role_name).into());
            }
        } else {
            result_roles.push(role.clone());
        }
    }

    Ok(Proposition { roles: result_roles })
}