#![feature(trait_alias)]
use crate::errors::BoxedError;

pub mod errors;
pub mod status;

pub type Result<T> = std::result::Result<T, BoxedError>;
