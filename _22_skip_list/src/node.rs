use std::ptr::NonNull;

pub type Link = Option<NonNull<Node>>;

#[derive(Debug)]
pub struct Node {
    pub key: i64,
    pub forward: Vec<Link>,
}

impl Node {
    pub fn new(key: i64, levels: usize) -> Self {
        Self {
            key,
            forward: vec![None; levels + 1],
        }
    }
}
