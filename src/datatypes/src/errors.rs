use snafu::{Backtrace, prelude::*};

use crate::types::TypeId;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum DataTypeError {
    #[snafu(display("Failed to parse DataType from: '{}'", from))]
    ParseDataType { from: String, backtrace: Backtrace },
    #[snafu(display("Invalid map key from: '{}'", from))]
    InvalidMapKey { from: String, backtrace: Backtrace },
    #[snafu(display("Type mismatch on conversion: expected {expect:?}, get {get:?}"))]
    TypeMismatch { expect: TypeId, get: TypeId, backtrace: Backtrace },
}

pub type Result<T> = std::result::Result<T, DataTypeError>;
