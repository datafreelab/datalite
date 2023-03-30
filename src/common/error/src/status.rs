use std::fmt;
use std::fmt::{Debug, Display, Formatter};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StatusCode(i32);

impl StatusCode {
    #[inline]
    pub fn code(&self) -> i32 {
        self.0
    }

    #[allow(unconditional_recursion)]
    pub fn name(&self) -> &'static str {
        match_name(self.0).unwrap_or("<unknown status code>")
    }

    #[inline]
    pub fn is_success(&self) -> bool {
        self.0 == 0
    }

    pub fn is_retryable(&self) -> bool {
        match_retryable(self.0)
    }
}

impl Default for StatusCode {
    #[inline]
    fn default() -> StatusCode {
        StatusCode::OK
    }
}

impl Debug for StatusCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Status:{{{:x},{}}}", self.0, self.name())?;
        Ok(())
    }
}

impl Display for StatusCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Debug::fmt(self, f)
    }
}

impl From<i32> for StatusCode {
    #[inline]
    fn from(code: i32) -> StatusCode {
        StatusCode(code)
    }
}
macro_rules! status_codes {
    (
        $(
            $(#[$docs:meta])*
            ($num:expr, $konst:ident, $phrase:expr, $retryable:expr);
        )+
    ) => {
        impl StatusCode {
        $(
            $(#[$docs])*
            pub const $konst: StatusCode = StatusCode($num);
        )+

        }

        fn match_name(num: i32) -> Option<&'static str> {
            match num {
                $(
                $num => Some($phrase),
                )+
                _ => None
            }
        }
        fn match_retryable(num:i32) ->bool{
            match num {
                $(
                $num => $retryable,
                )+
                _ => false
            }
        }
    }
}

status_codes! {
    (0x0000_0000, OK, "OK", true);
    (0x0000_0001, ERR_UNKNOWN, "ErrUnknown",false);
    (0x0001_0001, ERR_PARSE_DATATYPE, "ErrParseDataType",false);
    }
#[cfg(test)]
mod test {
    use crate::status::StatusCode;

    #[test]
    fn test_status_code() {
        assert_eq!(StatusCode::OK.code(), 0);
        assert_eq!(StatusCode::OK.name(), "OK");
        assert_eq!(StatusCode::ERR_UNKNOWN.code(), 0x0000_0001);
        assert_eq!(StatusCode::ERR_UNKNOWN.is_retryable(), false);
        assert_eq!(StatusCode::ERR_PARSE_DATATYPE.code(), 0x0001_0001);
    }
}