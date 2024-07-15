trait WaiterInterface {
    fn serve_dishes();
    fn take_order();
}

trait ChefInterface {
    fn cook_food();
    fn decide_menu();
}

struct Employee {
    name: String,
    age: u8,
}

impl WaiterInterface for Employee {
    
    fn serve_dishes() {
        println!("Waiter serving food..");
    }
    fn take_order() {
        
    }
}

impl ChefInterface for Employee {
    fn cook_food() {
        
    }
    fn decide_menu() {
        
    }
}