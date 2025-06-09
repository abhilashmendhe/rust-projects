use std::{collections::VecDeque, fmt::Debug, marker::PhantomData, ptr::NonNull};

type Link<T> = Option<NonNull<Node<T>>>;

pub struct Node<T: Debug> {
    elem: T,
    left: Link<T>,
    right: Link<T>,
    _level: usize,
    height: isize,
    _node_phantom: PhantomData<T>
}

impl<T: Debug> Node<T> {
    fn new(elem: T, _level: usize) -> Self {
        Self { 
            elem, 
            left: None, 
            right: None, 
            _level,
            height: 1,
            _node_phantom: PhantomData
        }
    }
}

#[allow(non_camel_case_types)]
pub enum PrintTraversal {
    
    DFS_PREORDER,
    DFS_INORDER,
    DFS_POSTORDER,
    BFS
}

pub struct AVLTree<T: Clone + Debug> {
    pub root: Option<NonNull<Node<T>>>,
    _tree_phanton: PhantomData<T>
}

impl<T: Clone + std::cmp::PartialOrd + Debug> AVLTree<T>  {
    pub fn new() -> Self {
        Self { 
            root: None,
            _tree_phanton: PhantomData
         }
    }

    fn get_balance_factor(node_ptr: Link<T>) -> isize {

        unsafe {
            if let Some(node) = node_ptr {
                let left_child_height = AVLTree::specific_node_height((*node.as_ptr()).left);
                let right_child_height = AVLTree::specific_node_height((*node.as_ptr()).right);
                left_child_height - right_child_height
            } else {
                0
            }
        }
    }

    pub fn specific_node_height(node_ptr: Link<T>) -> isize {
        // We won't be doing recursive calls, instead store a variable in Node struct as height
        // Everytime creating a node we set height to 1. This will take O(1) to compute the height
        // When we go up from leaves to root, we compute the height and set it accordingly
        // AVLTree::height_recurs(self.root)

        if let Some(node) = node_ptr {
            unsafe {
                (*node.as_ptr()).height
            }
        } else {
            0
        }

    }

    pub fn get_max_height(node_ptr: Link<T>) -> isize {

        if let Some(t_node) = node_ptr {
            unsafe {
                // get left child height and right child height
                let left_child_height = AVLTree::specific_node_height((*t_node.as_ptr()).left);
                let right_child_height = AVLTree::specific_node_height((*t_node.as_ptr()).right);
                
                // get the max height
                let height = left_child_height.max(right_child_height) + 1;
                return height;
            }
        }
        0
    }
    fn rotate_right(node_ptr: Link<T>) -> Link<T> {
        unsafe {

            if let Some(node) = node_ptr {
                if let Some(l_node) = (*node.as_ptr()).left {

                    let l_r_node = (*l_node.as_ptr()).right;
                    (*l_node.as_ptr()).right = Some(node);
                    (*node.as_ptr()).left = l_r_node;

                    // Update height
                    (*node.as_ptr()).height = AVLTree::get_max_height(Some(node));
                    (*l_node.as_ptr()).height = AVLTree::get_max_height(Some(l_node));

                    return Some(l_node);
                }
            }
            node_ptr
        }
    }

    fn rotate_left(node_ptr: Link<T>) -> Link<T> {
        unsafe {

            if let Some(node) = node_ptr {
                if let Some(r_node) = (*node.as_ptr()).right {

                    let r_l_node = (*r_node.as_ptr()).left;
                    (*r_node.as_ptr()).left = Some(node);
                    (*node.as_ptr()).right = r_l_node;

                    // Update height
                    (*node.as_ptr()).height = AVLTree::get_max_height(Some(node));
                    (*r_node.as_ptr()).height = AVLTree::get_max_height(Some(r_node));
                    
                    return Some(r_node);
                }
            }
            node_ptr
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
                        elem.clone(),
                        level + 1
                    );    
                } else if elem > (*t_node.as_ptr()).elem {
                    (*t_node.as_ptr()).right = AVLTree::insert_recurs(
                        (*t_node.as_ptr()).right, 
                        elem.clone(),
                        level + 1
                    );
                } else {
                    return Some(t_node);
                }
                
                // set the max height to the node when going upwards
                (*t_node.as_ptr()).height = AVLTree::get_max_height(node_ptr);

                // get balance factor
                let balance_factor = AVLTree::get_balance_factor(node_ptr);
                
                // For Left case
                if balance_factor > 1 {

                    if let Some(l_node) = (*t_node.as_ptr()).left {
                        
                        // For left left case   (rotate right)
                        if elem < (*l_node.as_ptr()).elem {
                            // println!("rotating right");
                            return AVLTree::rotate_right(Some(t_node));
                        }

                        // For left right case  (rotate left and then right)
                        if elem > (*l_node.as_ptr()).elem {
                            // rotate left
                            (*t_node.as_ptr()).left = AVLTree::rotate_left(Some(l_node));
                            // rotate right
                            return AVLTree::rotate_right(Some(t_node));
                        }           
                    }

                }

