use crate::scalar::ScalarImpl;
use crate::types::DataType;

#[derive(Debug, Clone)]
pub struct ListValue {
    items: Option<Box<Vec<ScalarImpl>>>,
    itemType: DataType,
}

#[derive(Debug, Clone, Copy)]
pub enum ListValueRef<'a> {
    Ref { val: &'a ListValue },
}