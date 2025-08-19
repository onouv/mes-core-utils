mod tests;

use std::fmt;
use std::fmt::{Debug, Display, Formatter};

use super::id_error::IdError;
use super::{ID_SEGMENT_DELIMITER_DEFAULT, Id};

pub const FUNCTION_ID_PREFIX: char = '=';

#[derive(PartialEq, PartialOrd)]
pub struct FunctionId {
    item_id: Id,
}

impl FunctionId {
    pub fn new(seg_delimiter: char, id: &str) -> Result<Self, IdError> {
        let item_id = Id::new(FUNCTION_ID_PREFIX, seg_delimiter, id)?;

        Ok(Self { item_id })
    }

    pub fn with_defaults(id: &str) -> Result<Self, IdError> {
        let item_id = Id::new(FUNCTION_ID_PREFIX, ID_SEGMENT_DELIMITER_DEFAULT, id)?;

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
