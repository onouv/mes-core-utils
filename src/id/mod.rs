pub mod id_error;
pub use super::id_error::IdError;

pub mod equipment;
pub use equipment::*;

pub mod function;
pub use function::*;

pub mod location;
pub use location::*;

pub mod builder;
use builder::*;

mod tests;

use std::fmt;
use std::fmt::{Display, Formatter};

/// The canonical standard delimiter between two id segments
pub static ID_SEGMENT_DELIMITER_DEFAULT: &str = ".";

/// A type to formally identify entities in the mes system.
/// This type is intended to be wrapped by user-facing types,
/// rather than used directly.
#[derive(PartialEq, PartialOrd)]
pub struct Id {
    prefix: String,
    segments: Vec<String>,
    seg_delimiter: String,
}
impl Id {
    pub fn new(prefix: &str, seg_delimiter: &str, id: &str) -> Result<Self, IdError> {
        let mut builder = IdBuilder::new(prefix, seg_delimiter);
        builder.id(id)?;

        Ok(builder.build())
    }
}

impl Display for Id {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let segs: String = self.segments.join(self.seg_delimiter.as_str());
        let id = format!("{}{}", self.prefix, segs);

        write!(f, "{}", id)
    }
}

pub struct IdBuilder {
    prefix: String,
    segments: Vec<String>,
    seg_delimiter: String,
}

impl IdBuilder {
    /// Creates a new IdBuilder which is initialized to use the given
    /// prefix and segment delimiter strings.
    fn new(prefix: &str, segment_delimiter: &str) -> Self {
        Self {
            prefix: String::from(prefix),
            seg_delimiter: String::from(segment_delimiter),
            segments: vec![],
        }
    }

    /// checks that the id &str provided fulfills the formatting rules.
    /// if it does, it returns the segments of this ID_SEGMENT_DELIMITER_DEFAULTed id for further use,
    /// otherwise it returns an error describing the issue.
    fn validate_id(&self, id: &str) -> Result<Vec<String>, IdError> {
        if id.is_empty() {
            return Err(IdError::EmptyIdString);
        }

        if !id.starts_with(self.prefix.as_str()) {
            return Err(IdError::InvalidIdString(String::from("mismatching prefix")));
        }

        let r = id.split_once(self.prefix.as_str());
        match r {
            Some((_, post)) => {
                let another_prefix = post.find(self.prefix.as_str());
                if another_prefix.is_some() {
                    return Err(IdError::InvalidIdString(String::from(
                        "prefix used more than once",
                    )));
                }
                let postfix = String::from(post);
                if postfix.is_empty() {
                    return Err(IdError::InvalidIdString(String::from("no segments")));
                }

                let segments: Vec<String> = postfix
                    .split(self.seg_delimiter.as_str())
                    .map(|v| v.to_string())
                    .collect();

                if segments.is_empty() {
                    return Err(IdError::InvalidIdString(String::from("no segments")));
                }

                if segments[0].is_empty() {
                    return Err(IdError::InvalidIdString(String::from("no segments")));
                }

                let mut last_group_len = segments[0].len();
                for s in segments.clone() {
                    let group_len = s.len();
                    if last_group_len != group_len {
                        return Err(IdError::InvalidIdString(String::from(
                            "segment deviates in length",
                        )));
                    }
                    last_group_len = group_len;
                }

                return Ok(segments);
            }
            None => {
                return Err(IdError::InvalidIdString(String::from(
                    "Unknown formatting error",
                )));
            }
        }
    }

    fn validate_segment(&mut self, segment: &str) -> Result<String, IdError> {
        if self.segments.is_empty() {
            return Ok(String::from(segment));
        }

        let mut last_group_len = self.segments[0].len();
        for s in self.segments.clone() {
            let group_len = s.len();
            if last_group_len != group_len {
                return Err(IdError::InvalidIdString(String::from(
                    "segment deviates in length",
                )));
            }
            last_group_len = group_len;
        }

        Ok(String::from(segment))
    }
}

impl Builder for IdBuilder {
    type IdType = Id;

    fn id(&mut self, id: &str) -> Result<(), IdError> {
        match self.validate_id(id) {
            Ok(segments) => {
                self.segments = segments;
                return Ok(());
            }
            Err(err) => {
                return Err(err);
            }
        }
    }

    fn add_segment(&mut self, segment: &str) -> Result<(), IdError> {
        match self.validate_segment(segment) {
            Ok(segment) => {
                self.segments.push(segment);
                return Ok(());
            }
            Err(err) => {
                return Err(err);
            }
        }
    }

    fn build(self) -> Self::IdType {
        Id {
            prefix: self.prefix,
            seg_delimiter: self.seg_delimiter,
            segments: self.segments,
        }
    }
}
