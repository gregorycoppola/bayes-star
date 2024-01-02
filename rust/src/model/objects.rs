use serde::{Deserialize, Serialize};
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::fmt;
use std::hash::{Hash, Hasher};
const BOUND_VARIABLE: &str = "?";

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum ArgumentType {
    Constant,
    Variable,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Domain {
    Jack,
    Jill,
    Verb,
}

impl Domain {
    pub fn from_str(s: &str) -> Option<Domain> {
        match s {
            "Jack" => Some(Domain::Jack),
            "Jill" => Some(Domain::Jill),
            "Verb" => Some(Domain::Verb),
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
        };
        write!(f, "{}", domain_str)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum FirstOrderArgument {
    Constant(ConstantArgument),
    Variable(VariableArgument),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct ConstantArgument {
    pub domain: Domain,
    pub entity_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct VariableArgument {
    pub domain: Domain,
}

impl ConstantArgument {
    pub fn new(domain: Domain, entity_id: String) -> Self {
        ConstantArgument { domain, entity_id }
    }

    pub fn search_string(&self) -> String {
        self.entity_id.clone()
    }
}

impl VariableArgument {
    pub fn new(domain: Domain) -> Self {
        VariableArgument { domain }
    }

    pub fn search_string(&self) -> String {
        format!("?{}", self.domain)
    }
}

impl FirstOrderArgument {
    pub fn search_string(&self) -> String {
        match self {
            FirstOrderArgument::Constant(arg) => arg.search_string(),
            FirstOrderArgument::Variable(arg) => arg.search_string(),
        }
    }

    pub fn convert_to_quantified(&self) -> FirstOrderArgument {
        match self {
            FirstOrderArgument::Constant(arg) => {
                FirstOrderArgument::Variable(VariableArgument::new(arg.domain.clone()))
            }
            FirstOrderArgument::Variable(arg) => FirstOrderArgument::Variable(arg.clone()),
        }
    }

    pub fn is_constant(&self) -> bool {
        match self {
            FirstOrderArgument::Constant(_) => true,
            FirstOrderArgument::Variable(_) => false,
        }
    }

    pub fn is_variable(&self) -> bool {
        !self.is_constant()
    }
}

impl fmt::Display for ConstantArgument {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Customize the formatting as needed
        write!(f, "{:?}", self) // For example, you can use Debug formatting here
    }
}

impl fmt::Display for VariableArgument {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Customize the formatting as needed
        write!(f, "{:?}", self) // For example, you can use Debug formatting here
    }
}

impl fmt::Display for FirstOrderArgument {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FirstOrderArgument::Constant(arg) => write!(f, "Constant({})", arg), // Update as needed
            FirstOrderArgument::Variable(arg) => write!(f, "Variable({})", arg), // Update as needed
                                                                                  // Add cases for other variants if they exist
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FilledRole {
    pub role_name: String,
    pub argument: FirstOrderArgument,
}

impl FilledRole {
    pub fn new(role_name: String, argument: FirstOrderArgument) -> Self {
        FilledRole {
            role_name,
            argument,
        }
    }

    pub fn search_string(&self) -> String {
        format!("{}={}", self.role_name, self.argument.search_string())
    }

    pub fn convert_to_quantified(&self) -> FilledRole {
        FilledRole::new(
            self.role_name.clone(),
            self.argument.convert_to_quantified(),
        )
    }
    pub fn do_substitution(&self, value: FirstOrderArgument) -> FilledRole {
        FilledRole::new(self.role_name.clone(), value)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Proposition {
    pub roles: Vec<FilledRole>,
}

impl Proposition {
    pub fn new(roles: Vec<FilledRole>) -> Self {
        Proposition { roles }
    }

    pub fn search_string(&self) -> String {
        let role_strings: Vec<String> = self.roles
            .iter()
            .map(|role| role.search_string()) // Assuming FilledRole has a search_string method
            .collect();

        format!("[{}]", role_strings.join(","))
    }
    pub fn role_names(&self) -> Vec<String> {
        self.roles
            .iter()
            .map(|role| role.role_name.clone())
            .collect()
    }
    pub fn is_fact(&self) -> bool {
        self.roles.iter().all(|role| role.argument.is_constant()) // Assuming `is_constant` method exists on Argument
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Implication {
    pub premise: Proposition,
    pub conclusion: Proposition,
    pub role_map: RoleMap,
}

impl Implication {
    // Return the search string based on the conclusion's search string
    pub fn search_string(&self) -> String {
        self.conclusion.search_string()
    }

    // Generate a unique key for the implication
    pub fn unique_key(&self) -> String {
        format!(
            "{}->{}{}",
            self.premise.search_string(),
            self.conclusion.search_string(),
            self.mapping_string()
        )
    }

    // Generate a feature string based on the premise and the role map
    pub fn feature_string(&self) -> String {
        format!("{}{}", self.premise.search_string(), self.mapping_string())
    }

    // Convert the role map to a string
    fn mapping_string(&self) -> String {
        self.role_map.to_string() // Assuming RoleMap has a ToString implementation
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

impl RoleMap {
    pub fn new(role_map: HashMap<String, String>) -> Self {
        RoleMap { role_map }
    }

    pub fn get(&self, role_name: &str) -> Option<&String> {
        self.role_map.get(role_name)
    }
}

impl fmt::Display for RoleMap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut entries: Vec<_> = self.role_map.iter().collect();
        // Sort the entries by key
        entries.sort_by(|(a_key, _), (b_key, _)| a_key.cmp(b_key));

        let entries_str: Vec<String> = entries
            .into_iter()
            .map(|(key, value)| format!("{}: {}", key, value))
            .collect();

        write!(f, "{{{}}}", entries_str.join(", "))
    }
}

#[derive(Debug, Clone)]
pub struct BackLink {
    pub implication: Implication,
    pub proposition: Proposition,
}

impl BackLink {
    pub fn new(implication: Implication, proposition: Proposition) -> Self {
        BackLink {
            implication,
            proposition,
        }
    }
}
