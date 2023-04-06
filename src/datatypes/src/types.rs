use std::str::FromStr;

use enum_as_inner::EnumAsInner;
use parse_display::{Display, FromStr, ParseError};
use serde::{Deserialize, Serialize};

use crate::errors::{DataTypeError, InvalidMapKeySnafu, ParseDataTypeSnafu};

use self::numbers::*;

pub mod numbers;

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

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::types::DataType;
    use crate::types::numbers::DataTypeNumber;

    #[test]
    fn test_data_type_from_str() {
        assert_eq!(DataType::from_str("char(5)").unwrap(), DataType::Char { width: 5 });
        assert_eq!(DataType::from_str("decimal(5,10)").unwrap(), DataType::Decimal { scale: 5, precision: 10 });
        assert_eq!(DataType::from_str("tinyint unsigned").unwrap(), DataType::Number(DataTypeNumber::UInt8));
        assert_eq!(DataType::try_from("list<tinyint>").unwrap(), DataType::List(Box::new(DataType::Number(DataTypeNumber::Int8))));
        assert_eq!(DataType::from_str("list<tinyint unsigned>").unwrap(), DataType::List(Box::new(DataType::Number(DataTypeNumber::UInt8))));
        assert_eq!(DataType::from_str("list<int4>").unwrap(), DataType::List(Box::new(DataType::Number(DataTypeNumber::Int32))));
        assert_eq!(DataType::from_str("list<int4 unsigned>").unwrap(), DataType::List(Box::new(DataType::Number(DataTypeNumber::UInt32))));
        assert_eq!(DataType::from_str("map<int4 unsigned, varchar>").unwrap(), DataType::Map(Box::new(DataType::Number(DataTypeNumber::UInt32)), Box::new(DataType::Varchar)));
        // DataType::try_from("map<List<int4 unsigned>, varchar>").err().unwrap();
        assert_eq!(DataType::from_str("map<List<int4 unsigned>, varchar>").unwrap(), DataType::Map(Box::new(DataType::List(Box::new(DataType::Number(DataTypeNumber::UInt32)))), Box::new(DataType::Varchar)));
        // println!("@@@@@@{}", DataType::from_str("map<int4 unsigned, varchar>").unwrap());
        // println!("@@@@@@{}", DataType::from_str("list<int4 unsigned>").unwrap());
        // println!("@@@@@@{:?}", DataType::from_str("tinyint unsigned1").err().unwrap());
        // println!("@@@@@@{}", DataType::try_from("tinyint unsigned1").err().unwrap());
        assert_eq!(0, 1);
    }
}
