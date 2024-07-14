fn main() {
    test_iterators();
}

fn test_iterators() {

    // 1. Basics
    let fruits = vec!["Strawberry","Orange","Apple","Mango","Banana"];

    // let mut fruit_iter = fruits.iter();

    // for fruit in fruit_iter {
    //     println!("{}",fruit);
    // }
    // 2. Next method
    // let item1 = fruit_iter.next();
    // println!("First item in iterator: {}",item1.unwrap());
    // let item2 = fruit_iter.next();
    // println!("First item in iterator: {}",item2.unwrap());

    // 3.  Chain -> combines multiple vec iters
    let nut_list = vec!["Walnut", "Almonds", "Pecans", "Brazil Nuts"];
    let aggregate_foods = fruits.iter().chain(&nut_list);
    // for item in aggregate_foods {
    //     println!("Eating {}",item);
    // }
    // 4. convert back to vectors from iters
    let all_foods = aggregate_foods.collect::<Vec<&&str>>();
    // all_foods.iter()

    // 5. map 
    let fruit_list = fruits
                        .iter()
                        .map(|fr| String::from(*fr));
    let new_fruits = fruit_list.map(|mut e: String| {e.push_str(" fruit."); return e});

    // new_fruits.clone().for_each(|e| println!("{}",e));

    // 6. skip
    let mut step_by_fruits = new_fruits.clone().step_by(2);
    println!("Step - {}",step_by_fruits.next().unwrap());
    println!("Step - {}",step_by_fruits.next().unwrap());
    println!("Step - {}",step_by_fruits.next().unwrap());


    // 7. zip
    let first_name = vec!["Trevor","Shannon","James","Tasha"];
    let first_name_strings = first_name.iter().map(|e| String::from(*e));

    let last_name = vec!["Jones","Sullivan","Tanner","Redman"];
    let last_name_strings = last_name.iter().map(|e| String::from(*e));

    let full_names = first_name_strings.zip(last_name_strings);
    // full_names.clone().for_each(|e| println!("{} {}",e.0,e.1));

    // 8. chain methods. e.g skip().take()
    // full_names.clone().skip(2).take(1).for_each(|e|println!("{:?}",e));

    // 9. fold
    let foods = vec![("potatoes",10),("straberries",25),("burgers",21)];
    let food_quantity = foods.iter().fold(0u32, |a,e|a+e.1);
    println!("{}",food_quantity);
    
}