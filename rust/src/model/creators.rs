use crate::model::objects::*;

// Import the necessary structs and enums
use crate::model::objects::{Implication, Proposition, FilledRole, VariableArgument, ConstantArgument};

// Function to create an Implication
fn implication(premise: Proposition, conclusion: Proposition, role_map: RoleMap) -> Implication {
    Implication { premise, conclusion, role_map }
}

// Function to create a Proposition
fn predicate(roles: Vec<FilledRole>) -> Proposition {
    Proposition { roles }
}

// Function to create a FilledRole
fn role(role_name: String, argument: FirstOrderArgument) -> FilledRole {
    // Assuming logger.noop is a logging function, you can implement similar functionality in Rust if needed.
    // For this example, it's omitted.
    FilledRole { role_name, argument }
}

// Function to create a VariableArgument
fn variable(domain: Domain) -> FirstOrderArgument {
    FirstOrderArgument::Variable(VariableArgument { domain })
}

// Function to create a ConstantArgument
fn constant(domain: Domain, entity_id: String) -> FirstOrderArgument {
    FirstOrderArgument::Constant(ConstantArgument { domain, entity_id })
}

// Helper functions for specific roles
fn subject(argument: FirstOrderArgument) -> FilledRole {
    role("subject".to_string(), argument)
}

fn object(argument: FirstOrderArgument) -> FilledRole {
    role("object".to_string(), argument)
}

fn relation(argument: FirstOrderArgument) -> FilledRole {
    role("relation".to_string(), argument)
}
