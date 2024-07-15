trait RestaurantEmployee {
    fn wash_dishes();
    fn serve_dishes();
    fn cook_food();
}

struct Waiter {
    name: String,
    age: u8,
}

impl RestaurantEmployee for Waiter {
    fn cook_food() {
        // not my job..
    }
    fn serve_dishes() {
        println!("Waiter serving food..");
    }
    fn wash_dishes() {
        // not my job..
    }
}