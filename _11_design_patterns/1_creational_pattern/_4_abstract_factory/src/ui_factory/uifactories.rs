use crate::os_class::{ linux::Linux, ui_components::{Button, CheckBox}, windows::Window};

pub trait UIFactory {
    fn create_button(&self) -> Box<dyn Button> ;
    fn create_checkbox(&self) -> Box<dyn CheckBox>;
}

pub struct WinUIFact;

impl UIFactory for WinUIFact {
    fn create_button(&self)  -> Box<dyn Button> {
        Box::new(Window)
    }
    fn create_checkbox(&self) -> Box<dyn CheckBox> {
        Box::new(Window)
    }
}

pub struct LinuxUIFact;

impl UIFactory for LinuxUIFact {
    fn create_button(&self) -> Box<dyn Button> {
        Box::new(Linux)
    }
    fn create_checkbox(&self) -> Box<dyn CheckBox> {
        Box::new(Linux)
    }
}