use std::fmt;

use num_traits::ToPrimitive;
use serde::{Deserialize, Serialize};

/// A JSON number that remembers whether it was written as an integer or a float,
/// so that `6378137` and `6378137.0` both round-trip unchanged.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Number {
    Integer(i64),
    Float(f64),
}

impl From<Number> for f64 {
    fn from(n: Number) -> Self {
        match n {
            Number::Integer(i) => i as f64,
            Number::Float(f) => f,
        }
    }
}

impl ToPrimitive for Number {
    fn to_i64(&self) -> Option<i64> {
        match *self {
            Number::Integer(i) => Some(i),
            Number::Float(f) => f.to_i64(),
        }
    }

    fn to_u64(&self) -> Option<u64> {
        match *self {
            Number::Integer(i) => i.to_u64(),
            Number::Float(f) => f.to_u64(),
        }
    }

    fn to_f64(&self) -> Option<f64> {
        Some(f64::from(*self))
    }
}

impl From<i64> for Number {
    fn from(i: i64) -> Self {
        Number::Integer(i)
    }
}

impl From<f64> for Number {
    fn from(f: f64) -> Self {
        Number::Float(f)
    }
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Number::Integer(i) => write!(f, "{i}"),
            Number::Float(x) => write!(f, "{x}"),
        }
    }
}
