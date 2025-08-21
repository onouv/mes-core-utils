mod tests;

use std::fmt;
use std::fmt::{Debug, Display, Formatter};

use super::id_error::IdError;
use super::{Builder, ID_SEGMENT_DELIMITER_DEFAULT, Id, IdBuilder};

/// The canonical standard prefix for a function
pub const FUNCTION_ID_PREFIX: &str = "=";

/// A type to formally identify functions.
/// This struct wraps the more generic [Id] type and specializes
/// it with a fixed prefix ([FUNCTION_ID_PREFIX]).
#[derive(PartialEq, PartialOrd)]
pub struct FunctionId {
    item_id: Id,
}

impl FunctionId {
    pub fn builder() -> FunctionIdBuilder {
        FunctionIdBuilder::new()
    }

    pub fn new(seg_delimiter: &str, id: &str) -> Result<Self, IdError> {
        let item_id = Id::new(FUNCTION_ID_PREFIX, seg_delimiter, id)?;

        Ok(Self { item_id })
    }
}

impl Display for FunctionId {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.item_id.fmt(f)
    }
}

impl Default for FunctionId {
    fn default() -> Self {
        // we carefully provide parameters secured at compile time
        // to this call to ensure that it will not fail, unless
        // something is fundamentally broken in which case we deliberatly panic
        let item_id = Id::new(FUNCTION_ID_PREFIX, ID_SEGMENT_DELIMITER_DEFAULT, "-001").unwrap();

        Self { item_id }
    }
}

impl Debug for FunctionId {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.item_id.fmt(f)
    }
}

pub struct FunctionIdBuilder {
    builder: IdBuilder,
}

impl FunctionIdBuilder {
    /// Creates a new builder, initialized to use [FUNCTION_ID_PREFIX] and
    /// [ID_SEGMENT_DELIMITER_DEFAULT].
    pub fn new() -> Self {
        Self {
            builder: IdBuilder::new(FUNCTION_ID_PREFIX, ID_SEGMENT_DELIMITER_DEFAULT),
        }
    }
}

impl Default for FunctionIdBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl Builder for FunctionIdBuilder {
    type IdType = FunctionId;

    fn id(&mut self, id: &str) -> Result<(), IdError> {
        self.builder.id(id)
    }

    fn add_segment(&mut self, segment: &str) -> Result<(), IdError> {
        self.builder.add_segment(segment)
    }

    fn build(self) -> Self::IdType {
        FunctionId {
            item_id: self.builder.build(),
        }
    }
}
