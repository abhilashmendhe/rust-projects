use std::fmt::Display;

use crate::errors::BitcoinErrors;

#[derive(Debug, PartialEq, Eq)]
pub struct Point {
    x: Option<isize>,
    y: Option<isize>,
    a: isize,
    b: isize
}

impl Point {
    pub fn new(x: Option<isize>, y: Option<isize>, a: isize, b: isize) -> Result<Self, BitcoinErrors> {

        let p = Point{x, y, a, b};
        
        if p.x.is_none() && p.y.is_none() {
            return Ok(p);
        }
        
        if p.y.unwrap().pow(2) != p.x.unwrap().pow(3) + a * p.x.unwrap() + b {
            return Err(BitcoinErrors::PointsNotOnCurve(p));
        } 

        Ok(p)
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.x.is_none() && self.y.is_none() {
            write!(f, "Point({:?}, {:?})_{}_{}", self.x, self.y, self.a, self.b)
        } else {
            write!(f, "Point({}, {})_{}_{}", self.x.unwrap(), self.y.unwrap(), self.a, self.b)
        }
    }
}