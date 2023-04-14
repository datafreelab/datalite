use parse_display::Display;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Deserialize, Serialize)]
pub struct Date(i32);

#[derive(Debug, Clone, Default, Copy, Serialize, Deserialize, Display)]
#[display("Timestamp({value} {unit})")]
pub struct Timestamp {
    value: i64,
    unit: TimeUnit,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Display)]
pub enum TimeUnit {
    #[display("Second")]
    Second,
    #[default]
    #[display("Millisecond")]
    Millisecond,
    #[display("Microsecond")]
    Microsecond,
    #[display("Nanosecond")]
    Nanosecond,
}