pub mod equipment_id_error;
pub use super::equipment_id_error::EquipmentIdError;

mod tests;

use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(PartialEq, PartialOrd)]
pub struct EquipmentId {
    prefix: char,
    segments: Vec<String>,
    seg_delimiter: char,
    group_len: usize,
}

impl EquipmentId {
    pub fn new(
        prefix: char,
        seg_delimiter: char,
        group_len: usize,
        id: &str,
    ) -> Result<Self, EquipmentIdError> {
        if id.is_empty() {
            return Err(EquipmentIdError::EmptyIdString);
        }

        let prefix_candidate = id.chars().nth(0).unwrap();
        if prefix_candidate != prefix {
            return Err(EquipmentIdError::InvalidIdString(String::from(
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
            return Err(EquipmentIdError::InvalidIdString(String::from(
                "no code groups",
            )));
        }

        if segments[0].is_empty() {
            return Err(EquipmentIdError::InvalidIdString(String::from(
                "no code groups",
            )));
        }

        for s in segments.clone() {
            if s.len() != group_len {
                return Err(EquipmentIdError::InvalidIdString(String::from(
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

impl Display for EquipmentId {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let delim = String::from(self.seg_delimiter);
        let prefix = String::from(self.prefix);
        let segs = self.segments.join(&delim);
        let id = format!("{}{}", prefix, segs);

        write!(f, "{}", id)
    }
}

impl Default for EquipmentId {
    fn default() -> Self {
        Self {
            prefix: '=',
            segments: vec![String::from("001")],
            seg_delimiter: '.',
            group_len: 3,
        }
    }
}
