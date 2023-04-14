pub use dyn_array::*;
pub use list_array::*;
pub use macros::*;
pub use primitive_array::*;
pub use primitives::*;
use crate::array::iterator::ArrayIterator;

use crate::errors::DataTypeError;
use crate::scalar::{Scalar, ScalarRef};

mod primitive_array;
mod list_array;
mod dyn_array;
mod primitives;
mod impls;
mod iterator;

mod all_arrays {
    pub use super::{
        BoolArray, DateArray, DecimalArray, F32Array,
        F64Array, I16Array, I32Array, I64Array,
        I8Array, U16Array, U32Array, ListArray,
        U64Array, U8Array,
    };
}

mod all_array_builders {
    pub use super::{
        BoolArrayBuilder, DateArrayBuilder, DecimalArrayBuilder, F32ArrayBuilder,
        F64ArrayBuilder, I16ArrayBuilder, I32ArrayBuilder, I64ArrayBuilder,
        I8ArrayBuilder, U16ArrayBuilder, U32ArrayBuilder,
        U64ArrayBuilder, U8ArrayBuilder,ListArrayBuilder,
    };
}

pub trait Array: Send + Sync + Sized + 'static + TryFrom<ArrayImpl, Error=DataTypeError> + Into<ArrayImpl> + std::fmt::Debug + Clone
    where for<'a> Self::OwnedItem: Scalar<RefType<'a>=Self::RefItem<'a>>,
{
    type Builder: ArrayBuilder<Array=Self>;

    type OwnedItem: Scalar<ArrayType=Self>;

    type RefItem<'a>: ScalarRef<'a, ScalarType=Self::OwnedItem, ArrayType=Self>;

    fn get(&self, idx: usize) -> Option<Self::RefItem<'_>>;

    fn len(&self) -> usize;

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn iter(&self) -> ArrayIterator<Self>;

    fn from_slice(data: &[Self::RefItem<'_>]) -> Self {
        let mut builder = Self::Builder::with_capacity(data.len());
        for item in data {
            builder.push(*item);
        }
        builder.finish()
    }
}

pub trait ArrayBuilder {
    type Array: Array<Builder=Self>;

    fn with_capacity(capacity: usize) -> Self;

    fn push(&mut self, value: <Self::Array as Array>::RefItem<'_>);

    fn finish(self) -> Self::Array;
}


#[derive(Clone, Debug)]
pub enum ArrayImpl {
    I8(I8Array),
    I16(I16Array),
    I32(I32Array),
    I64(I64Array),

    U8(U8Array),
    U16(U16Array),
    U32(U32Array),
    U64(U64Array),

    F32(F32Array),
    F64(F64Array),

    Bool(BoolArray),
    Decimal(DecimalArray),

    Date(DateArray),
    List(ListArray),
}

#[derive(Clone, Debug)]
pub enum ArrayImplRef<'a> {
    I8(&'a I16Array),
    I16(&'a I16Array),
    I32(&'a I32Array),
    I64(&'a I64Array),

    U8(&'a U8Array),
    U16(&'a U16Array),
    U32(&'a U32Array),
    U64(&'a U64Array),

    F32(&'a F32Array),
    F64(&'a F64Array),

    Bool(&'a BoolArray),
    Decimal(&'a DecimalArray),

    Date(&'a DateArray),
    List(&'a ListArray),
}

#[derive(Debug)]
pub struct BoxedArray(Box<dyn dyn_array::DynArray>);

pub enum ArrayBuilderImpl {
    I8(I8ArrayBuilder),
    I16(I16ArrayBuilder),
    I32(I32ArrayBuilder),
    I64(I64ArrayBuilder),

    U8(U8ArrayBuilder),
    U16(U16ArrayBuilder),
    U32(U32ArrayBuilder),
    U64(U64ArrayBuilder),

    F32(F32ArrayBuilder),
    F64(F64ArrayBuilder),
    Bool(BoolArrayBuilder),
    Decimal(DecimalArrayBuilder),
    Date(DateArrayBuilder),
    List(ListArrayBuilder),
}
