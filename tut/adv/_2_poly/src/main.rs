trait Sellable {
    fn price(&self) -> u16;
    fn description(&self) -> String;
}
struct Sword {
    name: String,
    damage: u16, 
    swing_time_ms: u16
}

impl Sellable for Sword {
    fn description(&self) -> String {
        format!("{}, damage: {}, swing time: {}ms", self.name, self.damage, self.swing_time_ms)
    }
    fn price(&self) -> u16 {
        self.damage * 100_u16 / (self.swing_time_ms * 10)
    }
}
struct Shield {
    name: String,
    armor: u16,
    block: u16
}

impl Sellable for Shield {
    fn description(&self) -> String {
        format!("{}, armour: {}, block: {}ms", self.name, self.armor, self.block)
    }
    fn price(&self) -> u16 {
        self.armor + self.block
    }
}

fn vendor_text_static<T: Sellable>(item: &T) -> String {
    format!("I offer you: {} [{}g]",item.description(), item.price())
}

fn vendor_text_dynamic(item: &dyn Sellable) -> String {
    format!("I offer you: {} [{}g]",item.description(), item.price())
}

fn main() {
    let sword = Sword {
        name: "Sword of Cowardice".into(),
        damage: 10,
        swing_time_ms: 1500,
    };
    let shield = Shield {
        name: "Golden Barrier".into(),
        armor: 50,
        block: 35
    };

    println!("{}",vendor_text_static(&sword));
    println!("{}",vendor_text_static(&shield));

    let sellables:Vec<&dyn Sellable> = vec![&sword, &shield];
    for s in  sellables {
        println!("{}",vendor_text_dynamic(s));
    }

    let owned_sellables: Vec<Box<dyn Sellable>> = vec![Box::from(sword), Box::from(shield)];
    for ss in owned_sellables{
        println!("{}",vendor_text_dynamic(ss.as_ref()));
    }
}
