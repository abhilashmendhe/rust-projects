use crate::items::GroceryItem;

#[derive(Debug)]
pub struct GroceryProduct {
    name: String, 
    price: f32, 
    grocery_name: String
}

impl GroceryProduct {
    pub fn new() -> Self {
        GroceryProduct {
            name: "".to_string(),
            price: 0.0,
            grocery_name: "".to_string()
        }
    }
}
impl GroceryItem for GroceryProduct {
    fn get_name(&self) -> String {
        self.name.clone()
    }
    fn get_price(&self) -> f32 {
        self.price
    }
    fn get_store_name(&self) -> String {
        self.grocery_name.clone()
    }
}