// 2. Open closed principle
// Software entities should be open for extension but closed for modification.
// You should be able to extend a class, without modifying it.
// This encourages developers to add new functionality through extensions rather than altering 
// existing code, thus minimizing the risk of introducing bugs.

// mod bad_example;
// use bad_example::Operation;

use good_example::{Add, Operation};

mod good_example;

fn main() {
    let add = Add::calculate(1.2, 2.1);
    println!("{}",add);
}
