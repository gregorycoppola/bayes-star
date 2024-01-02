use crate::model::storage::Storage;

pub fn setup_scenario(storage: &mut Storage) -> Result<(), String> {
    storage.drop_all_dbs().map_err(|e| e.to_string())?;
    Ok(())
}
