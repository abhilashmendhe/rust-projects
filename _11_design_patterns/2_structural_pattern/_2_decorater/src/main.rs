trait Pizza {
    fn bake(&self);
}
struct BasePizza;
impl Pizza for BasePizza {
    fn bake(&self){
        println!("Base Pizza..");
    }
}

struct CheeseBurstDecorator<'a> {
    pizza: &'a dyn Pizza
}

impl<'a> Pizza for CheeseBurstDecorator<'a> {
    fn bake(&self) {
        self.pizza.bake();
        println!("Adding Chesse Burst...");
    }
}

struct ThinCrustDecorator<'a> {
    pizza: &'a dyn Pizza
}

impl<'a> Pizza for ThinCrustDecorator<'a> {
    fn bake(&self) {
        self.pizza.bake();
        println!("Adding thin crust layer...");
    }
}


fn main() {
    println!("Ordering Base Pizza...");
    let base_pizza = BasePizza;
    base_pizza.bake();

    println!("\nOrdering Cheese Burst Pizza...");
    let cheese_burst = CheeseBurstDecorator {
        pizza: &base_pizza
    };
    cheese_burst.bake();

    println!("\nOrdering Thin Crust Pizza...");
    let thin_crust = ThinCrustDecorator {
        pizza: &base_pizza
    };
    thin_crust.bake();
}
