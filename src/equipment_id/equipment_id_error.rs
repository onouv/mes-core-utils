use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub enum EquipmentIdError {
    EmptyIdString,
    InvalidIdString(String),
    Uninitialized,
}

impl Display for EquipmentIdError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            EquipmentIdError::EmptyIdString => write!(f, "empty id string"),
            EquipmentIdError::InvalidIdString(s) => write!(f, "invalid id string: {}", s),
            EquipmentIdError::Uninitialized => write!(f, "uninitialized id string"),
        }
    }
}

impl std::error::Error for EquipmentIdError {}
