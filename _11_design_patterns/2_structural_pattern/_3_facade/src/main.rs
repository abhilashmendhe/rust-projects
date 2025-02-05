struct Restaurant;
impl Restaurant {
    fn new() -> Self {
        Self
    }
    fn prepare_order(&self) {
        println!("Preparing order...");
    }
}

struct DeliveryBoy;
impl DeliveryBoy {
    fn new() -> Self {
        Self
    }
    fn pickup_order(&self) {
        println!("Order picked up by the delivery boy..");
    }
    fn deliver_order(&self) {
        println!("Order deliver by the delivery boy...");
    }
}

struct DeliveryTeam;
impl DeliveryTeam {
    fn new() -> Self {
        Self
    }
    fn assign_delivery_boy(&self) {
        println!("Team assigning a delivery boy....");
    }
}

pub struct Client;
impl Client {
    fn new() -> Self {
        Self
    }
    pub fn order_food(&self) {
        println!("Food ordered by client...");
    }
}

struct UberFacade {
    restaurant: Restaurant,
    delivery_boy: DeliveryBoy,
    delivery_team: DeliveryTeam,
    client: Client
}


impl UberFacade {
    fn new()->UberFacade {
        UberFacade {
            restaurant: Restaurant::new(),
            delivery_boy: DeliveryBoy::new(),
            delivery_team: DeliveryTeam::new(),
            client: Client::new()
        }
    }
    
    fn place_order(&self) {
        self.client.order_food();
        self.restaurant.prepare_order();
        self.delivery_team.assign_delivery_boy();
        self.delivery_boy.pickup_order();
        self.delivery_boy.deliver_order();
    }
}


fn main() {
    let uber_app = UberFacade::new();
    uber_app.place_order();
}
