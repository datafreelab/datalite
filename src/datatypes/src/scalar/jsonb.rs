use serde_json::Value;

#[derive(Debug, Clone)]
pub struct JsonbValue(Box<Value>);

#[derive(Debug, Clone, Copy)]
pub enum JsonbValueRef<'a> {
    Ref { val: &'a JsonbValue },
}