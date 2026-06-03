use std::ptr::NonNull;

use crate::node::Node;

#[derive(Debug)]
pub struct SkipList {
    pub p: f32,
    pub levels: usize,
    pub head: NonNull<Node>,
}

impl SkipList {
    pub fn new(p: f32, levels: usize) -> Self {
        let head =
            unsafe { NonNull::new_unchecked(Box::into_raw(Box::new(Node::new(-1, levels)))) };
        Self { p, levels, head }
    }

    pub fn random_levels(&self) -> usize {
        let mut levels = 0;
        while rand::random::<f32>() < self.p && levels < self.levels {
            levels += 1;
        }
        levels
    }

    pub fn search(&self, target: i64) -> bool {
        unsafe {
            let mut current = self.head;
            let mut i = self.levels;

            while i > 0 {
                while let Some(nnode) = (&(*current.as_ptr()).forward)[i] {
                    if (*nnode.as_ptr()).key < target {
                        current = nnode;
                    } else {
                        break;
                    }
                }
                i -= 1;
            }
            if let Some(nnode) = (&(*current.as_ptr()).forward)[0] { 
                if (*nnode.as_ptr()).key == target {
                    return true;
                }
            }
        }
        false
    }
}

impl Drop for SkipList {
    fn drop(&mut self) {}
}
