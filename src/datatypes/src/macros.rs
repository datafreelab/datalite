/// Every tuple has four elements, where
/// `{ TypeId, function suffix name, array type, builder type, scalar owned, scalar ref }`
macro_rules! for_all_variants {
    ($macro:ident $(, $x:ident)*) => {
        $macro! {
            [$($x),*],
            { Int8, int8, I8Array, I8ArrayBuilder, i8, i8 },
            { Int16, int16, I16Array, I16ArrayBuilder, i16, i16 },
            { Int32, int32, I32Array, I32ArrayBuilder, i32, i32 },
            { Int64, int64, I64Array, I64ArrayBuilder, i64, i64 },
            { UInt8, uint8, U8Array, U8ArrayBuilder, u8, u8 },
            { UInt16, uint16, U16Array, U16ArrayBuilder, u16, u16 },
            { UInt32, uint32, U32Array, U32ArrayBuilder, u32, u32 },
            { UInt64, uint64, U64Array, U64ArrayBuilder, u64, u64 },
            { Float32, float32, F32Array, F32ArrayBuilder, F32, F32 },
            { Float64, float64, F64Array, F64ArrayBuilder, F64, F64 },
            { Bool, bool, BoolArray, BoolArrayBuilder, bool, bool },
            { Decimal, decimal, DecimalArray, DecimalArrayBuilder, Decimal, Decimal },
            { Date, date, DateArray, DateArrayBuilder, Date, Date },
            { List, list, ListArray, ListArrayBuilder, ListValue, ListValueRef<'a> }
        }
    };
}

pub(crate) use for_all_variants;

macro_rules! for_all_primitive_variants {
    ($macro:ident $(, $x:ident)*) => {
        $macro! {
            [$($x),*],
            { Int8, int8, I8Array, I8ArrayBuilder, i8, i8 },
            { Int16, int16, I16Array, I16ArrayBuilder, i16, i16 },
            { Int32, int32, I32Array, I32ArrayBuilder, i32, i32 },
            { Int64, int64, I64Array, I64ArrayBuilder, i64, i64 },
            { UInt8, uint8, U8Array, U8ArrayBuilder, u8, u8 },
            { UInt16, uint16, U16Array, U16ArrayBuilder, u16, u16 },
            { UInt32, uint32, U32Array, U32ArrayBuilder, u32, u32 },
            { UInt64, uint64, U64Array, U64ArrayBuilder, u64, u64 },
            { Float32, float32, F32Array, F32ArrayBuilder, F32, F32 },
            { Float64, float64, F64Array, F64ArrayBuilder, F64, F64 },
            { Bool, bool, BoolArray, BoolArrayBuilder, bool, bool },
            { Decimal, decimal, DecimalArray, DecimalArrayBuilder, Decimal, Decimal },
            { Date, date, DateArray, DateArrayBuilder, Date, Date }
        }
    };
}
pub(crate) use for_all_primitive_variants;
