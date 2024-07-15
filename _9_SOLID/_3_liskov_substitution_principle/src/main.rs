/*
3. Liskov Substitution Principle
Subclass should extend the capability of parent class, not narrow down

If class B is a subtype of class A, then we should be able to replace object of A with B without
breaking the behaviour of the program.

Definition: Objects of a superclass should be replaceable with objects of a subclass without affecting the correctness of the program. This principle ensures that derived classes extend the base class without changing its behavior.
*/

mod bad_example;
// use bad_example;
fn main() {

    println!("Hello, world!");
    
}
