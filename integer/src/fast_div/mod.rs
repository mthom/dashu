//! Prepared divisor types for fast division

mod barret;
mod const_div;
pub(crate) use barret::{FastDivideNormalized, FastDivideNormalized2, FastDivideSmall};
pub(crate) use const_div::{ConstSingleDivisor, ConstDoubleDivisor, ConstLargeDivisor, ConstDivisorRepr};
pub use const_div::ConstDivisor;
