use crate::tool_id_error::*;
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

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn can_be_created() {
        let seg_delimiter = '.';
        let prefix = '+';
        let group_len = 4;
        let candidate = "+0100.0001";
        let tool_id = ToolId::new(prefix, seg_delimiter, group_len, candidate)
            .unwrap_or_else(|error| panic!("{}", error));

        println!("{}", tool_id);

        let id = format!("{}", tool_id);

        assert_eq!(id, candidate);
    }

    #[test]
    #[should_panic(expected = "code group deviates in length")]
    fn wrong_group_length_cannot_be_created() {
        let seg_delimiter = '.';
        let prefix = '+';
        let group_len = 4;
        let candidate = "+0100.001";
        ToolId::new(prefix, seg_delimiter, group_len, candidate)
            .unwrap_or_else(|error| panic!("{}", error));
    }

    #[test]
    #[should_panic(expected = "code group deviates in length")]
    fn wrong_delimiter_cannot_be_created() {
        let seg_delimiter = '*';
        let prefix = '+';
        let group_len = 4;
        let candidate = "+0100.0001";
        ToolId::new(prefix, seg_delimiter, group_len, candidate)
            .unwrap_or_else(|error| panic!("{}", error));
    }

    #[test]
    #[should_panic(expected = "code group deviates in length")]
    fn lack_of_code_groups_canot_be_created() {
        let seg_delimiter = '.';
        let prefix = '+';
        let group_len = 4;
        let candidate = "+";
        ToolId::new(prefix, seg_delimiter, group_len, candidate)
            .unwrap_or_else(|error| panic!("{}", error));
    }
}
