mod tests;

use std::fmt::{self, format};
use std::fmt::{Debug, Display, Formatter};

use super::id_error::IdError;
use super::{ID_SEGMENT_DELIMITER_DEFAULT, Id};

/// The canocnical standard prefix for an equipment id.
pub const EQUIPMENT_ID_PREFIX: char = '-';

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

    pub fn new(seg_delimiter: char, id: &str) -> Result<Self, IdError> {
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
    prefix: char,
    seg_delimiter: char,
    segments: Vec<String>,
}

impl EquipmentIdBuilder {
    pub fn new() -> Self {
        Self {
            prefix: EQUIPMENT_ID_PREFIX,
            seg_delimiter: ID_SEGMENT_DELIMITER_DEFAULT,
            segments: vec![String::from("001")],
        }
    }
    pub fn with_prefix(mut self, prefix: char) -> Self {
        self.prefix = prefix;
        self
    }

    pub fn with_segment_delimiter(mut self, seg_delimiter: char) -> Self {
        self.seg_delimiter = seg_delimiter;
        self
    }

    /// Overwrites the existing id field. Must provide a string beginning with correct prefix and
    /// using correct segment separators to avoid failing
    pub fn id(mut self, id: &str) -> Result<Self, IdError> {
        let segments = Id::validate_id(self.prefix, self.seg_delimiter, id)?;
        self.segments = segments;

        Ok(self)
    }

    /// Adds a segment to the id after validating the new segment follows the rules.
    pub fn added_segment(mut self, segment: &str) -> Result<Self, IdError> {
        Ok(self)
    }

    pub fn build(self) -> Result<EquipmentId, IdError> {
        let item_id = Id::new(self.prefix, self.seg_delimiter, self.id.as_str())?;

        Ok(EquipmentId { item_id })
    }
}
