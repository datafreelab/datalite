use snafu::{Backtrace, prelude::*};

#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum DataTypeError {
    #[snafu(display("Failed to parse DataType from: '{}'", from))]
    ParseDataType { from: String, backtrace: Backtrace },
    #[snafu(display("Invalid map key from: '{}'", from))]
    InvalidMapKey { from: String, backtrace: Backtrace }
}

pub type Result<T> = std::result::Result<T, DataTypeError>;
