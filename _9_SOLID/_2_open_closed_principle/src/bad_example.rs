pub struct Operation { }
// let's say we need more operations like division, multiplication, modulus and many more.
// But this principle states that it should not modify the class or struct.
// Therefore we then create different class that should be able to extend the ops.
// if one of the ops has bug then , it makes tedious job to go through this big class definitions
// hence better to have different class.

impl Operation {
    pub fn calculate(v1: f32, v2: f32, ops: char) -> f32 {
        match ops {
            '+' => v1 + v2,
            '-' => v1 - v2,
             _ => 0.0
        }
    }
}