use _4_abstract_factory::{ui_factory::uifactories::{LinuxUIFact, WinUIFact}, App};

fn main() {

    let ui_linux = App::new(Box::new(LinuxUIFact));
    ui_linux.paint();

    println!("Now for windows");
    App::new(Box::new(WinUIFact)).paint();
}
