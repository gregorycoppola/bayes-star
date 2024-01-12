use crate::model::objects::*;

// Import the necessary structs and enums
use crate::model::objects::{Implication, Predicate, FilledRole, VariableArgument, ConstantArgument};

pub fn conjunction(terms: Vec<Predicate>) -> Conjunction {
    Conjunction { terms }
}

pub fn implication(premise: Conjunction, conclusion: Predicate, role_maps: Vec<RoleMap>) -> Implication {
    let role_maps = ConjunctionRoleMap {role_maps};
    Implication { premise, conclusion, role_maps }
}

// Function to create a Proposition
pub fn proposition(roles: Vec<FilledRole>) -> Predicate {
    Predicate { roles }
}

// Function to create a FilledRole
pub fn role(role_name: String, argument: Argument) -> FilledRole {
    // Assuming logger.noop is a logging function, you can implement similar functionality in Rust if needed.
    // For this example, it's omitted.
    FilledRole { role_name, argument }
}

// Function to create a VariableArgument
pub fn variable(domain: Domain) -> Argument {
    Argument::Variable(VariableArgument { domain })
}

// Function to create a ConstantArgument
pub fn constant(domain: Domain, entity_id: String) -> Argument {
    Argument::Constant(ConstantArgument { domain, entity_id })
}

// Helper functions for specific roles
pub fn subject(argument: Argument) -> FilledRole {
    role("subject".to_string(), argument)
}

pub fn object(argument: Argument) -> FilledRole {
    role("object".to_string(), argument)
}

pub fn relation(argument: Argument) -> FilledRole {
    role("relation".to_string(), argument)
}
