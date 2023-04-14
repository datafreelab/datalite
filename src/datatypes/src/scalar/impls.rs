use rust_decimal::Decimal;

use crate::errors::{DataTypeError, TypeMismatchSnafu};
use crate::macros::{for_all_primitive_variants, for_all_variants};
use crate::scalar::*;
use crate::types::TypeId;

macro_rules! impl_scalar_type_id {
    ([], $( { $Abc:ident, $abc:ident, $AbcArray:ty, $AbcArrayBuilder:ty, $Owned:ty, $Ref:ty } ),*) => {
        impl ScalarImpl {
            pub fn type_id(&self) -> TypeId {
                match self {
                    $(
                        Self::$Abc(_) => TypeId::$Abc,
                    )*
                }
            }
        }
    }
}

for_all_variants! { impl_scalar_type_id }

macro_rules! impl_scalar_ref_type_id {
    ([], $( { $Abc:ident, $abc:ident, $AbcArray:ty, $AbcArrayBuilder:ty, $Owned:ty, $Ref:ty } ),*) => {
        impl <'a> ScalarRefImpl<'a> {
            pub fn type_id(&self) -> TypeId {
                match self {
                    $(
                        Self::$Abc(_) => TypeId::$Abc,
                    )*
                }
            }
        }
    }
}

for_all_variants! { impl_scalar_ref_type_id }

/// Implements `TryFrom` and `From` for [`Scalar`] and [`ScalarRef`].
macro_rules! impl_scalar_conversion {
    ([], $({ $Abc:ident, $abc:ident, $AbcArray:ty, $AbcArrayBuilder:ty, $Owned:ty, $Ref:ty }),*) => {
        $(
            impl<'a> TryFrom<ScalarImpl> for $Owned {
                type Error = DataTypeError;
                fn try_from(that: ScalarImpl) -> Result<Self, Self::Error> {
                    match that {
                        ScalarImpl::$Abc(v) => Ok(v),
                        other => TypeMismatchSnafu { expect:TypeId::$Abc, get:other.type_id()}.fail()
                    }
                }
            }

            impl From<$Owned> for ScalarImpl {
                fn from(that: $Owned) -> Self {
                    ScalarImpl::$Abc(that)
                }
            }

            impl<'a> TryFrom<ScalarRefImpl<'a>> for $Ref {
                type Error = DataTypeError;
                fn try_from(that: ScalarRefImpl<'a>) -> Result<Self, Self::Error> {
                    match that {
                        ScalarRefImpl::$Abc(v) => Ok(v),
                        other => TypeMismatchSnafu { expect:TypeId::$Abc, get:other.type_id()}.fail()
                    }
                }
            }

            impl<'a> From<$Ref> for ScalarRefImpl<'a> {
                fn from(that: $Ref) -> Self {
                    ScalarRefImpl::$Abc(that)
                }
            }
        )*
    };
}

for_all_variants! { impl_scalar_conversion }

macro_rules! impl_scalar_primitive {
    ([], $( { $Abc:ident, $abc:ident, $AbcArray:ty, $AbcArrayBuilder:ty, $Owned:ty, $Ref:ty } ),*) => {
        $(
            impl Scalar for $Owned {
                type ArrayType = $AbcArray;
                type RefType<'a> = $Owned;

                fn as_scalar_ref(&self) -> $Owned {
                    *self
                }

                fn upcast_gat<'short, 'long: 'short>(long: $Owned) -> $Owned {
                    long
                }
            }

            impl<'a> ScalarRef<'a> for $Owned {
                type ArrayType = $AbcArray;
                type ScalarType = $Owned;

                fn to_owned_scalar(&self) -> $Owned {
                    *self
                }
            }
        )*
    }
}

for_all_primitive_variants! { impl_scalar_primitive }

fn debug_array<A: Array>(f: &mut std::fmt::Formatter<'_>, array: &A) -> std::fmt::Result {
    f.debug_list().entries(array.iter()).finish()
}

/// Implements Debug for [`Array`]
macro_rules! impl_array_debug {
    (
        [], $({ $Abc:ident, $abc:ident, $AbcArray:ty, $AbcArrayBuilder:ty, $Owned:ty, $Ref:ty }),*
    ) => {
        $(
            impl std::fmt::Debug for $AbcArray {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    debug_array(f, self)
                }
            }
        )*
    };
}

for_all_variants! { impl_array_debug }