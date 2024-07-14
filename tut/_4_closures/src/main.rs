fn main() {
    test_closures();
}

struct  Person {
    fisrt_name: String, 
    last_name: String
}
fn test_closures() {
    // 1. Basic closure
    let add = || println!("Retruning some text");
    add();

    // 2. With arguments
    let add = |x: i32, y: i32| x + y;
    let result = add(10,2);
    println!("{}",result);

    // 3. Printing result
    let print_result = |x: i32| println!("The result is {}",(result + x));
    print_result(100);

    // 4. Mutating struct data using closure
    let mut p1 = Person {
        fisrt_name: "Abhi".to_string(), 
        last_name: "Menda".to_string()
    };
    let mut change_name = |new_last_name: &str| p1.last_name=new_last_name.to_string();
    change_name("Mend");
    // println!("{}",p1.last_name);
    change_name("Mendhe");
    println!("{}",p1.last_name);
}