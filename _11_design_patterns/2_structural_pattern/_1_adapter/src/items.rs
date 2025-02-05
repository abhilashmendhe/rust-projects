pub trait Item {
    fn get_item_name(&self) -> String;
    fn get_price(&self) -> f32;
    fn get_restaurant_name(&self) -> String;
}

pub trait GroceryItem {
    fn get_name(&self) -> String;
    fn get_price(&self) -> f32;
    fn get_store_name(&self) -> String;
}