use std::{fmt::Display, ops::Add};

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
            return Err(BitcoinErrors::PointNotOnCurve(p));
        } 

        Ok(p)
    }
}

impl Add for Point {
    type Output = Result<Self, BitcoinErrors>;
    fn add(self, rhs: Self) -> Result<Self, BitcoinErrors> {
        
        if self.a != rhs.a || self.b != rhs.b {
            return Err(BitcoinErrors::TwoPointsNotOnSameCurve(self, rhs));
        }

        if self.x.is_none() {
            return Ok(rhs);
        } 
        if rhs.x.is_none() {
            return Ok(self);
        }
        if self.x == rhs.x {
            if self.y.is_none() && rhs.y.is_none() {
                return Point::new(None, None, self.a, self.b);
            } else if self.y.is_none() {
                return Ok(rhs)
            } else if rhs.y.is_none() {
                return Ok(self)
            } else {
                if self.y.unwrap() == -1*rhs.y.unwrap() {
                    return Point::new(None, None, self.a, self.b);
                }
            }
        }

        let x1 = self.x.unwrap();
        let y1 = self.y.unwrap();
        let x2 = rhs.x.unwrap();
        let y2 = rhs.y.unwrap();
        let slope = (y2 - y1) / (x2 - x1);
        
        let x3 = slope.pow(2) - x1 - x2;
        let y3 = slope*(x1 - x3) - y1;

        Point::new(Some(x3), Some(y3), self.a, self.b)
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