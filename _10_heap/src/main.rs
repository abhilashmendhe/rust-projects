use _10_heap::Heap;

fn main() {

    let arr:Vec<i32> = vec![4,2,5,7,1,6,12,8,10,3,9,11];
    let mut min_heap = Heap::min_heap(12);
    min_heap.insert(4);
    for elem in arr {{
        min_heap.insert(elem);
    }}
    min_heap.print();

    // println!("Head elem: {:?}",min_heap.peek());
    println!("{:?}",min_heap.extract());
    // min_heap.print();
    println!("{:?}",min_heap.extract());
    println!("{:?}",min_heap.extract());
    println!("{:?}",min_heap.extract());
    // // min_heap.print();
    println!("{:?}",min_heap.extract());
    println!("{:?}",min_heap.extract());
    println!("{:?}",min_heap.extract());
    println!("{:?}",min_heap.extract());
    println!("{:?}",min_heap.extract());
    println!("{:?}",min_heap.extract());
    println!("{:?}",min_heap.extract());
    println!("{:?}",min_heap.extract());
    println!("{:?}",min_heap.extract());
    
}
