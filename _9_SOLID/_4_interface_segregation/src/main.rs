/*
4. Interface Segregation Principle (ISP)
Definition: Clients should not be forced to depend on interfaces they do not use. 
Instead of one large interface, multiple smaller and more specific interfaces are preferred, 
promoting a more decoupled and flexible system.

Interfaces should be such, that client should not implement unecessary functions they do not need.
*/

mod bad_example;
mod good_example;
fn main() {
    println!("Hello, world!");
}
