use std::sync::{Arc, Mutex, mpsc};
use std::thread;
use std::time::Duration;

#[derive(Debug)]
struct Chopstick;

struct Philosopher {
    name: String,
    left_chopstick: Arc<Mutex<Chopstick>>,
    right_chopstick: Arc<Mutex<Chopstick>>,
    thoughts: mpsc::Sender<String>
}

impl Philosopher {
    pub fn new(
        name: String,
        left_chopstick: Arc<Mutex<Chopstick>>,
        right_chopstick: Arc<Mutex<Chopstick>>,
        thoughts: mpsc::Sender<String>
    ) -> Self {
        Philosopher {
            name, 
            left_chopstick, 
            right_chopstick, 
            thoughts
        }
    }

    fn think(&self) {
        self.thoughts
            .send(format!("Eureka! {} has a new idea!", &self.name))
            .unwrap();
    }

    fn eat(&self) {
        // Pick up chopsticks...
        let _l = self.left_chopstick.lock().unwrap();
        let _r = self.right_chopstick.lock().unwrap();
        println!("{} is eating... {:?}, {:?}", &self.name,_l,_r);
        thread::sleep(Duration::from_millis(10));
        println!("{} done eating!\n",&self.name);
    }
}

// static PHILOSOPHERS: &[&str] =
//     &["Socrates", "Hypatia", "Plato", "Aristotle", "Pythagoras"];
static PHILOSOPHERS: &[&str] =
    &["Socrates", "Hypatia", "Plato"];

fn main() {
    // Sender and Receiver channels 
    let (tx, rx) = mpsc::channel();

    // Create chopsticks
    let mut chopsticks = vec![];
    for _ in 0..PHILOSOPHERS.len() {
        chopsticks.push(Arc::new(Mutex::new(Chopstick)));
    }
    // println!("{:?}",chopsticks);

    // Create philosophers
    for i in 0..chopsticks.len() {
        
        // First we get the left and right chopsticks for the respective philosophers
        // let mut right_chopsticks = Arc::clone(&chopsticks[i]);
        // let mut left_chopsticks = Arc::clone(&chopsticks[(i+1) % chopsticks.len()]);

        let right_chopsticks: Arc<Mutex<Chopstick>>;
        let left_chopsticks: Arc<Mutex<Chopstick>>;
                
        // 1. To prevent deadlock let even number phil. pick right chopsticks first
        // then left chopsticks. And, the odd number phil. pick vice-versa.
        if i % 2 == 0 {
            right_chopsticks = Arc::clone(&chopsticks[i]);
            left_chopsticks = Arc::clone(&chopsticks[(i+1) % chopsticks.len()]);
        } else {
            right_chopsticks = Arc::clone(&chopsticks[(i+1) % chopsticks.len()]);
            left_chopsticks = Arc::clone(&chopsticks[i]);
        }
        
        // 2. The below logic is to prevent deadlock.
        // If everyone picks up the right chopsticks first, then let the last phil. picks
        // the left chopsticks first.
        // if i==chopsticks.len() - 1{
        //     std::mem::swap(&mut left_chopsticks, &mut right_chopsticks);
        // }

        let thoughts = tx.clone();
        let phil = Philosopher::new(
            PHILOSOPHERS[i].to_string(), 
            left_chopsticks, 
            right_chopsticks, 
            thoughts
        );

        // Now iterate N times, so that each phil. eats and thinks N times.
        thread::spawn(move || {
            for _ in 0..10 {
                phil.eat();
                phil.think();
            }
        });
    }

    drop(tx);
    for _thought in rx {
        println!("{}",_thought);
    }
}
