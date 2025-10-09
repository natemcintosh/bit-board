use std::error::Error;
use std::fmt;

pub mod bitboard;
pub mod bitboarddyn;

#[derive(Debug)]
pub struct DimensionMismatch;

impl fmt::Display for DimensionMismatch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Dimensions do not match.")
    }
}

impl Error for DimensionMismatch {}
