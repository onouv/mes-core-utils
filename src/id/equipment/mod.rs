mod tests;

use std::fmt::{self, Debug, Display, Formatter};

use super::id_error::IdError;
use super::{Builder, ID_SEGMENT_DELIMITER_DEFAULT, Id, IdBuilder};

/// The canocnical standard prefix for an equipment id.
pub const EQUIPMENT_ID_PREFIX: &str = "-";

/// A type to formally identify equipment
/// This struct wraps the more generic [Id] type and specializes
/// it with a fixed prefix.
#[derive(PartialEq, PartialOrd)]
pub struct EquipmentId {
    item_id: Id,
}

impl EquipmentId {
    pub fn builder() -> EquipmentIdBuilder {
        EquipmentIdBuilder::new()
    }

    pub fn new(seg_delimiter: &str, id: &str) -> Result<Self, IdError> {
        let item_id = Id::new(EQUIPMENT_ID_PREFIX, seg_delimiter, id)?;

        Ok(Self { item_id })
    }
}

impl Display for EquipmentId {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.item_id.fmt(f)
    }
}

impl Default for EquipmentId {
    fn default() -> Self {
        let id = format!("{}001", ID_SEGMENT_DELIMITER_DEFAULT);
        // we carefully provide parameters secured at compile time
        // to this call to ensure that it will not fail, unless
        // something is fundamentally broken in which case we deliberatly panic
        let item_id = Id::new(
            EQUIPMENT_ID_PREFIX,
            ID_SEGMENT_DELIMITER_DEFAULT,
            id.as_str(),
        )
        .unwrap();

        Self { item_id }
    }
}

impl Debug for EquipmentId {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.item_id.fmt(f)
    }
}

pub struct EquipmentIdBuilder {
    builder: IdBuilder,
}

impl EquipmentIdBuilder {
    pub fn new() -> Self {
        Self {
            builder: IdBuilder::new(EQUIPMENT_ID_PREFIX, ID_SEGMENT_DELIMITER_DEFAULT),
        }
    }
}

impl Builder for EquipmentIdBuilder {
    type IdType = EquipmentId;

    fn id(&mut self, id: &str) -> Result<(), IdError> {
        self.builder.id(id)
    }

    /// Adds a segment to the id after validating the new segment follows the rules.
    fn add_segment(&mut self, segment: &str) -> Result<(), IdError> {
        self.builder.add_segment(segment)
    }

    fn build(self) -> Self::IdType {
        EquipmentId {
            item_id: self.builder.build(),
        }
    }
}
