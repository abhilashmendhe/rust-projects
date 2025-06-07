use std::{fmt::Debug, marker::PhantomData, ptr::NonNull};

type Link<T> = Option<NonNull<Node<T>>>;

pub struct Node<T: Debug> {
    elem: T,
    left: Link<T>,
    right: Link<T>,
    level: usize,
    _node_phantom: PhantomData<T>
}

impl<T: Debug> Node<T> {
    fn new(elem: T, level: usize) -> Self {
        Self { 
            elem, 
            left: None, 
            right: None, 
            level,
            _node_phantom: PhantomData
        }
    }
}

pub enum PrintTraversal {
    PREORDER,
    INORDER,
    POSTORDER
}

pub struct AVLTree<T: Clone + Debug> {
    pub root: Option<NonNull<Node<T>>>,
    // height: usize,
    _tree_phanton: PhantomData<T>
}

impl<T: Clone + std::cmp::PartialOrd + Debug> AVLTree<T>  {
    pub fn new() -> Self {
        Self { 
            root: None,
            // height: 0,
            _tree_phanton: PhantomData
         }
    }

    pub fn insert(&mut self, elem: T) {

        let root_node = self.root;
        self.root = AVLTree::insert_recurs(root_node, elem, 0);
    }

    fn insert_recurs(node_ptr: Link<T>, elem: T, level: usize) -> Link<T> {
        unsafe {
            if let Some(t_node) = node_ptr {
                
                if elem < (*t_node.as_ptr()).elem {
                    (*t_node.as_ptr()).left = AVLTree::insert_recurs(
                        (*t_node.as_ptr()).left,
                        elem,
                        level + 1
                    );    
                } else if elem > (*t_node.as_ptr()).elem {
                    (*t_node.as_ptr()).right = AVLTree::insert_recurs(
                        (*t_node.as_ptr()).right, 
                        elem,
                        level + 1
                    );
                } else {
                    return Some(t_node);
                }
                
                Some(t_node)
            } else {
                let new = NonNull::new_unchecked(
                    Box::into_raw(Box::new(
                        Node::new(elem, level)
                    ))
                );
                Some(new)
            }
        }
    }
    
    pub fn total_nodes(&self) -> usize {
        AVLTree::total_nodes_recurs(self.root)
    }

    fn total_nodes_recurs(node_ptr: Link<T>) -> usize {
        unsafe {
            if let Some(node) = node_ptr {

                AVLTree::total_nodes_recurs((*node.as_ptr()).left) + 
                AVLTree::total_nodes_recurs((*node.as_ptr()).right) + 
                1
            } else {

                0
            }
        }
    } 

    pub fn height(&self) -> usize {
        AVLTree::height_recurs(self.root)
    }

    fn height_recurs(node_ptr: Link<T>) -> usize {
        unsafe {
            if let Some(node) = node_ptr {
                let left_count = AVLTree::height_recurs((*node.as_ptr()).left) + 1;
                let right_count = AVLTree::height_recurs((*node.as_ptr()).right) + 1;
                if left_count > right_count {
                    left_count
                }  else {
                    right_count
                }
            } else {
                0
            }
        }
    }

    pub fn print_tree(&self, order_type: PrintTraversal) {
        
        match order_type {
            PrintTraversal::PREORDER => AVLTree::preorder(self.root),
            PrintTraversal::INORDER => AVLTree::inorder(self.root),
            PrintTraversal::POSTORDER => AVLTree::postorder(self.root),
        }
        println!();
    }
    fn preorder(node_ptr: Link<T>) {
        unsafe {
            if let Some(n) = node_ptr {
                print!("{:?} ({}) -> ", (*n.as_ptr()).elem, (*n.as_ptr()).level);
                AVLTree::preorder((*n.as_ptr()).left);
                AVLTree::preorder((*n.as_ptr()).right);
            }
        }
    }
    fn inorder(node_ptr: Link<T>) {
        unsafe {
            if let Some(n) = node_ptr {
                AVLTree::inorder((*n.as_ptr()).left);
                print!("{:?} ({}) -> ", (*n.as_ptr()).elem, (*n.as_ptr()).level);
                AVLTree::inorder((*n.as_ptr()).right);
            }
        }
    }
    fn postorder(node_ptr: Link<T>) {
        unsafe {
            if let Some(n) = node_ptr {
                AVLTree::postorder((*n.as_ptr()).left);
                AVLTree::postorder((*n.as_ptr()).right);
                print!("{:?} ({}) -> ", (*n.as_ptr()).elem, (*n.as_ptr()).level);
            }
        }
    }
}

// impl <T: PartialEq + Clone + ?Sized> PartialEq for AVLTree<T> {
//     fn eq(&self, other: &Self) -> bool {
//         true
//     }
//     fn ne(&self, other: &Self) -> bool {
//         false
//     }
// }
// impl <T: PartialOrd + Clone + ?Sized> PartialOrd for AVLTree<T> {
//     fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
//         Some(std::cmp::Ordering::Less)
//     }
// }