use adapter::GroceryItemAdapter;
use food::FoodItem;
use grocery::GroceryProduct;
use uber_inventory::UberInv;

pub mod items;
pub mod food;
pub mod grocery;
mod adapter;
mod uber_inventory;

fn main() {
    let mut u = UberInv::new();

    u.add_items(Box::new(FoodItem::new()));
    u.add_items(Box::new(FoodItem::new()));
    u.add_items(Box::new(GroceryItemAdapter::new(Box::new(GroceryProduct::new()))));
    
}
