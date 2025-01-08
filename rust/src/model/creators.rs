use crate::model::objects::*;

// Import the necessary structs and enums
use crate::model::objects::{
    ConstantArgument, LabeledArgument, Predicate, PredicateFactor, VariableArgument,
};

pub fn conjunction(terms: Vec<Predicate>) -> PredicateGroup {
    PredicateGroup { terms }
}

pub fn implication(
    premise: PredicateGroup,
    conclusion: Predicate,
    role_maps: Vec<RoleMap>,
) -> PredicateFactor {
    let role_maps = GroupRoleMap { role_maps };
    PredicateFactor {
        premise,
        conclusion,
        role_maps,
    }
}

pub fn relation(relation_name: String, roles: Vec<VariableArgument>) -> Relation {
    Relation::new(relation_name, roles)
}

pub fn proposition(relation: Relation, roles: Vec<LabeledArgument>) -> Proposition {
    Proposition::from(Predicate::new(relation, roles))
}

pub fn predicate(relation: Relation, roles: Vec<LabeledArgument>) -> Predicate {
    Predicate::new(relation, roles)
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
pub fn variable(domain: String) -> Argument {
    Argument::Variable(VariableArgument { domain })
}

// Function to create a ConstantArgument
pub fn constant(domain: String, entity_id: String) -> Argument {
    Argument::Constant(ConstantArgument { domain, entity_id })
}

// Helper functions for specific roles
pub fn sub(argument: Argument) -> LabeledArgument {
    role("sub".to_string(), argument)
}

pub fn obj(argument: Argument) -> LabeledArgument {
    role("obj".to_string(), argument)
}
