use crate::common::interface::PropositionDB;
use crate::common::model::InferenceModel;
use crate::model::{
    choose::extract_backimplications_from_proposition, exponential::compute_potential,
    weights::CLASS_LABELS,
};
use crate::print_red;
use std::{collections::HashMap, error::Error};

use super::objects::{Predicate, PredicateGroup, Proposition, PropositionGroup, EXISTENCE_FUNCTION};
use super::weights::ExponentialWeights;
