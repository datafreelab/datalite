use snafu::{Backtrace, Snafu};

#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum Error {
    #[snafu(display("Failed to serialize data, source: {}", source))]
    Serialize {
        source: serde_json::Error,
        backtrace: Backtrace,
    },

    #[snafu(display("Failed to deserialize data, source: {}, json: {}", source, json))]
    Deserialize {
        source: serde_json::Error,
        backtrace: Backtrace,
        json: String,
    },

    #[snafu(display("Bad array access, Index out of bounds: {}, size: {}", index, size))]
    BadArrayAccess {
        index: usize,
        size: usize,
        backtrace: Backtrace,
    },

    #[snafu(display("Unknown vector, {}", msg))]
    UnknownVector { msg: String, backtrace: Backtrace },

    #[snafu(display(
    "Failed to parse version in schema meta, value: {}, source: {}",
    value,
    source
    ))]
    ParseDataType {
        value: String,
        source: std::num::ParseIntError,
        backtrace: Backtrace,
    },

    #[snafu(display("{}", msg))]
    CastType { msg: String, backtrace: Backtrace },

    #[snafu(display("Failed to convert value into scalar value, reason: {}", reason))]
    ToScalarValue {
        reason: String,
        backtrace: Backtrace,
    },
}

impl ErrorExt for Error {
    fn status_code(&self) -> StatusCode {
        // Inner encoding and decoding error should not be exposed to users.
        StatusCode::Internal
    }

    fn backtrace_opt(&self) -> Option<&Backtrace> {
        ErrorCompat::backtrace(self)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub type Result<T> = std::result::Result<T, Error>;
