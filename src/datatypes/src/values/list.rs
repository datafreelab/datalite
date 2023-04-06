use crate::types::DataType;
use crate::values::ScalarValue;

#[derive(Debug, Clone)]
pub struct ListValue{
    items: Option<Box<Vec<ScalarValue>>>,
    itemType: DataType,
}