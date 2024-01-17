use crate::model::objects::*;

// Import the necessary structs and enums
use crate::model::objects::{
    ConstantArgument, LabeledArgument, PredicateInferenceFactor, Predicate, VariableArgument,
};

pub fn conjunction(terms: Vec<Predicate>) -> PredicateGroup {
    PredicateGroup { terms }
}

pub fn implication(
    premise: PredicateGroup,
    conclusion: Predicate,
    role_maps: Vec<RoleMap>,
) -> PredicateInferenceFactor {
    let role_maps = GroupRoleMap { role_maps };
    PredicateInferenceFactor {
        premise,
        conclusion,
        role_maps,
    }
}

pub fn proposition(function:String, roles: Vec<LabeledArgument>) -> Proposition {
    Proposition::from(Predicate { function,  roles })
}
pub fn predicate(function:String, roles: Vec<LabeledArgument>) -> Predicate {
    Predicate { function, roles }
}

// Function to create a FilledRole
pub fn role(role_name: String, argument: Argument) -> LabeledArgument {
    // Assuming logger.noop is a logging function, you can implement similar functionality in Rust if needed.
    // For this example, it's omitted.
    LabeledArgument {
        role_name,
        argument,
    }
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
pub fn subject(argument: Argument) -> LabeledArgument {
    role("subject".to_string(), argument)
}

pub fn object(argument: Argument) -> LabeledArgument {
    role("object".to_string(), argument)
}