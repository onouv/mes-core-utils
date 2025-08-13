use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub enum IdError {
    EmptyIdString,
    InvalidIdString(String),
    Uninitialized,
}

impl Display for IdError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            IdError::EmptyIdString => write!(f, "empty id string"),
            IdError::InvalidIdString(s) => write!(f, "invalid id string: {}", s),
            IdError::Uninitialized => write!(f, "uninitialized id string"),
        }
    }
}

impl std::error::Error for IdError {}
