use std::{sync::mpsc, thread::{self, sleep}, time::Duration};

fn test_mpsc() {

    // 1.single sender and receiver
    // let (tx, rx) = mpsc::channel::<String>(); 

    // let thread = thread::spawn(move || {
    //     sleep(Duration::from_secs(2));
    //     tx.send("Message from thread 1".to_string()).unwrap();
    //     tx.send("Message from thread 2".to_string()).unwrap();
    // });

    // 1.  if something was not send, this will run infinetly, like blocking in nature
    // let msg = rx.recv().unwrap();
    // println!("Received message: {}",msg);

    // 2.  with try recv() you will either get a message or not, but this will not block
    // for i in 0..10000 {
    //     let possible_msg = rx.try_recv();
    //     match possible_msg {
    //         Ok(msg) => println!("Received message in iteration {i}, '{}'",msg),
    //         Err(_) => ()
    //     }
    // }

    // 3. with timeout
    // let possible_msg = rx.recv_timeout(Duration::from_secs(3));
    // match possible_msg {
    //     Ok(msg) => println!("Received message in iteration '{}'",msg),
    //     Err(_) => println!("No message!!")
    // }

    // // 4. receive with iter
    // for msg in rx {
    //     println!("Received message: {}",msg);
    // }
    // println!("Receive iteration terminated.");
    // thread.join().unwrap();

    // // 2. multiple sender and receiver
    // let (tx, rx) = mpsc::channel::<String>(); 
    // let tx2 = tx.clone();
    // let thread1 = thread::spawn(move || {
    //     sleep(Duration::from_secs(1));
    //     tx.send("Message from thread 1.1".to_string()).unwrap();
    //     tx.send("Message from thread 1.2".to_string()).unwrap();
    // });
    // let thread2 = thread::spawn(move || {
    //     // sleep(Duration::from_secs(2));
    //     tx2.send("Message from thread 2.1".to_string()).unwrap();
    //     tx2.send("Message from thread 2.2".to_string()).unwrap();
    // });

    // for msg in rx {
    //     println!("Received message: '{}'.",msg);
    // }
    // thread1.join().unwrap();
    // thread2.join().unwrap();

    // // 3. Async unbounded channel
    // // Unbounded means, n numbers of messages can be sent over channel even if the 
    // // receiver is not there. But this could consume RAM. So we can use bounded one
    // // that is the sync_channel
    // let (tx, rx) = mpsc::channel();
    // let thread = thread::spawn(move || {
    //     for i in 1..=5 {
    //         tx.send(format!("Message {i} from thread.")).unwrap();
    //     }
    //     println!("Finished sending, thread terminating");
    // });
    // sleep(Duration::from_secs(3));
    // for m in rx.iter() {
    //     println!("Got msg in loop: '{}'",m);
    // }
    // println!("Rx loop terminated");

    // thread.join().unwrap();

    // 4. sync_channel, bounded one
    let (tx, rx) = mpsc::sync_channel(1);
    let thread = thread::spawn(move || {
        let msg1 = "Message 1 from thread".to_string();
        let msg2 = "Message 2 from thread".to_string();
        tx.send(msg1).unwrap();
        println!("Sent message 1...");
        tx.send(msg2).unwrap();
        println!("Sent message 2 ...");
    });
    sleep(Duration::from_secs(3));
    let msg = rx.recv().unwrap();
    println!("Received first message: {}",msg);
    let msg = rx.recv().unwrap();
    println!("Received second message: {}",msg);
    thread.join().unwrap();
}
