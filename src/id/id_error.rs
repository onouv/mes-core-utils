use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub enum IdError {
    EmptyIdString,
    InvalidIdString(String),
}

impl Display for IdError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            IdError::EmptyIdString => write!(f, "empty id string"),
            IdError::InvalidIdString(s) => write!(f, "invalid id string: {}", s),
        }
    }
}

impl std::error::Error for IdError {}
