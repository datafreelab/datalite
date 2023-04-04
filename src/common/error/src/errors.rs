use std::any::Any;
use std::backtrace::Backtrace;
use std::fmt;

use crate::status::Status;

pub trait ErrorExt: std::error::Error + Send + Sync + 'static {
    fn status(&self) -> Status {
        Status::ERR_UNKNOWN
    }

    fn backtrace_opt(&self) -> Option<&Backtrace>;

    fn as_any(&self) -> &dyn Any;
}

pub struct BoxedError {
    inner: Box<dyn ErrorExt>,
}


impl BoxedError {
    pub fn new<E: ErrorExt>(err: E) -> Self {
        Self {
            inner: Box::new(err),
        }
    }

    pub fn from_string(msg: String, status: Status) -> Self {
        BoxedError::new(PlainError::message(msg, status))
    }
}

impl fmt::Debug for BoxedError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.", self.inner)?;
        if let Some(backtrace) = self.backtrace_opt() {
            // Add a newline to separate causes and backtrace.
            write!(f, "\nBacktrace:\n{backtrace}")?;
        }
        Ok(())
    }
}

impl fmt::Display for BoxedError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.inner)
    }
}

impl std::error::Error for BoxedError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.inner.source()
    }
}

impl ErrorExt for BoxedError {
    fn status(&self) -> Status {
        self.inner.status()
    }

    fn backtrace_opt(&self) -> Option<&Backtrace> {
        self.inner.backtrace_opt()
    }

    fn as_any(&self) -> &dyn Any {
        self.inner.as_any()
    }
}

pub struct PlainError {
    message: String,
    status: Status,
    backtrace: Option<Backtrace>,
}

impl PlainError {
    pub fn message(message: String, status: Status) -> Self {
        Self { message, status, backtrace: Some(Backtrace::force_capture()) }
    }
}

impl fmt::Display for PlainError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.", self.status)?;
        write!(f, " Caused by: {}", self.message)
    }
}

impl fmt::Debug for PlainError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)?;
        if let Some(backtrace) = self.backtrace_opt() {
            // Add a newline to separate causes and backtrace.
            write!(f, "\nBacktrace:\n{backtrace}")?;
        }
        Ok(())
    }
}

impl std::error::Error for PlainError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

impl ErrorExt for PlainError {
    fn status(&self) -> Status {
        self.status
    }

    fn backtrace_opt(&self) -> Option<&Backtrace> {
        self.backtrace.as_ref()
    }

    fn as_any(&self) -> &dyn Any {
        self as _
    }
}


#[cfg(test)]
mod test {
    use crate::errors::{BoxedError, PlainError};
    use crate::status::Status;

    #[test]
    fn test_debug() {
        let err = PlainError::message("test".to_string(), Status::ERR_UNKNOWN);
        println!("{:?}", err);
        let err = BoxedError::new(err);
        println!("{:?}", err);
    }
}