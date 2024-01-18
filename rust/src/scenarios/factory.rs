use std::{error::Error, rc::Rc};

use crate::common::interface::ScenarioMaker;

use super::{dating_simple::SimpleDating, one_var::OneVariable, two_var::TwoVariable};

pub struct ScenarioMakerFactory;

impl ScenarioMakerFactory {
    pub fn new_shared(type_name: &str) -> Result<Rc<dyn ScenarioMaker>, Box<dyn Error>> {
        match type_name {
            "SimpleDating" => Ok(Rc::new(SimpleDating {})),
            "OneVariable" => Ok(Rc::new(OneVariable {})),
            "TwoVariable" => Ok(Rc::new(TwoVariable {})),
            _ => Err("Unknown ScenarioMaker type".into()),
        }
    }
}
