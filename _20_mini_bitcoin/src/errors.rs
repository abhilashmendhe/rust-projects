use thiserror::Error;

use crate::elliptic_curves::Point;

#[derive(Debug, Error)]
pub enum BitcoinErrors {

    #[error("Number {} is not in the field range 0 to {}", .number, .prime)]
    FiniteFieldRangeErr{
        number: isize,
        prime: isize
    },

    #[error("Finite fields have different primes. Cant perform {}", .0)]
    TwoDiffFiniteFields(String),

    #[error("{} is not on curve!", .0)]
    PointsNotOnCurve(Point)
}


