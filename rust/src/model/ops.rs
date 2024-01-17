use crate::model::objects::{LabeledArgument, Predicate, RoleMap};
use std::{collections::HashMap, error::Error};

use super::objects::{Argument, Proposition};

pub fn convert_to_quantified(proposition: &Proposition, roles: &[String]) -> Predicate {
    let role_set: std::collections::HashSet<String> = roles.iter().cloned().collect();
    let result: Vec<LabeledArgument> = proposition
        .predicate
        .roles
        .iter()
        .map(|crole| {
            if role_set.contains(&crole.role_name) {
                crole.convert_to_quantified()
            } else {
                crole.clone()
            }
        })
        .collect();

    Predicate::new(proposition.predicate.function.clone(), result)
}

pub fn convert_to_proposition(
    predicate: &Predicate,
    role_map: &HashMap<String, Argument>,
) -> Result<Proposition, Box<dyn Error>> {
    debug!(
        "Converting to proposition: {:?}, role_map {:?}",
        predicate, &role_map
    );
    let mut result_roles = Vec::new();
    for role in &predicate.roles {
        debug!("Processing role: {:?}", role);
        if role.argument.is_variable() {
            debug!("Role is a variable, attempting substitution.");
            match role_map.get(&role.role_name) {
                Some(substitute) => {
                    debug!(
                        "Substitution found for role: {}, substitute: {:?}",
                        role.role_name, substitute
                    );
                    let new_role = role.do_substitution(substitute.clone()); // Assuming this method exists in FilledRole
                    debug!("New role after substitution: {:?}", new_role);

                    assert!(
                        new_role.argument.is_constant(),
                        "After substitution, arg must be a constant in new_role: {:?}",
                        new_role
                    );
                    result_roles.push(new_role);
                }
                None => {
                    error!("Substitution not found for role: {}", role.role_name);
                    return Err(
                        format!("Substitution not found for role: {}", role.role_name).into(),
                    );
                }
            }
        } else {
            debug!("Role is not a variable, pushing as is.");
            result_roles.push(role.clone());
        }
    }
    debug!("Conversion to proposition completed successfully.");
    let function = predicate.function.clone();
    Ok(Proposition {
        predicate: Predicate {
            function,
            roles: result_roles,
        },
    })
}

pub fn extract_premise_role_map(
    proposition: &Proposition,
    role_map: &RoleMap,
) -> HashMap<String, Argument> {
    debug!(
        "Extracting premise role map for proposition: {:?}",
        proposition
    );
    let mut result = HashMap::new();
    for crole in &proposition.predicate.roles {
        assert!(
            crole.argument.is_constant(),
            "crole must be a constant {:?}",
            &crole
        );
        let role_name = &crole.role_name;
        trace!("Processing role: {:?}", crole);
        if let Some(premise_role_name) = role_map.get(role_name) {
            trace!("Mapping found: {} -> {}", role_name, premise_role_name);
            result.insert(premise_role_name.clone(), crole.argument.clone());
        } else {
            trace!("No mapping found for role: {}", role_name);
        }
    }
    debug!("Extraction complete, result: {:?}", result);
    result
}
