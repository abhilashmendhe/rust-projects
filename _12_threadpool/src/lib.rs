use std::sync::{mpsc::{channel, Sender}, Arc, Mutex};

pub struct ThreadPool{
    _handles: Vec<std::thread::JoinHandle<()>>,
    sender: Sender<Box<dyn Fn() + Send>>
}

impl ThreadPool {
    pub fn new(num_threads: u8) -> Self {

        let (sender, receiver) = channel::<Box<dyn Fn() + Send>>();
        let receiver = Arc::new(Mutex::new(receiver))   ;
        let mut _handles = vec![];

        for _ in 0..num_threads {
            let clone_recv = receiver.clone();
            
            let handle = std::thread::spawn(move || {
                loop {        
                        // check for work
                        let work = clone_recv.lock().unwrap().recv().unwrap();
                                
                        // once work, do work
                        println!("Start");
                        work();
                        println!("End");
                    }
            });
            _handles.push(handle);
        }

        Self {
            _handles,
            sender
        }
    }

    pub fn execute<T>(&self, work: T)
        where T: Fn() + Send + 'static
    {
        // Box<(dyn Fn() + Send + 'static)>
        self.sender.send(Box::new(work)).unwrap();
    }
}





/*

dynamic dispatch example

// fn foo<T: std::fmt::Debug>(item: T) {
//     println!("Foo");
// }

fn foo(item: Box<dyn std::fmt::Debug>) {
    println!("foo");
}
fn bar() {

    // foo func is generating code for every input type. This is called monomorphization

    foo(Box::new(1u8));
    foo(Box::new(String::new()));
    foo(Box::new(2 as i32));
}
*/

/*

let _handles = (0..num_threads)
        // .map(|_| {
        //     std::thread::spawn(move || {
        //         loop {
                    
        //             // check for work
        //             let work = receiver.lock().unwrap().recv().unwrap();
                    
        //             // once work, do work
        //             work();

        //             // then go back

        //         }
        //     })
        // }).collect();
*/