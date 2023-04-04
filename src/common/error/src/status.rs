use std::fmt;
use std::fmt::{Debug, Display, Formatter};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Status(i32);

impl Status {
    #[inline]
    pub fn code(&self) -> i32 {
        self.0
    }

    pub fn name(&self) -> &'static str {
        name_opt(self.0).unwrap_or("<unknown status>")
    }

    #[inline]
    pub fn is_success(&self) -> bool {
        self.0 == 0
    }

    pub fn is_retryable(&self) -> bool {
        retryable_opt(self.0)
    }
}

impl Default for Status {
    #[inline]
    fn default() -> Status {
        Status::OK
    }
}

impl Debug for Status {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Status:{{{:x},{}}}", self.0, self.name())?;
        Ok(())
    }
}

impl Display for Status {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Debug::fmt(self, f)
    }
}

impl From<i32> for Status {
    #[inline]
    fn from(code: i32) -> Status {
        Status(code)
    }
}
macro_rules! status {
    (
        $(
            $(#[$docs:meta])*
            ($num:expr, $konst:ident, $phrase:expr, $retryable:expr);
        )+
    ) => {
        impl Status {
        $(
            $(#[$docs])*
            pub const $konst: Status = Status($num);
        )+

        }

        fn name_opt(num: i32) -> Option<&'static str> {
            match num {
                $(
                $num => Some($phrase),
                )+
                _ => None
            }
        }
        fn retryable_opt(num:i32) ->bool{
            match num {
                $(
                $num => $retryable,
                )+
                _ => false
            }
        }
    }
}

status! {
    (0x0000_0000, OK, "OK", true);
    (0x0000_0001, ERR_UNKNOWN, "ErrUnknown",false);
    (0x0001_0001, ERR_PARSE_DATATYPE, "ErrParseDataType",false);
    }


#[cfg(test)]
mod test {
    use crate::status::Status;

    #[test]
    fn test_status_code() {
        assert_eq!(Status::OK.code(), 0);
        assert_eq!(Status::OK.name(), "OK");
        assert_eq!(Status::ERR_UNKNOWN.code(), 0x0000_0001);
        assert_eq!(Status::ERR_UNKNOWN.is_retryable(), false);
        assert_eq!(Status::ERR_PARSE_DATATYPE.code(), 0x0001_0001);
    }
}