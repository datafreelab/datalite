#![feature(assert_matches)]

use ordered_float::OrderedFloat;

pub mod types;
pub mod errors;
pub mod scalar;
pub mod values;

pub type F32 = OrderedFloat<f32>;
pub type F64 = OrderedFloat<f64>;

