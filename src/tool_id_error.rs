use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub enum ToolIdError {
    EmptyIdString,
    InvalidIdString(String),
    Uninitialized,
}

impl Display for ToolIdError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ToolIdError::EmptyIdString => write!(f, "EmptyIdString"),
            ToolIdError::InvalidIdString(s) => write!(f, "InvalidString: {}", s),
            ToolIdError::Uninitialized => write!(f, "Uninitialized"),
        }
    }
}

impl std::error::Error for ToolIdError {}
