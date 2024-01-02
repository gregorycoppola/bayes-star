use std::fmt;
use serde::Serialize;

const BOUND_VARIABLE: &str = "?";

#[derive(Serialize, Debug)]
pub enum ArgumentType {
    Constant,
    Variable,
}

#[derive(Serialize, Debug)]
pub enum Domain {
    Jack,
    Jill,
    Verb,
}

impl Domain {
    pub fn from_str(s: &str) -> Option<Domain> {
        match s {
            "jack" => Some(Domain::Jack),
            "jill" => Some(Domain::Jill),
            "verb" => Some(Domain::Verb),
            _ => None,
        }
    }
}

impl fmt::Display for Domain {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let domain_str = match self {
            Domain::Jack => "Jack",
            Domain::Jill => "Jill",
            Domain::Verb => "Verb",
            // ... match other variants ...
        };
        write!(f, "{}", domain_str)
    }
}

#[derive(Serialize, Debug)]
pub enum FirstOrderArgument {
    Constant(ConstantArgument),
    Variable(VariableArgument),
}

#[derive(Serialize, Debug)]
pub struct ConstantArgument {
    pub domain: Domain,
    pub entity_id: String,
}

#[derive(Serialize, Debug)]
pub struct VariableArgument {
    pub domain: Domain,
}

#[derive(Serialize, Debug)]
pub struct FilledRole {
    pub role_name: String,
    pub argument: FirstOrderArgument,
}

#[derive(Serialize, Debug)]
pub struct Proposition {
    pub roles: Vec<FilledRole>,
}

#[derive(Debug)]
pub struct Implication {
    pub premise: Proposition,
    pub conclusion: Proposition,
    pub role_map: RoleMap,
}

#[derive(Debug)]
pub struct Entity {
    pub domain: Domain,
    pub name: String,
}

#[derive(Debug)]
pub struct RoleMap {
    pub role_map: std::collections::HashMap<String, String>,
}

#[derive(Debug)]
pub struct BackLink {
    pub implication: Implication,
    pub proposition: Proposition,
}
