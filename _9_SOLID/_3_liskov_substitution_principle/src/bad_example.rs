trait Bike {
    fn turn_on_engine(&mut self);
    fn accelerate(&mut self);
}

pub struct MotorCycle {
    is_engine_on: bool,
    speed: u8,
}
impl Bike for MotorCycle {
    fn accelerate(&mut self) {
        self.speed += 10;
    }
    fn turn_on_engine(&mut self) {
        self.is_engine_on = true;
    }
}

pub struct Bicycle {
    speed: u8
}
impl Bike for Bicycle {
    fn accelerate(&mut self) {
        self.speed += 3;
    }
    fn turn_on_engine(&mut self) {
        // breaking the behaviour of the code..
        panic!("No engine in Bicyle!!");
    }
}