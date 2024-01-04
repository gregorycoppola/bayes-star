use std::error::Error;

use crate::model::{maxent::{features_from_backlinks, compute_probability}, weights::read_weights, choose::compute_backlinks};

use super::{objects::{Proposition, Conjunction}, storage::Storage};

fn ensure_probabilities_are_stored(storage:&mut Storage, conjunction:&Conjunction) -> Result<(), Box<dyn Error>> {
    Ok(())
}
pub fn inference_probability(storage:&mut Storage, proposition:&Proposition) -> Result<f64, Box<dyn Error>> {
    trace!("inference_probability - Start: {:?}", proposition);
    trace!("inference_probability - Getting features from backlinks");
    let backlinks = compute_backlinks(storage, &proposition)?;

    let features = match features_from_backlinks(storage, &backlinks) {
        Ok(f) => f,
        Err(e) => {
            trace!("inference_probability - Error in features_from_backlinks: {:?}", e);
            return Err(e);
        }
    };

    trace!("inference_probability - Reading weights");
    let weight_vector = match read_weights(
        storage.get_redis_connection(),
        &features.keys().cloned().collect::<Vec<_>>(),
    ) {
        Ok(w) => w,
        Err(e) => {
            trace!("inference_probability - Error in read_weights: {:?}", e);
            return Err(e);
        }
    };

    trace!("inference_probability - Computing probability");
    let probability = compute_probability(&weight_vector, &features);

    Ok(probability)
}