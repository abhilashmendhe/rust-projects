use crate::items::Item;

pub struct UberInv{
    items: Vec<Box<dyn Item>>
}

impl UberInv {
    pub fn new() -> UberInv {
        UberInv {
            items: vec![]
        }
    }

    pub fn add_items(&mut self, item: Box<dyn Item>) {
        self.items.push(item);
    }

    // pub fn print(&self) {
    //     for item in self.items.iter() {
            
    //     }
    // }
}