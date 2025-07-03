use std::{fmt::Display, ops::{Add, Div, Mul, Sub}};


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

    pub fn pow_modulo(&self, exponent: isize) -> Self {
        
        let mut exp = modulo(exponent, self.prime - 1);
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

        Self {
            number: result,
            prime: self.prime
        }
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

/*
    For division, we know that a/b. It can transform to the inverse multiplication i.e; a*b^-1.
    Since we are taking modulo over the div result, we will use Fermat theorem.
    The theorem says that:
                n^(p-1) % p= 1
    Because div is inverse multiplication, we can reduce to multiplication problem.
                a / b = a * b^-1
    From fermat theorem,
                b ^ (p-1) = 1 
    Multiply above equation by b^-1 gives,
                b^-1 = b^(p-2)
    For e.g. F19 (p=19) is b^-1 = b^17
    Now we compute the modulo exponential value of b^17
    The final answer of above modulo exponetial value is equal to b^-1
*/

impl Div for FieldElement {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        if self.prime != rhs.prime {
            panic!("Two different fields (a & b). Can't perform division");
        }
        let exp_value = rhs.pow_modulo(rhs.prime - 2);
        let fout = self * exp_value;
        fout
    }
}

pub fn modulo(a: isize, b: isize) -> isize {
    ((a % b) + b) % b // can perform -ve % +ve and give a +ve modulo output
}
