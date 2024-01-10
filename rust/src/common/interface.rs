use std::error::Error;

use crate::model::{storage::Storage, objects::{Proposition, Conjunction}};

trait LogicalModel {
    fn train_on_example(conjunct:&Conjunction, proposition:&Proposition) -> Result<(), Box<dyn Error>>;
}