/*
5. Dependency Inversion Principle (DIP)
Definition: High-level modules should not depend on low-level modules. Both should depend on 
abstractions (e.g., interfaces). Additionally, abstractions should not depend on details. 
This principle encourages the decoupling of software modules, making the system more flexible 
and easier to maintain.
*/

use good_example::{MacBook, WiredMouse, WirelessKeyboard};

mod bad_example;
mod good_example;
fn main() {
    let wired_mouse = Box::new(WiredMouse {name:"Logitech Wired Mouse".to_string()});
    let wireless_keyboard = Box::new(WirelessKeyboard{name: "Razer Blade wireless".to_string()});

    let macbook_air = MacBook::new(wireless_keyboard, wired_mouse);

}
