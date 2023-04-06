use serde_json::Value;

#[derive(Debug, Clone)]
pub struct JsonbValue(Box<Value>);