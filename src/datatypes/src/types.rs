use std::str::FromStr;

use enum_as_inner::EnumAsInner;
use parse_display::{Display, FromStr, ParseError};
use serde::{Deserialize, Serialize};

use crate::errors::{DataTypeError, InvalidMapKeySnafu, ParseDataTypeSnafu};


#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, EnumAsInner, FromStr, Display)]
pub enum DataType {
    #[display("null")]
    #[from_str(regex = "(?i)^null$")]
    Null,
    #[display("nothing")]
    #[from_str(regex = r#"(?i)^nothing$"#)]
    Nothing,
    #[display("bool")]
    #[from_str(regex = "(?i)^bool$|^boolean$")]
    Bool,
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
    #[display("decimal({scale},{precision})")]
    #[from_str(regex = r#"(?i)^decimal\((?P<scale>\d+),\s*(?P<precision>\d+)\)$"#)]
    Decimal { scale: u16, precision: u16 },

    #[display("char({width})")]
    #[from_str(regex = r#"(?i)^char\((?P<width>\d+)\)$"#)]
    Char { width: u16 },
    #[display("string")]
    #[from_str(regex = "(?i)^string$")]
    String,
    #[display("date")]
    #[from_str(regex = "(?i)^date$")]
    Date,
    #[display("timestamp")]
    #[from_str(regex = "(?i)^timestamp$")]
    Timestamp,
    #[display("bytea")]
    #[from_str(regex = "(?i)^bytea$")]
    Bytea,
    #[display("jsonb")]
    #[from_str(regex = "(?i)^jsonb$")]
    Jsonb,
    #[display("list<{0}>")]
    #[from_str(regex = r#"(?i)^list<(?P<0>[a-z][a-z\d\s<>]*)>$"#)]
    List(Box<DataType>),
    #[display("map<{0}:{1}>")]
    #[from_str(regex = r#"(?i)^map<(?P<0>[a-z][a-z\d\s<>]*),\s*(?P<1>[a-z][a-z\d\s<>]*)>$"#)]
    Map(Box<DataType>, Box<DataType>),
    #[display("nullable<{0}>")]
    #[from_str(regex = r#"(?i)^nullable<(?P<0>[a-z][a-z\d\s<>]*)>$"#)]
    Nullable(Box<DataType>),

}

impl TryFrom<&str> for DataType {
    type Error = DataTypeError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match DataType::from_str(s) {
            Ok(t) => {
                if let DataType::Map(key, ..) = &t {
                    match **key {
                        DataType::Map(..) | DataType::List(..) | DataType::Jsonb | DataType::Null => {
                            return InvalidMapKeySnafu { from: s.to_string() }.fail();
                        }
                        _ => {}
                    }
                }
                Ok(t)
            }
            Err(_) => ParseDataTypeSnafu { from: s.to_string() }.fail()
        }
    }
}

impl std::str::FromStr for Box<DataType> {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        DataType::from_str(s).map(|x| Box::new(x))
    }
}

#[derive(Clone, Debug, Copy, Eq, PartialEq)]
pub enum TypeId {
    Bool,
    Int8,
    Int16,
    Int32,
    Int64,
    UInt8,
    UInt16,
    UInt32,
    UInt64,
    Float32,
    Float64,
    Decimal,
    Date,
    List,
}

pub trait TypeIdOf {
    fn type_id(&self) -> TypeId;
}


