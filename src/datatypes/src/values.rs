pub mod numbers;
pub mod jsonb;
pub mod list;
pub mod map;
mod times;
mod string;


use bytes::Bytes;
use serde::{Deserialize, Serialize};
use crate::values::jsonb::JsonbValue;
use crate::values::list::ListValue;
use crate::values::map::MapValue;
use crate::values::numbers::NumberValue;
use rust_decimal::Decimal;
use crate::values::string::StringValue;
use crate::values::times::{DateValue, TimestampValue};

#[derive(Debug, Clone)]
pub enum ScalarValue {
    Null,
    Nothing,
    Boolean(bool),

    Char(StringValue),
    Varchar(StringValue),

    Number(NumberValue),
    Decimal(Decimal),

    Date(DateValue),
    Timestamp(TimestampValue),

    Bytea(Bytes),
    Jsonb(JsonbValue),
    List(ListValue),
    Map(MapValue),
}