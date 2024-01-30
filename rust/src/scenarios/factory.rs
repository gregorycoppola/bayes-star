use std::{error::Error, rc::Rc};

use crate::common::{interface::ScenarioMaker, resources::FactoryResources};

use super::{dating_simple::SimpleDating, dating_triangle::EligibilityTriangle, one_var::OneVariable, two_var::TwoVariable};

pub struct ScenarioMakerFactory;

impl ScenarioMakerFactory {
    pub fn new_shared(resources: &FactoryResources) -> Result<Rc<dyn ScenarioMaker>, Box<dyn Error>> {
        match resources.config.scenario_name.as_str() {
            "SimpleDating" => Ok(Rc::new(SimpleDating {})),
            "EligibilityTriangle" => Ok(Rc::new(EligibilityTriangle {})),
            "OneVariable" => Ok(Rc::new(OneVariable {})),
            "TwoVariable" => Ok(Rc::new(TwoVariable {})),
            _ => Err("Unknown ScenarioMaker type".into()),
        }
    }
}
