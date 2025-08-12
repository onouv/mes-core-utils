use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub enum PlantItemIdError {
    EmptyIdString,
    InvalidIdString(String),
    Uninitialized,
}

impl Display for PlantItemIdError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            PlantItemIdError::EmptyIdString => write!(f, "empty id string"),
            PlantItemIdError::InvalidIdString(s) => write!(f, "invalid id string: {}", s),
            PlantItemIdError::Uninitialized => write!(f, "uninitialized id string"),
        }
    }
}

impl std::error::Error for PlantItemIdError {}
