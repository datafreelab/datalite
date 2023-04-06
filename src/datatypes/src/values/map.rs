use std::iter::Map;
use crate::types::DataType;
use crate::values::ScalarValue;

#[derive(Debug, Clone)]
pub struct MapValue{
    keyType:DataType,
    valType:DataType,
    items: Option<Box<Map<ScalarValue,ScalarValue>>>,
}