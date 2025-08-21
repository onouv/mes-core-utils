mod tests;

use std::fmt;
use std::fmt::{Debug, Display, Formatter};

use super::id_error::IdError;
use super::{Builder, ID_SEGMENT_DELIMITER_DEFAULT, Id, IdBuilder};

/// The canonical standard prefix for a system
pub const SYSTEM_ID_PREFIX: &str = "#";

/// A type to formally identify systems.
/// This struct wraps the more generic [Id] type and specializes
/// it with a fixed prefix ([SYSTEM_ID_PREFIX]).
#[derive(PartialEq, PartialOrd)]
pub struct SystemId {
    item_id: Id,
}

impl SystemId {
    pub fn builder() -> SystemIdBuilder {
        SystemIdBuilder::new()
    }

    pub fn new(seg_delimiter: &str, id: &str) -> Result<Self, IdError> {
        let item_id = Id::new(SYSTEM_ID_PREFIX, seg_delimiter, id)?;

        Ok(Self { item_id })
    }
}

impl Display for SystemId {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.item_id.fmt(f)
    }
}

impl Default for SystemId {
    fn default() -> Self {
        // we carefully provide parameters secured at compile time
        // to this call to ensure that it will not fail, unless
        // something is fundamentally broken in which case we deliberatly panic
        let item_id = Id::new(SYSTEM_ID_PREFIX, ID_SEGMENT_DELIMITER_DEFAULT, "-001").unwrap();

        Self { item_id }
    }
}

impl Debug for SystemId {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.item_id.fmt(f)
    }
}

pub struct SystemIdBuilder {
    builder: IdBuilder,
}

impl SystemIdBuilder {
    /// Creates a new builder, initialized to use [SYSTEM_ID_PREFIX] and
    /// [ID_SEGMENT_DELIMITER_DEFAULT].
    pub fn new() -> Self {
        Self {
            builder: IdBuilder::new(SYSTEM_ID_PREFIX, ID_SEGMENT_DELIMITER_DEFAULT),
        }
    }
}

impl Default for SystemIdBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl Builder for SystemIdBuilder {
    type IdType = SystemId;

    fn id(&mut self, id: &str) -> Result<(), IdError> {
        self.builder.id(id)
    }

    fn add_segment(&mut self, segment: &str) -> Result<(), IdError> {
        self.builder.add_segment(segment)
    }

    fn build(self) -> Self::IdType {
        SystemId {
            item_id: self.builder.build(),
        }
    }
}
