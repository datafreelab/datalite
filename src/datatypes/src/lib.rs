#![feature(assert_matches)]
#![feature(trusted_len)]

use ordered_float::OrderedFloat;

pub mod types;
pub mod errors;
pub mod scalar;
pub mod array;
pub mod macros;

pub type F32 = OrderedFloat<f32>;
pub type F64 = OrderedFloat<f64>;

