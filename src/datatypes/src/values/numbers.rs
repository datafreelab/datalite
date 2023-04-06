use enum_as_inner::EnumAsInner;
use serde::{Deserialize, Serialize};
use crate::{F32, F64};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, EnumAsInner)]
pub enum NumberValue {
    UInt8(u8),
    UInt16(u16),
    UInt32(u32),
    UInt64(u64),
    Int8(i8),
    Int16(i16),
    Int32(i32),
    Int64(i64),
    Float32(F32),
    Float64(F64),
}