                // For right case
                if balance_factor < -1 {
                    if let Some(r_node) = (*t_node.as_ptr()).right {
                        // For right right case   (rotate right)
                        if elem > (*r_node.as_ptr()).elem {
                            // println!("rotating right");
                            return AVLTree::rotate_left(Some(t_node));
                        }

                        // For right left case (rotate)
                        if elem < (*r_node.as_ptr()).elem {
                            (*t_node.as_ptr()).right = AVLTree::rotate_right(Some(r_node));
                            return AVLTree::rotate_left(Some(t_node));
                        }
                    }
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
    
    pub fn delete(&mut self, elem: T) {
        let root_node = self.root;
        self.root = AVLTree::delete_recurs(root_node, elem);
    }

    pub fn delete_recurs(node_ptr: Link<T>, elem: T) -> Link<T> {

        if let Some(t_node) = node_ptr {
            unsafe {
                if elem < (*t_node.as_ptr()).elem {
                    (*t_node.as_ptr()).left = AVLTree::delete_recurs(
                        (*t_node.as_ptr()).left, 
                        elem
                    );
                } else if elem > (*t_node.as_ptr()).elem {
                    (*t_node.as_ptr()).right = AVLTree::delete_recurs(
                        (*t_node.as_ptr()).right, 
                        elem
                    );
                } else {

                    if (*t_node.as_ptr()).left == None {
                        let temp = (*t_node.as_ptr()).right;
                        return temp;
                    }
                    if (*t_node.as_ptr()).right == None {
                        let temp = (*t_node.as_ptr()).left;
                        return temp;
                    }

                    let in_succ = AVLTree::get_successor(Some(t_node));

                    if let Some(succ) = in_succ {
                        (*t_node.as_ptr()).elem = (*succ.as_ptr()).elem.clone();
                        if let Some(r_node) = (*t_node.as_ptr()).right {
                            (*t_node.as_ptr()).right = AVLTree::delete_recurs(Some(r_node), (*succ.as_ptr()).elem.clone());
                        }
                    }
                }

                // get balance factor
                let balance_factor = AVLTree::get_balance_factor(node_ptr);
                
                // For Left case
                if balance_factor > 1 {

                    if let Some(l_node) = (*t_node.as_ptr()).left {

                        // check left left node
                        if let Some(_) = (*l_node.as_ptr()).left {
                            // simply rotate right     
                            return AVLTree::rotate_right(Some(t_node));
                        } 

                        // check left right node 
                        if let Some(_) = (*l_node.as_ptr()).right {
                            // rotate left
                            (*t_node.as_ptr()).left = AVLTree::rotate_left(Some(l_node));
                            // rotate right
                            return AVLTree::rotate_right(Some(t_node));
                        }
                    }

                }
                // For right case
                if balance_factor < -1 {
                    if let Some(r_node) = (*t_node.as_ptr()).right {
                        
                        // check right right node
                        if let Some(_) = (*r_node.as_ptr()).right {
                            // simply rotate left     
                            return AVLTree::rotate_left(Some(t_node));
                        }

                        // check right left node
                        if let Some(_) = (*r_node.as_ptr()).left {
                            // rotate right
                            (*t_node.as_ptr()).right = AVLTree::rotate_right(Some(r_node));                
                            // rotate left 
                            return AVLTree::rotate_left(Some(t_node));
                        }
                    }
                }
            }
        }
        node_ptr
    }

    fn get_successor(node_ptr: Link<T>) -> Link<T> {
        
        if let Some(t_node) = node_ptr {
            unsafe {
                if let Some(mut r_node) = (*t_node.as_ptr()).right {

                    while let Some(r_l_node) = (*r_node.as_ptr()).left {
                        r_node = r_l_node;
                    }
                    return Some(r_node);
                }
            }
        }    
        None
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
            PrintTraversal::DFS_PREORDER => AVLTree::preorder(self.root),
            PrintTraversal::DFS_INORDER => AVLTree::inorder(self.root),
            PrintTraversal::DFS_POSTORDER => AVLTree::postorder(self.root),
            PrintTraversal::BFS => AVLTree::bfs(self.root),
        }
        println!();
    }
    fn preorder(node_ptr: Link<T>) {
        unsafe {
            if let Some(n) = node_ptr {
                print!("{:?} ({}) -> ", (*n.as_ptr()).elem, (*n.as_ptr()).height);
                AVLTree::preorder((*n.as_ptr()).left);
                AVLTree::preorder((*n.as_ptr()).right);
            }
        }
    }
    fn inorder(node_ptr: Link<T>) {
        unsafe {
            if let Some(n) = node_ptr {
                AVLTree::inorder((*n.as_ptr()).left);
                print!("{:?} ({}) -> ", (*n.as_ptr()).elem, (*n.as_ptr()).height);
                AVLTree::inorder((*n.as_ptr()).right);
            }
        }
    }
    fn postorder(node_ptr: Link<T>) {
        unsafe {
            if let Some(n) = node_ptr {
                AVLTree::postorder((*n.as_ptr()).left);
                AVLTree::postorder((*n.as_ptr()).right);
                print!("{:?} ({}) -> ", (*n.as_ptr()).elem, (*n.as_ptr()).height);
            }
        }
    }

    fn bfs(node_ptr: Link<T>) {
        
        let mut queue = VecDeque::<Link<T>>::new();
        queue.push_back(node_ptr);
        queue.push_back(None);
        
        while let Some(node) = queue.pop_front() {
            if let Some(n) = node {
                unsafe {
                    queue.push_back((*n.as_ptr()).left);
                    queue.push_back((*n.as_ptr()).right);
                    queue.push_back(None);
                    print!("{:?} ({}) -> ", (*n.as_ptr()).elem, (*n.as_ptr()).height);
                }
            } else {
                // println!();
            }
        }
        println!();
    }
}
