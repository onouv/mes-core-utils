mod tests;

use std::fmt;
use std::fmt::{Debug, Display, Formatter};

use super::plant_item_id_error::*;
use crate::plant_item_id::{ID_GROUP_LEN_DEFAULT, ID_SEGMENT_DELIMITER_DEFAULT, PlantItemId};

pub const EQUIPMENT_ID_PREFIX: char = '-';

#[derive(PartialEq, PartialOrd)]
pub struct EquipmentId {
    item_id: PlantItemId,
}

impl EquipmentId {
    pub fn new(seg_delimiter: char, group_len: usize, id: &str) -> Result<Self, PlantItemIdError> {
        let item_id = PlantItemId::new(EQUIPMENT_ID_PREFIX, seg_delimiter, group_len, id)?;

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
        let item_id = PlantItemId::new(
            EQUIPMENT_ID_PREFIX,
            ID_SEGMENT_DELIMITER_DEFAULT,
            ID_GROUP_LEN_DEFAULT,
            "-001",
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
