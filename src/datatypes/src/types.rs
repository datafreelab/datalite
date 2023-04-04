use std::str::FromStr;

use enum_as_inner::EnumAsInner;
use parse_display::{Display, FromStr, ParseError};
use serde::{Deserialize, Serialize};
use snafu::ResultExt;

use crate::errors::{DataTypeError, InvalidMapKeySnafu, ParseDataTypeSnafu};

use self::type_numbers::*;

mod type_numbers;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, EnumAsInner, FromStr, Display)]
pub enum DataType {
    #[display("null")]
    #[from_str(regex = "(?i)^null$")]
    Null,
    #[display("bool")]
    #[from_str(regex = "(?i)^bool$|^boolean$")]
    Boolean,
    #[display("char({width})")]
    #[from_str(regex = r#"(?i)^char\((?P<width>\d+)\)$"#)]
    Char { width: u16 },
    #[display("varchar")]
    #[from_str(regex = "(?i)^varchar$|^string$")]
    Varchar,
    #[display("{0}")]
    Number(#[from_str(regex = r#"(?i)^[a-z][a-z\d\s]*"#)]DataTypeNumber),
    #[display("decimal({scale},{precision})")]
    #[from_str(regex = r#"(?i)^decimal\((?P<scale>\d+),\s*(?P<precision>\d+)\)$"#)]
    Decimal { scale: u16, precision: u16 },
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
    #[display("list<{item}>")]
    #[from_str(regex = r#"(?i)^list<(?P<item>[a-z][a-z\d\s]*)>$"#)]
    List { item: Box<DataType> },
    #[display("map<{key}:{val}>")]
    #[from_str(regex = r#"(?i)^map<(?P<key>[a-z][a-z\d\s]*),\s*(?P<val>[a-z][a-z\d\s]*)>$"#)]
    Map { key: Box<DataType>, val: Box<DataType> },

}

impl TryFrom<&str> for DataType {
    type Error = DataTypeError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match DataType::from_str(s) {
            Ok(t) => {
                if let DataType::Map { key, .. } = &t {
                    match **key {
                        DataType::Map { .. } | DataType::List { .. } | DataType::Jsonb | DataType::Null => {
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

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::types::DataType;
    use crate::types::type_numbers::DataTypeNumber;

    #[test]
    fn test_data_type_from_str() {
        assert_eq!(DataType::from_str("char(5)").unwrap(), DataType::Char { width: 5 });
        assert_eq!(DataType::from_str("decimal(5,10)").unwrap(), DataType::Decimal { scale: 5, precision: 10 });
        assert_eq!(DataType::from_str("tinyint unsigned").unwrap(), DataType::Number(DataTypeNumber::UInt8));
        assert_eq!(DataType::try_from("list<tinyint>").unwrap(), DataType::List { item: Box::new(DataType::Number(DataTypeNumber::Int8)) });
        assert_eq!(DataType::from_str("list<tinyint unsigned>").unwrap(), DataType::List { item: Box::new(DataType::Number(DataTypeNumber::UInt8)) });
        assert_eq!(DataType::from_str("list<int4>").unwrap(), DataType::List { item: Box::new(DataType::Number(DataTypeNumber::Int32)) });
        assert_eq!(DataType::from_str("list<int4 unsigned>").unwrap(), DataType::List { item: Box::new(DataType::Number(DataTypeNumber::UInt32)) });
        assert_eq!(DataType::from_str("map<int4 unsigned, varchar>").unwrap(), DataType::Map { key: Box::new(DataType::Number(DataTypeNumber::UInt32)), val: Box::new(DataType::Varchar) });
        println!("@@@@@@{}", DataType::from_str("map<int4 unsigned, varchar>").unwrap());
        println!("@@@@@@{:?}", DataType::from_str("tinyint unsigned1").err().unwrap());
        println!("@@@@@@{}", DataType::try_from("tinyint unsigned1").err().unwrap());
        assert_eq!(0, 1)
    }
}
