pub mod id_error;
pub use super::id_error::IdError;

mod tests;

pub mod equipment;
pub use equipment::*;

pub mod function;
pub use function::*;

use std::fmt;
use std::fmt::{Display, Formatter};

pub static ID_SEGMENT_DELIMITER_DEFAULT: char = '.';

#[derive(PartialEq, PartialOrd)]
pub struct Id {
    prefix: char,
    segments: Vec<String>,
    seg_delimiter: char,
}
impl Id {
    pub fn new(prefix: char, seg_delimiter: char, id: &str) -> Result<Self, IdError> {
        if id.is_empty() {
            return Err(IdError::EmptyIdString);
        }

        let prefix_candidate = id.chars().nth(0).unwrap();
        if prefix_candidate != prefix {
            return Err(IdError::InvalidIdString(String::from("mismatching prefix")));
        }

        if count_occurrence(prefix, id) > 1 {
            return Err(IdError::InvalidIdString(String::from(
                "prefix used more than once",
            )));
        }
        let stripped = id.trim_matches(prefix);
        let segments: Vec<String> = stripped
            .split(seg_delimiter)
            .map(|v| v.to_string())
            .collect();

        if segments.is_empty() {
            return Err(IdError::InvalidIdString(String::from("no code groups")));
        }

        if segments[0].is_empty() {
            return Err(IdError::InvalidIdString(String::from("no code groups")));
        }

        let mut last_group_len = segments[0].len();
        for s in segments.clone() {
            let group_len = s.len();
            if last_group_len != group_len {
                return Err(IdError::InvalidIdString(String::from(
                    "code group deviates in length",
                )));
            }
            last_group_len = group_len;
        }

        Ok(Self {
            prefix,
            segments,
            seg_delimiter,
        })
    }
}

impl Display for Id {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let delim = String::from(self.seg_delimiter);
        let prefix = String::from(self.prefix);
        let segs = self.segments.join(&delim);
        let id = format!("{}{}", prefix, segs);

        write!(f, "{}", id)
    }
}

fn count_occurrence(c: char, s: &str) -> usize {
    s.chars().filter(|v| *v == c).count()
}
