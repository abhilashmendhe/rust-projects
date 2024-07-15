// let's say we need more operations like division, multiplication, modulus and many more.
// But this principle states that it should not modify the class or struct.
// Therefore we then create different class that should be able to extend the ops.
// if one of the ops has bug then , it makes tedious job to go through this big class definitions
// hence better to have different class.

pub trait Operation {
    fn calculate(v1: f32, v2: f32) -> f32;
}
pub enum Add {}
impl Operation for Add {
    fn calculate(v1: f32, v2: f32) -> f32 {
        v1 + v2
    }
}

pub enum Sub{}
impl Operation for Sub {
    fn calculate(v1: f32, v2: f32) -> f32 {
        v1 - v2
    }
}
pub enum Mul{}
impl Operation for Mul {
    fn calculate(v1: f32, v2: f32) -> f32 {
        v1 * v2
    }
}

pub enum Div{}
impl Operation for Div {
    fn calculate(v1: f32, v2: f32) -> f32 {
        v1 / v2
    }
}