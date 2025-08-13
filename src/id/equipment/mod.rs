mod tests;

use std::fmt;
use std::fmt::{Debug, Display, Formatter};

use super::id_error::IdError;
use super::{ID_SEGMENT_DELIMITER_DEFAULT, Id};

pub const EQUIPMENT_ID_PREFIX: char = '-';

#[derive(PartialEq, PartialOrd)]
pub struct EquipmentId {
    item_id: Id,
}

impl EquipmentId {
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
        // we carefully provide parameters secured at compile time
        // to this call to ensure that it will not fail, unless
        // something is fundamentally broken in which case we deliberatly panic
        let item_id = Id::new(EQUIPMENT_ID_PREFIX, ID_SEGMENT_DELIMITER_DEFAULT, "-001").unwrap();

        Self { item_id }
    }
}

impl Debug for EquipmentId {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.item_id.fmt(f)
    }
}
