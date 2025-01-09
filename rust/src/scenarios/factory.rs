use std::{error::Error, rc::Rc};

use crate::common::{interface::ScenarioMaker, resources::ResourceBundle};

use super::{dating_simple::SimpleDating, one_var::OneVariable};

pub struct ScenarioMakerFactory;

impl ScenarioMakerFactory {
    pub fn new_shared(resources: &ResourceBundle) -> Result<Rc<dyn ScenarioMaker>, Box<dyn Error>> {
        match resources.namespace.as_str() {
            "dating_simple" => Ok(Rc::new(SimpleDating {})),
            // "dating_triangle" => Ok(Rc::new(EligibilityTriangle {})),
            "one_var" => Ok(Rc::new(OneVariable {})),
            // "long_chain" => Ok(Rc::new(long_chain::Scenario {})),
            // "mid_chain" => Ok(Rc::new(mid_chain::Scenario {})),
            // "long_and" => Ok(Rc::new(long_and::Scenario {})),
            // "two_var" => Ok(Rc::new(TwoVariable {})),
            _ => Err("Unknown ScenarioMaker type".into()),
        }
    }
}
