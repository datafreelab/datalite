use enum_as_inner::EnumAsInner;
use ordered_float::OrderedFloat;
use parse_display::{Display, FromStr};
use serde::{Deserialize, Serialize};

pub type F32 = OrderedFloat<f32>;
pub type F64 = OrderedFloat<f64>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, FromStr, Display)]
pub enum DataTypeNumber {
    #[display("tinyint unsigned")]
    #[from_str(regex = "(?i)^tinyint unsigned$|^int1 unsigned$")]
    UInt8,
    #[display("smallint unsigned")]
    #[from_str(regex = "(?i)^smallint unsigned$|^int2 unsigned$")]
    UInt16,
    #[display("int unsigned")]
    #[from_str(regex = "(?i)^integer unsigned$|^int unsigned$|^int4 unsigned$")]
    UInt32,
    #[display("bigint unsigned")]
    #[from_str(regex = "(?i)^bigint unsigned$|^int8 unsigned$")]
    UInt64,
    #[display("tinyint")]
    #[from_str(regex = "(?i)^tinyint$|^int1$")]
    Int8,
    #[display("smallint")]
    #[from_str(regex = "(?i)^smallint$|^int2$")]
    Int16,
    #[display("int")]
    #[from_str(regex = "(?i)^integer$|^int$|^int4$")]
    Int32,
    #[display("bigint")]
    #[from_str(regex = "(?i)^bigint$|^int8$")]
    Int64,
    #[display("float")]
    #[from_str(regex = "(?i)^float$|^float4$")]
    Float32,
    #[display("double")]
    #[from_str(regex = "(?i)^double|^float8$")]
    Float64,
}

#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, EnumAsInner)]
pub enum ScalarNumber {
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

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::types::type_numbers::DataTypeNumber;

    #[test]
    fn test_data_type_from_str() {
        assert_eq!(DataTypeNumber::from_str("int1").unwrap(), DataTypeNumber::Int8);
        assert_eq!(DataTypeNumber::from_str("tinyint").unwrap(), DataTypeNumber::Int8);
        assert_eq!(DataTypeNumber::from_str("INT1").unwrap(), DataTypeNumber::Int8);
        assert_eq!(DataTypeNumber::from_str("TINYINT").unwrap(), DataTypeNumber::Int8);

        assert_eq!(DataTypeNumber::from_str("int1 unsigned").unwrap(), DataTypeNumber::UInt8);
        assert_eq!(DataTypeNumber::from_str("tinyint unsigned").unwrap(), DataTypeNumber::UInt8);
        assert_eq!(DataTypeNumber::from_str("INT1 UNSIGNED").unwrap(), DataTypeNumber::UInt8);
        assert_eq!(DataTypeNumber::from_str("TINYINT UNSIGNED").unwrap(), DataTypeNumber::UInt8);

        assert_eq!(DataTypeNumber::from_str("int2").unwrap(), DataTypeNumber::Int16);
        assert_eq!(DataTypeNumber::from_str("smallint").unwrap(), DataTypeNumber::Int16);
        assert_eq!(DataTypeNumber::from_str("INT2").unwrap(), DataTypeNumber::Int16);
        assert_eq!(DataTypeNumber::from_str("SMALLINT").unwrap(), DataTypeNumber::Int16);

        assert_eq!(DataTypeNumber::from_str("int2 unsigned").unwrap(), DataTypeNumber::UInt16);
        assert_eq!(DataTypeNumber::from_str("smallint unsigned").unwrap(), DataTypeNumber::UInt16);
        assert_eq!(DataTypeNumber::from_str("INT2 UNSIGNED").unwrap(), DataTypeNumber::UInt16);
        assert_eq!(DataTypeNumber::from_str("SMALLINT UNSIGNED").unwrap(), DataTypeNumber::UInt16);

        assert_eq!(DataTypeNumber::from_str("integer").unwrap(), DataTypeNumber::Int32);
        assert_eq!(DataTypeNumber::from_str("int4").unwrap(), DataTypeNumber::Int32);
        assert_eq!(DataTypeNumber::from_str("int").unwrap(), DataTypeNumber::Int32);
        assert_eq!(DataTypeNumber::from_str("INT4").unwrap(), DataTypeNumber::Int32);
        assert_eq!(DataTypeNumber::from_str("INTEGER").unwrap(), DataTypeNumber::Int32);
        assert_eq!(DataTypeNumber::from_str("INT").unwrap(), DataTypeNumber::Int32);

        assert_eq!(DataTypeNumber::from_str("integer unsigned").unwrap(), DataTypeNumber::UInt32);
        assert_eq!(DataTypeNumber::from_str("int4 unsigned").unwrap(), DataTypeNumber::UInt32);
        assert_eq!(DataTypeNumber::from_str("int unsigned").unwrap(), DataTypeNumber::UInt32);
        assert_eq!(DataTypeNumber::from_str("INT4 UNSIGNED").unwrap(), DataTypeNumber::UInt32);
        assert_eq!(DataTypeNumber::from_str("INTEGER UNSIGNED").unwrap(), DataTypeNumber::UInt32);
        assert_eq!(DataTypeNumber::from_str("INT UNSIGNED").unwrap(), DataTypeNumber::UInt32);

        assert_eq!(DataTypeNumber::from_str("int8").unwrap(), DataTypeNumber::Int64);
        assert_eq!(DataTypeNumber::from_str("bigint").unwrap(), DataTypeNumber::Int64);
        assert_eq!(DataTypeNumber::from_str("INT8").unwrap(), DataTypeNumber::Int64);
        assert_eq!(DataTypeNumber::from_str("BIGINT").unwrap(), DataTypeNumber::Int64);

        assert_eq!(DataTypeNumber::from_str("int8 unsigned").unwrap(), DataTypeNumber::UInt64);
        assert_eq!(DataTypeNumber::from_str("bigint unsigned").unwrap(), DataTypeNumber::UInt64);
        assert_eq!(DataTypeNumber::from_str("INT8 UNSIGNED").unwrap(), DataTypeNumber::UInt64);
        assert_eq!(DataTypeNumber::from_str("BIGINT UNSIGNED").unwrap(), DataTypeNumber::UInt64);

        assert_eq!(DataTypeNumber::from_str("float4").unwrap(), DataTypeNumber::Float32);
        assert_eq!(DataTypeNumber::from_str("float").unwrap(), DataTypeNumber::Float32);
        assert_eq!(DataTypeNumber::from_str("FLOAT4").unwrap(), DataTypeNumber::Float32);
        assert_eq!(DataTypeNumber::from_str("FLOAT").unwrap(), DataTypeNumber::Float32);

        assert_eq!(DataTypeNumber::from_str("float8").unwrap(), DataTypeNumber::Float64);
        assert_eq!(DataTypeNumber::from_str("double").unwrap(), DataTypeNumber::Float64);
        assert_eq!(DataTypeNumber::from_str("FLOAT8").unwrap(), DataTypeNumber::Float64);
        assert_eq!(DataTypeNumber::from_str("DOUBLE").unwrap(), DataTypeNumber::Float64);
    }
}