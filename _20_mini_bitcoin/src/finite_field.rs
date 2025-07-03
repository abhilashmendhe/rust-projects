use std::{fmt::Display, ops::{Add, Mul, Sub}};


#[derive(Debug, PartialEq, PartialOrd, Eq, Clone)]
pub struct FieldElement {
    number: isize,
    prime: isize
}

impl FieldElement {
    pub fn new(number: isize, prime: isize) -> Self {
        if number >= prime {
            panic!("Number {} is not in the field range 0 to {}", number, prime - 1);
        }
        Self {
            number,
            prime
        }
    }

    pub fn pow_modulo(&self, exponent: isize) -> isize {
        
        let mut exp = exponent;
        let mut base = self.number;
        let mut result = 1;

        while exp > 0 {
            if exp % 2 == 0 {
                exp /= 2;
                base = modulo(modulo(base, self.prime) * modulo(base, self.prime), self.prime);
            } else {
                exp -= 1;
                result = modulo(modulo(result, self.prime) * modulo(base, self.prime), self.prime);
            }
            
        }
        result
    }
}

impl Display for FieldElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Field element is number: {}, prime: {}", self.number, self.prime)
    }
}

impl Add for FieldElement {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        if self.prime != rhs.prime {
            panic!("Two different fields (a & b). Can't perform addition");
        }
        let left = modulo(self.number , self.prime );
        let right = modulo(rhs.number , rhs.prime );
        let fout = modulo(left + right , self.prime );
        Self {
            number: fout,
            prime: self.prime
        }
    }
}

impl Sub for FieldElement {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        if self.prime != rhs.prime {
            panic!("Two different fields (a & b). Can't perform subtraction");
        }
        let left = modulo(self.number , self.prime );
        let right = modulo(rhs.number , rhs.prime );

        let fout = modulo(left - right + self.prime , self.prime );
        Self {
            number: fout,
            prime: self.prime
        }
    }
}

impl Mul for FieldElement {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        if self.prime != rhs.prime {
            panic!("Two different fields (a & b). Can't perform multiplication");
        }
        let left = modulo(self.number , self.prime );
        let right = modulo(rhs.number , rhs.prime );
        let fout = modulo(left * right , self.prime );
        Self {
            number: fout,
            prime: self.prime
        }
    }
}

pub fn modulo(a: isize, b: isize) -> isize {
    ((a % b) + b) % b // can perform -ve % +ve and give a +ve modulo output
}
