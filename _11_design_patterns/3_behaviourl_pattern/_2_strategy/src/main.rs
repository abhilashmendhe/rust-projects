trait Strategy {
    fn do_opearation(&self, n1: i32, n2: i32) -> i32;
}

struct OperationAdd; // concrete class that implements Strategy

impl OperationAdd {
    fn new() -> Self {
        Self
    }
}
impl Strategy for OperationAdd {
    fn do_opearation(&self, n1: i32, n2: i32) -> i32 {
        n1 + n2
    }
}

struct OperationMultiply;

impl OperationMultiply {
    fn new() -> Self {
        Self
    }
}

impl Strategy for OperationMultiply {
    fn do_opearation(&self, n1: i32, n2: i32) -> i32 {
        n1 * n2
    }
}

struct OperationSub;
impl OperationSub {
    fn new() -> Self {
        Self
    }
}
impl Strategy for OperationSub {
    fn do_opearation(&self, n1: i32, n2: i32) -> i32 {
        n1 - n2
    }
}

struct Context<'a> {
    strategy: &'a dyn Strategy
}

impl<'a> Context<'a> {
    fn new(strategy: &'a dyn Strategy) -> Self {
        Self {
            strategy
        }
    }

    fn execute_strategy(&self, n1: i32, n2: i32) -> i32 {
        self.strategy.do_opearation(n1, n2)
    }
}

fn main() {
    let add_ops = OperationAdd::new();
    let context = Context::new(&add_ops);
    println!("10 + 2 = {}",context.execute_strategy(10, 2));

    let mul_ops = OperationMultiply::new();
    let context = Context::new(&mul_ops);
    println!("12 * 6 = {}",context.execute_strategy(12, 6));

}
