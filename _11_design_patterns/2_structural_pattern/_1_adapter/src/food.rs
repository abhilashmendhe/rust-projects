use crate::items::Item;

#[derive(Debug)]
pub struct FoodItem {
    name: String, 
    price: f32, 
    restaurant_name: String
}

impl FoodItem {
    pub fn new() -> FoodItem {
        Self {
            name: "".to_string(), 
            price: 0.0,
            restaurant_name: "".to_string()
        }
    }
}

impl Item for FoodItem {
    fn get_item_name(&self) -> String {
        let name = self.name.clone();
        name
    }
    fn get_price(&self) -> f32 {
        self.price
    }
    fn get_restaurant_name(&self) -> String {
        self.restaurant_name.clone()
    }
}

