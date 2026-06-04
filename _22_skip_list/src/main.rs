use std::ptr::NonNull;

use _22_skip_list::skip_list::SkipList;

fn main() {
    // let mut value = 123;
    // unsafe {
    //     let x = NonNull::new(&mut value);
    //     println!("{}", x.unwrap().as_ref());
    //     if let Some(v) = x {
    //         (*v.as_ptr()) = 10;
    //     }
    //     println!("{}", *x.unwrap().as_ptr());
    // }
    // println!("{}", value);

    let skip_list = SkipList::new(0.5, 3);
    skip_list.insert(1);
    skip_list.insert(2);
    skip_list.insert(4);
    skip_list.insert(3);
    skip_list.display();

    skip_list.erase(0);
    skip_list.display();

    skip_list.erase(2);
    skip_list.display();
}
