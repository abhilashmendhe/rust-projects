use super::ui_components::{Button, CheckBox};


pub struct Window;

impl Button for Window {
    fn paint(&self) {
        println!("Windows Button!");
    }
}

impl CheckBox for Window {
    fn paint(&self) {
        println!("Windows Checkbox!")
    }
}