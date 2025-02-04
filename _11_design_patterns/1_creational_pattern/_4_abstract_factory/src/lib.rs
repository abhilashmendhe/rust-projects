use os_class::ui_components::{Button, CheckBox};
use ui_factory::uifactories::UIFactory;

pub mod ui_factory;
pub mod os_class;
pub struct App {
    pub button: Box<dyn Button>,
    pub checkbox: Box<dyn CheckBox>
}

impl App {
    pub fn new(ui: Box<dyn UIFactory>) -> App {
        let button = ui.create_button();
        App {
            button,
            checkbox: ui.create_checkbox()
        }
    }
    pub fn paint(&self) {
        self.button.paint();
        self.checkbox.paint();
    }
}