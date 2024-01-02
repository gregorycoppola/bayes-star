use crate::model::objects::{Domain, Entity, Proposition, FilledRole};

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
