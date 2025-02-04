/*
Singleton
---------
Singleton is a creational design pattern that lets you ensure that a class has only one instance, 
while providing a global access point to this instance.

Ensure that a class has just a single instance. Why would anyone want to control how many instances 
a class has? The most common reason for this is to control access to some shared resource—for example, 
a database or a file.
*/

use _1_singleton::Database;

fn main() {
    let dbobj1 = Database::new("http://127.0.0.1:3306".to_string());
    println!("{:?}",dbobj1);
    let dbobj1 = Database::new("http://127.0.0.1:3306".to_string());
    println!("{:?}",dbobj1);
}
