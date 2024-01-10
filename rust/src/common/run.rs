pub trait ScenarioMaker {
    fn setup_scenario(storage:&mut Storage) -> Result<(), Box<dyn Error>>;
}

pub fn train_and_test(scenario_maker:&dyn ScenarioMaker) -> Result<(), Box<dyn Error>> {
    Ok(())
}