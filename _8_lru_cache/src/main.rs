use _8_lru_cache::DoubleList;

fn main() {

    let mut list = DoubleList::new(3);
    println!("{:#?}",list);
    list.put(1, 10);
    list.print();
    list.put(1, 20);
    list.print();
    list.put(3, 30);
    list.print();
    list.put(2, 100);
    list.print();
    list.put(1, 100);
    list.print();

    let ret_val = list.get(10);
    println!("Value for key 10 is {}",ret_val);
    let ret_val = list.get(2);
    println!("Value for key 2 is {}",ret_val);
    list.print();
}
