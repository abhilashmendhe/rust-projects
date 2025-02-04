#[derive(Debug)]
struct Burger {
    size: String, 
    egg: bool,
    extra_cheese: bool,
    mayo: bool,
    onion: bool,
    lettuce: bool
}

impl Burger {

    fn new() -> Self {
        Burger {
            size: "small".to_string(),
            egg: false,
            extra_cheese: false,
            mayo: false,
            onion: false,
            lettuce: false
        }

    }
    fn size(mut self, size: String) -> Self {
        self.size = size;
        self
    }
    fn egg(mut self, egg: bool) -> Self {
        self.egg = egg;
        self
    }
    fn extra_cheese(mut self, extra_cheese: bool) -> Self {
        self.extra_cheese = extra_cheese;
        self
    }
    fn mayo(mut self, mayo: bool) -> Self {
        self.mayo = mayo;
        self
    }
    fn onion(mut self, onion: bool) -> Self {
        self.onion = onion;
        self
    }
    fn lettuce(mut self, lettuce: bool) -> Self {
        self.lettuce = lettuce;
        self
    }
    fn build(self) -> Self {
        self
    }
}
fn main() {
    let burger = Burger::new().size("medium".to_string()).build();
    println!("{:#?}",burger);
    
}
