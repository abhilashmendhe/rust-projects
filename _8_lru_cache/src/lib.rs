use std::{collections::HashMap, ptr};


#[derive(Debug,Clone)]
pub struct Node {
    pub key: i32,
    pub value: i32,
    pub next: *mut Node,
    pub prev: *mut Node
}

impl Node {
    pub fn new(key: i32, value: i32) -> *mut Node {
        Box::into_raw(Box::new(Node {
            key,
            value, 
            next: ptr::null_mut(),
            prev: ptr::null_mut()
        }))
    }
}

#[derive(Debug)]
pub struct DoubleList {
    pub head: *mut Node,
    pub tail: *mut Node,
    pub capacity: i32,
    pub size: i32,
    pub kv: HashMap<i32, *mut Node>
}


impl DoubleList {
    pub fn new(capacity: i32) -> Self {
        let new_double_list = DoubleList {
            head: Node::new(-1, -1),
            tail: Node::new(-1, -1),
            capacity, 
            size: 0,
            kv: HashMap::new()
        };
        unsafe {
            (*new_double_list.head).next = new_double_list.tail;
            (*new_double_list.tail).prev = new_double_list.head;
        }
        new_double_list
    }
    fn insert(&mut self, node: *mut Node) {
        unsafe {
            let next_node = (*self.head).next;
            (*node).next = next_node;
            (*node).prev = self.head;
            (*next_node).prev = node;
            (*self.head).next = node;
        }
    }
    fn delete(&mut self, node: &mut Node) {

        unsafe {
            let ret_prev_node = node.prev;
            let ret_next_node = node.next;
            node.next = ptr::null_mut();
            node.prev = ptr::null_mut();
            (*ret_prev_node).next = ret_next_node;
            (*ret_next_node).prev = ret_prev_node;
        }
    }
    pub fn get(&mut self, key: i32) -> i32 {
        match self.kv.get_mut(&key) {
            Some(node) => {
                unsafe {
                    let value = (*(*node)).value;
                    let node = node.clone();
                    self.delete(node.as_mut().unwrap());
                    self.insert(node.as_mut().unwrap());
                    value
                }
            },
            None => {
                -1
            }
        }
        
    }
    pub fn put(&mut self, key: i32, value: i32) {
        
        let new_node = Node::new(key, value);
        
        match self.kv.get_mut(&key) {
            Some(node) => {
                unsafe {
                    (*(*node)).value = value;
                    let node = node.clone();
                    self.delete(node.as_mut().unwrap());
                    self.insert(node.as_mut().unwrap());
                }
            },
            None => {
                if self.size < self.capacity {
                    self.insert(new_node);
                    self.kv.insert(key, new_node);
                    self.size += 1;
                } else {
                    unsafe {
                        let last_node = (*self.tail).prev;
                        self.delete(last_node.as_mut().unwrap());
                        self.insert(new_node);
                        self.kv.insert(key, new_node);
                        self.kv.remove(&(*last_node).key);
                    }
                }
            }
        }
    }

    pub fn print(&mut self) {
        unsafe {
            let mut t_node = self.head;

            while !t_node.is_null() {
                
                // print!("{:?} -> ",(*t_node).value);
                print!("({:?}, {:?}) -> ",(*t_node).key, (*t_node).value);
                t_node = (*t_node).next;
            }
            println!();

        }
    }
    pub fn print_rev(&mut self) {
        unsafe {
            let mut t_node = self.tail;

            while !t_node.is_null() {
                
                print!("({:?}, {:?}) -> ",(*t_node).key, (*t_node).value);
                // println!("{:?}",t_node);
                t_node = (*t_node).prev;
            }
            println!();

        }
    }
}