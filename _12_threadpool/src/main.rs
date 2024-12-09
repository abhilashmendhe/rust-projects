use std::time::Duration;

use _12_threadpool::ThreadPool;

fn main() {

    let pool = ThreadPool::new(10);
    

    let f = || {
        std::thread::sleep(Duration::from_secs(5));    
        // println!("Hello from thread");
    };
    pool.execute(f.clone());
    pool.execute(f);
    // pool.execute(|| println!("Hello from thread1"));
    // pool.execute(|| println!("Hello from thread2"));
    // pool.execute(|| println!("Hello from thread3"));
    // pool.execute(|| println!("Hello from thread4"));
    

    
}