use std::{sync::{Arc, Mutex, RwLock}, thread};

fn main() {
    // 1. Basic example to share data in threads using Arc
    // // counter = 1
    // let data = Arc::new(vec![1,2,3,4,5]);
    // // counter = 2
    // let data_for_thread = Arc::clone(&data);
    // // println!("data counter {}",Arc::strong_count(&data));
    // let handle = thread::spawn( move || {   
    //     println!("Print data in thread: {:?}",data_for_thread);
    // });

    // let other_data = data;
    // println!("Print data in main: {:?}",other_data);
    // let _ = handle.join();
    
    // // 2. example using Arc with mutex to manipulate data
    // let data = Arc::new(Mutex::new(vec![1,2,3,4,5]));
    // let mut handles = vec![];

    // for i in 0..=3 {
    //     let data_clone = Arc::clone(&data);
    //     let handle = thread::spawn(move || {
    //         let mut data = data_clone.lock().unwrap();
    //         data.push(i);
    //         drop(data);
    //         println!("Print data in thread {}: {:?}",i, data_clone.lock().unwrap());
    //     });
    //     handles.push(handle);
    // }

    // for handle in handles {
    //     handle.join().unwrap();
    // }
    // println!("Print data in main: {:?}",data);

    // 3. RwLock with Arc to manipulate data
    let data = Arc::new(RwLock::new(vec![1,2,3,4,5]));
    let mut handles = vec![];

    for i in 0..=3 {
        let data_clone = Arc::clone(&data);
        let handle = thread::spawn(move || {
            let mut data = data_clone.write().unwrap();
            data.push(i);
            drop(data);

            println!("Print data in thread {}: {:?}",i, data_clone.read().unwrap());
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
    println!("Print data in main: {:?}",data);

}
