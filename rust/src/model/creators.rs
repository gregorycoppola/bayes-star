use crate::model::objects::*;

// Import the necessary structs and enums
use crate::model::objects::{Implication, Proposition, FilledRole, VariableArgument, ConstantArgument};

// Function to create an Implication
pub fn implication(premise: Conjunction, conclusion: Proposition, role_map: RoleMap) -> Implication {
    Implication { premise, conclusion, role_map }
}

// Function to create a Proposition
pub fn predicate(roles: Vec<FilledRole>) -> Proposition {
    Proposition { roles }
}

// Function to create a FilledRole
pub fn role(role_name: String, argument: FirstOrderArgument) -> FilledRole {
    // Assuming logger.noop is a logging function, you can implement similar functionality in Rust if needed.
    // For this example, it's omitted.
    FilledRole { role_name, argument }
}

// Function to create a VariableArgument
pub fn variable(domain: Domain) -> FirstOrderArgument {
    FirstOrderArgument::Variable(VariableArgument { domain })
}

// Function to create a ConstantArgument
pub fn constant(domain: Domain, entity_id: String) -> FirstOrderArgument {
    FirstOrderArgument::Constant(ConstantArgument { domain, entity_id })
}

// Helper functions for specific roles
pub fn subject(argument: FirstOrderArgument) -> FilledRole {
    role("subject".to_string(), argument)
}

pub fn object(argument: FirstOrderArgument) -> FilledRole {
    role("object".to_string(), argument)
}

pub fn relation(argument: FirstOrderArgument) -> FilledRole {
    role("relation".to_string(), argument)
}
