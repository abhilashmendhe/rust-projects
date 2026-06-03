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

    fn in_search(&self, target: i64) -> (NonNull<Node>, Vec<Option<NonNull<Node>>>) {
        let mut current = self.head;
        let mut update = vec![None; self.levels + 1];
        unsafe {
            for i in (0..=self.levels).rev() {
                while let Some(nnode) = (&(*current.as_ptr()).forward)[i] {
                    if (*nnode.as_ptr()).key < target {
                        current = nnode;
                    } else {
                        break;
                    }
                }
                update[i] = Some(current);
            }
            if let Some(nnode) = (&(*current.as_ptr()).forward)[0] {
                current = nnode;
            }
        }
        (current, update)
    }

    pub fn search(&self, target: i64) -> bool {
        let (current, _) = self.in_search(target);
        unsafe {
            if (*current.as_ref()).key == target {
                return true;
            } else {
                false
            }
        }
    }

    pub fn insert(&self, key: i64) {
        let (_current, update) = self.in_search(key);
        let new_node_level = self.random_levels();
        unsafe {
            let new_node_tower =
                NonNull::new_unchecked(Box::into_raw(Box::new(Node::new(key, new_node_level))));
            for i in 0..=new_node_level {
                let pred = update[i];
                // let succ = (&(*update[i].unwrap().as_ptr()).forward)[i];
                if let Some(succ) = update[i] {
                    (&mut (*new_node_tower.as_ptr()).forward)[i] = (&(*succ.as_ptr()).forward)[i];
                }
                if let Some(predd) = pred {
                    (&mut (*predd.as_ptr()).forward)[i] = Some(new_node_tower);
                }
            }
        }
    }

    pub fn display(&self) {
        println!("\n*******Skip List********");
        let head = self.head;
        for level in 0..=self.levels {
            print!("Level {}: ", level);
            unsafe {
                let mut node = (&(*head.as_ptr()).forward)[level];
                while let Some(n) = node {
                    print!("{} ", (*n.as_ptr()).key);
                    node = (&(*n.as_ptr()).forward)[level];
                }
            }
            println!();
        }
    }
}

impl Drop for SkipList {
    fn drop(&mut self) {}
}
