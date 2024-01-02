use std::fmt;
use serde::{Serialize, Deserialize};

const BOUND_VARIABLE: &str = "?";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ArgumentType {
    Constant,
    Variable,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum FirstOrderArgument {
    Constant(ConstantArgument),
    Variable(VariableArgument),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConstantArgument {
    pub domain: Domain,
    pub entity_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VariableArgument {
    pub domain: Domain,
}
impl ConstantArgument {
    fn new(domain: Domain, entity_id: String) -> Self {
        ConstantArgument { domain, entity_id }
    }

    fn search_string(&self) -> String {
        self.entity_id.clone()
    }
}

impl VariableArgument {
    fn new(domain: Domain) -> Self {
        VariableArgument { domain }
    }

    fn search_string(&self) -> String {
        format!("?{}", self.domain)
    }
}

impl FirstOrderArgument {
    fn search_string(&self) -> String {
        match self {
            FirstOrderArgument::Constant(arg) => arg.search_string(),
            FirstOrderArgument::Variable(arg) => arg.search_string(),
        }
    }
    fn convert_to_quantified(&self) -> FirstOrderArgument {
        match self {
            FirstOrderArgument::Constant(arg) => FirstOrderArgument::Variable(VariableArgument::new(arg.domain.clone())),
            FirstOrderArgument::Variable(arg) => FirstOrderArgument::Variable(arg.clone()),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FilledRole {
    pub role_name: String,
    pub argument: FirstOrderArgument,
}

impl FilledRole {
    fn new(role_name: String, argument: FirstOrderArgument) -> Self {
        FilledRole { role_name, argument }
    }

    pub fn search_string(&self) -> String {
        format!("{}={}", self.role_name, self.argument.search_string())
    }

    pub fn convert_to_quantified(&self) -> FilledRole {
        FilledRole::new(self.role_name.clone(), self.argument.convert_to_quantified())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Proposition {
    pub roles: Vec<FilledRole>,
}

impl Proposition {
    pub fn search_string(&self) -> String {
        "dummy_search_string".to_string()
    }
}
impl Proposition {
    pub fn new(roles: Vec<FilledRole>) -> Self {
        Proposition { roles }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Implication {
    pub premise: Proposition,
    pub conclusion: Proposition,
    pub role_map: RoleMap,
}

impl Implication {
    pub fn search_string(&self) -> String {
        "dummy_search_string".to_string()
    }
}

#[derive(Debug, Clone)]
pub struct Entity {
    pub domain: Domain,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RoleMap {
    pub role_map: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct BackLink {
    pub implication: Implication,
    pub proposition: Proposition,
}
