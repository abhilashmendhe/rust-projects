#[derive(Debug)]
pub struct WiredKeyboard {
    pub name: String
}
#[derive(Debug)]
pub struct WiredMouse {
    pub name: String
}
#[derive(Debug)]
pub struct WirelessKeyboard {
    pub name: String
}
#[derive(Debug)]
pub struct WirelessMouse {
    pub name: String
}

pub trait Keyboard {
    // fn company();
}
pub trait Mouse {
    // fn company();
}
// #[derive(Debug)]
impl Keyboard for WiredKeyboard {}
impl Keyboard for WirelessKeyboard {}
impl Mouse for WiredMouse {}
impl Mouse for WirelessMouse {}

// #[derive(Debug)]
pub struct MacBook {
    pub keyboard: Box<dyn Keyboard>,
    pub mouse: Box<dyn Mouse>,
}

impl MacBook {
    pub fn new(keyboard: Box<dyn Keyboard>, mouse: Box<dyn Mouse>) -> Self {
        MacBook {
            keyboard,
            mouse
        }
    }
}
