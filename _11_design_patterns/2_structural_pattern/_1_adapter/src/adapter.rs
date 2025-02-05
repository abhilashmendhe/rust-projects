use crate::items::{GroceryItem, Item};

pub struct GroceryItemAdapter {
    item: Box<dyn GroceryItem>
}

impl GroceryItemAdapter {
    pub fn new(item: Box<dyn GroceryItem>) -> Self {
        GroceryItemAdapter {
            item
        }
    }
}

impl Item for GroceryItemAdapter {
    
    fn get_item_name(&self) -> String {
        self.item.get_name()
    }
    fn get_price(&self) -> f32 {
        self.item.get_price()
    }
    fn get_restaurant_name(&self) -> String {
        self.item.get_store_name()
    }
}