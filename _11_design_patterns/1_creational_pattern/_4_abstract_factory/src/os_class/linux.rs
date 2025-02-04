use super::ui_components::{Button, CheckBox};


pub struct Linux;

impl Button for Linux {
    fn paint(&self) {
        println!("Linux button created!");
    }
}

impl CheckBox for Linux {
    fn paint(&self) {
        println!("Linux checkbox created!");
    }
}