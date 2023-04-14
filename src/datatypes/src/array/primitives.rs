use rust_decimal::Decimal;


use crate::array::*;
use crate::{F32, F64};
use crate::scalar::times::Date;

pub trait PrimitiveType: Scalar + Default {}

impl PrimitiveType for i8 {}

impl PrimitiveType for i16 {}

impl PrimitiveType for i32 {}

impl PrimitiveType for i64 {}

impl PrimitiveType for u8 {}

impl PrimitiveType for u16 {}

impl PrimitiveType for u32 {}

impl PrimitiveType for u64 {}

impl PrimitiveType for F32 {}

impl PrimitiveType for F64 {}

impl PrimitiveType for Decimal {}

impl PrimitiveType for Date {}

impl PrimitiveType for bool {}
