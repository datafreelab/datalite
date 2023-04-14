use std::iter::Map;
use crate::scalar::ScalarImpl;

use crate::types::DataType;
#[derive(Debug, Clone)]
pub struct MapValue {
    keyType: DataType,
    valType: DataType,
    items: Option<Box<Map<ScalarImpl, ScalarImpl>>>,
}

#[derive(Debug, Clone, Copy)]
pub enum MapValueRef<'a> {
    Ref { val: &'a MapValue },
}