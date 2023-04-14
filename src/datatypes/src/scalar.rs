pub mod bytes;
pub mod jsonb;
pub mod list;
pub mod map;
pub mod times;
mod impls;

use rust_decimal::Decimal;

use crate::{F32, F64};
use crate::array::*;
use crate::scalar::bytes::{Bytes, StringBytes};
use crate::scalar::jsonb::{JsonbValue, JsonbValueRef};
use crate::scalar::list::{ListValue, ListValueRef};
use crate::scalar::map::{MapValue, MapValueRef};

use crate::scalar::times::{Date, Timestamp};

pub trait Scalar: std::fmt::Debug + Clone + Send + Sync + 'static + TryFrom<ScalarImpl> + Into<ScalarImpl>
    where
            for<'a> Self::ArrayType: Array<RefItem<'a>=Self::RefType<'a>>,
{
    type ArrayType: Array<OwnedItem=Self>;

    /// The corresponding [`ScalarRef`] type.
    type RefType<'a>: ScalarRef<'a, ScalarType=Self, ArrayType=Self::ArrayType>;

    /// Get a reference of the current value.
    fn as_scalar_ref(&self) -> Self::RefType<'_>;

    /// Upcast GAT type's lifetime.
    fn upcast_gat<'short, 'long: 'short>(long: Self::RefType<'long>) -> Self::RefType<'short>;
}

pub trait ScalarRef<'a>: std::fmt::Debug + Clone + Copy + Send + 'a + TryFrom<ScalarRefImpl<'a>> + Into<ScalarRefImpl<'a>>
{
    /// The corresponding [`Array`] type.
    type ArrayType: Array<RefItem<'a>=Self>;

    /// The corresponding [`Scalar`] type.
    type ScalarType: Scalar<RefType<'a>=Self>;

    /// Convert the reference into an owned value.
    fn to_owned_scalar(&self) -> Self::ScalarType;
}

#[derive(Debug, Clone)]
pub enum ScalarImpl {
    Null,
    Nothing,
    Bool(bool),
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

    Decimal(Decimal),

    Char(StringBytes),
    String(StringBytes),

    Date(Date),
    Timestamp(Timestamp),

    Bytea(Bytes),
    Jsonb(JsonbValue),
    List(ListValue),
    Map(MapValue),
}

#[derive(Debug, Clone, Copy)]
pub enum ScalarRefImpl<'a> {
    Null,
    Bool(bool),

    // Numeric types:
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
    Decimal(Decimal),

    Char(&'a str),
    Varchar(&'a str),
    Bytea(&'a [u8]),
    Jsonb(JsonbValueRef<'a>),

    Date(Date),
    Timestamp(Timestamp),
    List(ListValueRef<'a>),
    Map(MapValueRef<'a>),
}