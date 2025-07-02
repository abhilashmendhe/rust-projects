use std::fmt::{write, Display};

#[derive(Debug, PartialEq, PartialOrd, Eq)]
pub struct FieldElement {
    number: usize,
    prime: usize
}

impl FieldElement {
    pub fn new(number: usize, prime: usize) -> Self {
        if number >= prime {
            panic!("Number {} is not in the field range 0 to {}", number, prime - 1);
        }
        Self {
            number,
            prime
        }
    }
}

impl Display for FieldElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Field element is number: {}, prime: {}", self.number, self.prime)
    }
}