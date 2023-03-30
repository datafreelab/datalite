use enum_as_inner::EnumAsInner;
use parse_display::{Display, FromStr, ParserError};
use serde::{Deserialize, Serialize};

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
    Number(#[from_str(regex = ".*")]DataTypeNumber),
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
    #[display("list<{datatype}>")]
    List { datatype: Box<DataType> },

}

impl std::str::FromStr for Box<DataType> {
    type Err = ParserError;

    fn from_str(s: &str) -> Result<Self, ()> {
        Ok(Box::new(DataType::from_str(s)?))
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
    }
}
