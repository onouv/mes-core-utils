//! # Builder
//!
//! Trait to define how ID builders should behave.
//!

use super::IdError;

pub trait Builder {
    type IdType;

    /// Overwrites the existing id field. Must provide a string beginning with correct prefix and
    /// using correct segment separators to avoid failing. The id string must also contain segments
    /// of equal length.   
    /// Example: "+0100.0001.0125" is a valid id string for prefix "+" and segment delimiter "."
    fn id(&mut self, id: &str) -> Result<(), IdError>;

    /// Append a segment to the id, if it fulfills the formatting requirements. Otherwise
    /// returns an error describing the issue.
    fn add_segment(&mut self, segment: &str) -> Result<(), IdError>;

    /// instanciate the IdType
    fn build(self) -> Self::IdType;
}
