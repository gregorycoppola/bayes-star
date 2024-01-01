use std::collections::HashSet;

// Constants
const BOUND_VARIABLE: &str = "?";

// Helper Enums
#[derive(Debug)]
enum ArgumentType {
    Constant,
    Variable,
}

#[derive(Debug)]
enum Domain {
    Jack,
    Jill,
    Verb,
}

// Helper functions to convert strings to enums
impl Domain {
    fn from_str(s: &str) -> Option<Domain> {
        match s {
            "jack" => Some(Domain::Jack),
            "jill" => Some(Domain::Jill),
            "verb" => Some(Domain::Verb),
            _ => None,
        }
    }
}

// FirstOrderArgument
#[derive(Debug)]
enum FirstOrderArgument {
    Constant(ConstantArgument),
    Variable(VariableArgument),
}

// ConstantArgument
#[derive(Debug)]
struct ConstantArgument {
    domain: Domain,
    entity_id: String,
}

// VariableArgument
#[derive(Debug)]
struct VariableArgument {
    domain: Domain,
}

// FilledRole
#[derive(Debug)]
struct FilledRole {
    role_name: String,
    argument: FirstOrderArgument,
}

// Proposition
#[derive(Debug)]
struct Proposition {
    roles: Vec<FilledRole>,
}

// Implication
#[derive(Debug)]
struct Implication {
    premise: Proposition,
    conclusion: Proposition,
    role_map: RoleMap,
}

// Entity
#[derive(Debug)]
struct Entity {
    domain: Domain,
    name: String,
}

// RoleMap
#[derive(Debug)]
struct RoleMap {
    role_map: std::collections::HashMap<String, String>,
}

// BackLink
#[derive(Debug)]
struct BackLink {
    implication: Implication,
    proposition: Proposition,
}

// Implementations for each struct go here
// ...
