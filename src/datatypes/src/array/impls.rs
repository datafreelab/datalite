use crate::macros::for_all_variants;
use crate::array::all_arrays::*;
use crate::array::all_array_builders::*;
use crate::types::*;

macro_rules! impl_array_type_id {
    (
        [], $({ $Abc:ident, $abc:ident, $AbcArray:ty, $AbcArrayBuilder:ty, $Owned:ty, $Ref:ty }),*
    ) => {
        $(
            impl $AbcArray {
                fn type_id(&self) -> TypeId {
                    TypeId::$Abc
                }
            }

            impl $AbcArrayBuilder {
                fn type_id(&self) -> TypeId {
                    TypeId::$Abc
                }
            }
        )*
    };
}

for_all_variants! { impl_array_type_id }