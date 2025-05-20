use std::any::Any;

#[derive(Debug)]
pub struct Activity<Details:?Sized> {
    title: String,
    time_spent: f32,
    details: Details
}

impl<T:?Sized> Activity<T> {
    // Methods that apply to all activities
    fn details(&self)->&T { &self.details }
    fn details_mut(&mut self)->&mut T { &mut self.details }

    // ...
}

struct Walk {
    route: String
}
fn main() {
    let mut activity_log: Vec<Box<Activity<dyn Any>>> = vec![];
    activity_log.push(
        Box::new( Activity {
        title: String::from("Walking"),
        time_spent: 1.5,
        details: Walk {
            route: String::from("Beach")
        }
        }
    )
    );
    println!("{:?}", activity_log);
}
