use std::{fmt::Debug,  ptr::NonNull};

type Link<T>= Option<NonNull<Node<T>>>;

#[derive(Debug)]
pub struct Node<T: Debug> {
    key: T, 
    forward: Vec<Link<T>>,
}

impl<T: Debug> Node<T> {
    pub fn new(key: T, levels: usize) -> Self {
        Self { key, forward: vec![None; levels + 1] }
    }
}