#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::types::DataType;
    use crate::types::numbers::DataType;

    #[test]
    fn test_data_type_from_number() {
        assert_eq!(DataType::from_str("int1").unwrap(), DataType::Int8);
        assert_eq!(DataType::from_str("tinyint").unwrap(), DataType::Int8);
        assert_eq!(DataType::from_str("INT1").unwrap(), DataType::Int8);
        assert_eq!(DataType::from_str("TINYINT").unwrap(), DataType::Int8);

        assert_eq!(DataType::from_str("int1 unsigned").unwrap(), DataType::UInt8);
        assert_eq!(DataType::from_str("tinyint unsigned").unwrap(), DataType::UInt8);
        assert_eq!(DataType::from_str("INT1 UNSIGNED").unwrap(), DataType::UInt8);
        assert_eq!(DataType::from_str("TINYINT UNSIGNED").unwrap(), DataType::UInt8);

        assert_eq!(DataType::from_str("int2").unwrap(), DataType::Int16);
        assert_eq!(DataType::from_str("smallint").unwrap(), DataType::Int16);
        assert_eq!(DataType::from_str("INT2").unwrap(), DataType::Int16);
        assert_eq!(DataType::from_str("SMALLINT").unwrap(), DataType::Int16);

        assert_eq!(DataType::from_str("int2 unsigned").unwrap(), DataType::UInt16);
        assert_eq!(DataType::from_str("smallint unsigned").unwrap(), DataType::UInt16);
        assert_eq!(DataType::from_str("INT2 UNSIGNED").unwrap(), DataType::UInt16);
        assert_eq!(DataType::from_str("SMALLINT UNSIGNED").unwrap(), DataType::UInt16);

        assert_eq!(DataType::from_str("integer").unwrap(), DataType::Int32);
        assert_eq!(DataType::from_str("int4").unwrap(), DataType::Int32);
        assert_eq!(DataType::from_str("int").unwrap(), DataType::Int32);
        assert_eq!(DataType::from_str("INT4").unwrap(), DataType::Int32);
        assert_eq!(DataType::from_str("INTEGER").unwrap(), DataType::Int32);
        assert_eq!(DataType::from_str("INT").unwrap(), DataType::Int32);

        assert_eq!(DataType::from_str("integer unsigned").unwrap(), DataType::UInt32);
        assert_eq!(DataType::from_str("int4 unsigned").unwrap(), DataType::UInt32);
        assert_eq!(DataType::from_str("int unsigned").unwrap(), DataType::UInt32);
        assert_eq!(DataType::from_str("INT4 UNSIGNED").unwrap(), DataType::UInt32);
        assert_eq!(DataType::from_str("INTEGER UNSIGNED").unwrap(), DataType::UInt32);
        assert_eq!(DataType::from_str("INT UNSIGNED").unwrap(), DataType::UInt32);

        assert_eq!(DataType::from_str("int8").unwrap(), DataType::Int64);
        assert_eq!(DataType::from_str("bigint").unwrap(), DataType::Int64);
        assert_eq!(DataType::from_str("INT8").unwrap(), DataType::Int64);
        assert_eq!(DataType::from_str("BIGINT").unwrap(), DataType::Int64);

        assert_eq!(DataType::from_str("int8 unsigned").unwrap(), DataType::UInt64);
        assert_eq!(DataType::from_str("bigint unsigned").unwrap(), DataType::UInt64);
        assert_eq!(DataType::from_str("INT8 UNSIGNED").unwrap(), DataType::UInt64);
        assert_eq!(DataType::from_str("BIGINT UNSIGNED").unwrap(), DataType::UInt64);

        assert_eq!(DataType::from_str("float4").unwrap(), DataType::Float32);
        assert_eq!(DataType::from_str("float").unwrap(), DataType::Float32);
        assert_eq!(DataType::from_str("FLOAT4").unwrap(), DataType::Float32);
        assert_eq!(DataType::from_str("FLOAT").unwrap(), DataType::Float32);

        assert_eq!(DataType::from_str("float8").unwrap(), DataType::Float64);
        assert_eq!(DataType::from_str("double").unwrap(), DataType::Float64);
        assert_eq!(DataType::from_str("FLOAT8").unwrap(), DataType::Float64);
        assert_eq!(DataType::from_str("DOUBLE").unwrap(), DataType::Float64);
    }

    #[test]
    fn test_data_type_from_str() {
        assert_eq!(DataType::from_str("char(5)").unwrap(), DataType::Char { width: 5 });
        assert_eq!(DataType::from_str("decimal(5,10)").unwrap(), DataType::Decimal { scale: 5, precision: 10 });
        assert_eq!(DataType::from_str("tinyint unsigned").unwrap(), DataType::Number(DataType::UInt8));
        assert_eq!(DataType::try_from("list<tinyint>").unwrap(), DataType::List(Box::new(DataType::Number(DataType::Int8))));
        assert_eq!(DataType::from_str("list<tinyint unsigned>").unwrap(), DataType::List(Box::new(DataType::Number(DataType::UInt8))));
        assert_eq!(DataType::from_str("list<int4>").unwrap(), DataType::List(Box::new(DataType::Number(DataType::Int32))));
        assert_eq!(DataType::from_str("list<int4 unsigned>").unwrap(), DataType::List(Box::new(DataType::Number(DataType::UInt32))));
        assert_eq!(DataType::from_str("map<int4 unsigned, string>").unwrap(), DataType::Map(Box::new(DataType::Number(DataType::UInt32)), Box::new(DataType::String)));
        // DataType::try_from("map<List<int4 unsigned>, varchar>").err().unwrap();
        assert_eq!(DataType::from_str("map<List<int4 unsigned>, string>").unwrap(), DataType::Map(Box::new(DataType::List(Box::new(DataType::Number(DataType::UInt32)))), Box::new(DataType::String)));
        // println!("@@@@@@{}", DataType::from_str("map<int4 unsigned, varchar>").unwrap());
        // println!("@@@@@@{}", DataType::from_str("list<int4 unsigned>").unwrap());
        // println!("@@@@@@{:?}", DataType::from_str("tinyint unsigned1").err().unwrap());
        // println!("@@@@@@{}", DataType::try_from("tinyint unsigned1").err().unwrap());
        assert_eq!(0, 1);
    }
}
