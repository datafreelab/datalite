use rust_decimal::Decimal;

use bitvec::prelude::BitVec;

use crate::{F32, F64};
use crate::array::iterator::ArrayIterator;
use crate::array::primitives::PrimitiveType;
use crate::errors::DataTypeError;
use crate::scalar::{Scalar, ScalarRef};
use crate::scalar::times::Date;
use crate::types::*;

use super::{Array, ArrayBuilder, ArrayImpl};

pub type I8Array = PrimitiveArray<i8>;
pub type I16Array = PrimitiveArray<i16>;
pub type I32Array = PrimitiveArray<i32>;
pub type I64Array = PrimitiveArray<i64>;

pub type U8Array = PrimitiveArray<u8>;
pub type U16Array = PrimitiveArray<u16>;
pub type U32Array = PrimitiveArray<u32>;
pub type U64Array = PrimitiveArray<u64>;

pub type F32Array = PrimitiveArray<F32>;
pub type F64Array = PrimitiveArray<F64>;

pub type BoolArray = PrimitiveArray<bool>;
pub type DecimalArray = PrimitiveArray<Decimal>;

pub type DateArray = PrimitiveArray<Date>;

pub type I8ArrayBuilder = PrimitiveArrayBuilder<i8>;
pub type I16ArrayBuilder = PrimitiveArrayBuilder<i16>;
pub type I32ArrayBuilder = PrimitiveArrayBuilder<i32>;
pub type I64ArrayBuilder = PrimitiveArrayBuilder<i64>;

pub type U8ArrayBuilder = PrimitiveArrayBuilder<u8>;
pub type U16ArrayBuilder = PrimitiveArrayBuilder<u16>;
pub type U32ArrayBuilder = PrimitiveArrayBuilder<u32>;
pub type U64ArrayBuilder = PrimitiveArrayBuilder<u64>;

pub type F32ArrayBuilder = PrimitiveArrayBuilder<F32>;
pub type F64ArrayBuilder = PrimitiveArrayBuilder<F64>;

pub type BoolArrayBuilder = PrimitiveArrayBuilder<bool>;
pub type DecimalArrayBuilder = PrimitiveArrayBuilder<Decimal>;
pub type DateArrayBuilder = PrimitiveArrayBuilder<Date>;


#[derive(Clone)]
pub struct PrimitiveArray<T: PrimitiveType> {
    data: Vec<T>,
}

impl<T> Array for PrimitiveArray<T>
    where
        T: PrimitiveType,
        T: Scalar<ArrayType=Self>,
        for<'a> T: ScalarRef<'a, ScalarType=T, ArrayType=Self>,
        for<'a> T: Scalar<RefType<'a>=T>,
        Self: Into<ArrayImpl>,
        Self: TryFrom<ArrayImpl, Error=DataTypeError>,
        Self: std::fmt::Debug,
{
    type Builder = PrimitiveArrayBuilder<T>;

    type OwnedItem = T;

    /// For `PrimitiveType`, we can always get the value from the array with little overhead.
    /// Therefore, we do not use the `'a` lifetime here, and simply copy the value to the user when
    /// calling `get`.
    type RefItem<'a> = T;

    #[inline]
    fn get(&self, idx: usize) -> Option<T> {
        Some(self.data[idx])
    }

    #[inline]
    fn len(&self) -> usize {
        self.data.len()
    }

    fn iter(&self) -> ArrayIterator<Self> {
        ArrayIterator::new(self)
    }
}

pub struct PrimitiveArrayBuilder<T: PrimitiveType> {
    /// The actual data of this array.
    data: Vec<T>,
}

impl<T> ArrayBuilder for PrimitiveArrayBuilder<T>
    where
        T: PrimitiveType,
        T: Scalar<ArrayType=PrimitiveArray<T>>,
        for<'a> T: ScalarRef<'a, ScalarType=T, ArrayType=PrimitiveArray<T>>,
        for<'a> T: Scalar<RefType<'a>=T>,
        PrimitiveArray<T>: Into<ArrayImpl>,
        PrimitiveArray<T>: TryFrom<ArrayImpl, Error=DataTypeError>,
        PrimitiveArray<T>: std::fmt::Debug,
{
    type Array = PrimitiveArray<T>;

    fn with_capacity(capacity: usize) -> Self {
        Self {
            data: Vec::with_capacity(capacity),
        }
    }

    fn push(&mut self, v: T) {
        self.data.push(v);
    }

    fn finish(self) -> Self::Array {
        PrimitiveArray {
            data: self.data
        }
    }
}
