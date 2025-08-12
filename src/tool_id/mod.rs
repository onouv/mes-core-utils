pub mod tool_id_error;
pub use super::tool_id_error::ToolIdError;

mod tests;

use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(PartialEq, PartialOrd)]
pub struct ToolId {
    prefix: char,
    segments: Vec<String>,
    seg_delimiter: char,
    group_len: usize,
}

impl ToolId {
    pub fn new(
        prefix: char,
        seg_delimiter: char,
        group_len: usize,
        id: &str,
    ) -> Result<Self, ToolIdError> {
        if id.is_empty() {
            return Err(ToolIdError::EmptyIdString);
        }

        let prefix_candidate = id.chars().nth(0).unwrap();
        if prefix_candidate != prefix {
            return Err(ToolIdError::InvalidIdString(String::from(
                "mismatching prefix",
            )));
        }

        let stripped = id.trim_matches(prefix);
        //let groups: Vec<&str> = stripped.split(seg_delimiter).collect();
        let segments: Vec<String> = stripped
            .split(seg_delimiter)
            .map(|v| v.to_string())
            .collect();

        if segments.is_empty() {
            return Err(ToolIdError::InvalidIdString(String::from("no code groups")));
        }

        if segments[0].is_empty() {
            return Err(ToolIdError::InvalidIdString(String::from("no code groups")));
        }

        for s in segments.clone() {
            if s.len() != group_len {
                return Err(ToolIdError::InvalidIdString(String::from(
                    "code group deviates in length",
                )));
            }
        }

        Ok(Self {
            prefix,
            segments,
            seg_delimiter,
            group_len,
        })
    }
}

impl Display for ToolId {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let delim = String::from(self.seg_delimiter);
        let prefix = String::from(self.prefix);
        let segs = self.segments.join(&delim);
        let id = format!("{}{}", prefix, segs);

        write!(f, "{}", id)
    }
}

impl Default for ToolId {
    fn default() -> Self {
        Self {
            prefix: '=',
            segments: vec![String::from("001")],
            seg_delimiter: '.',
            group_len: 3,
        }
    }
}
