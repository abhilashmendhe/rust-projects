use std::ptr;


#[derive(Debug)]
pub struct Node {
    pub value: i32,
    pub left: *mut Node,
    pub right: *mut Node
}

impl Node {
    pub fn new(value: i32) -> *mut Node {
        Box::into_raw(Box::new(Node {
            value, 
            left: ptr::null_mut(),
            right: ptr::null_mut()
        }))
    }
}
#[derive(Debug)]
pub struct BST {
    pub top: *mut Node
}

impl BST {
    pub fn new() -> Self {
        BST {
            top: ptr::null_mut()
        }
    }

    pub fn insert(value: i32, node: *mut Node) -> *mut Node {
       
        unsafe {
            
            if node.is_null() {
                return Node::new(value);
            }
            if value < (*node).value {
                (*node).left = Self::insert(value, (*node).left);
            } else if value > (*node).value {
                (*node).right = Self::insert(value, (*node).right);
            }
            node
        }
    }
    pub fn num_of_nodes(node: *mut Node) -> u32 {
        unsafe {
            if node.is_null() {
                return 0;
            }
            Self::num_of_nodes((*node).left) + Self::num_of_nodes((*node).right) + 1
        }
    }
    pub fn height(node: *mut Node) -> u32 {

        unsafe  {
            if node.is_null() {
                return 1;
            }
            let l_count = Self::height((*node).left) + 1;
            let r_count = Self::height((*node).right) + 1;
            if l_count > r_count {
                l_count
            } else {
                r_count
            }
        }
    }
    pub fn delete(value: i32, mut node: *mut Node) -> *mut Node {
        unsafe {
            if node.is_null() {
                println!("Node with value: {} not found!",value);
                return ptr::null_mut();
            }
            if value == (*node).value {
                println!("Found with value: {}",value);
                if (*node).left.is_null() && (*node).right.is_null() {
                    node = ptr::null_mut();
                    return node;
                }
                if (*node).left.is_null() {
                    return (*node).right;
                } 
                if (*node).right.is_null() {
                    return (*node).left;
                } 
                if !(*node).left.is_null() && !(*node).right.is_null() {
                    // println!("Do pre-suce");
                    if Self::height((*node).left) > Self::height((*node).right) {
                        println!("inorder predecessor");
                        if (*(*node).left).right.is_null() {
                            let new_node = Node::new((*(*node).left).value);
                            (*new_node).right = (*node).left;
                            (*new_node).left = (*(*node).left).left;
                            (*node).left = ptr::null_mut();
                            (*node).right = ptr::null_mut();
                            return new_node;
                        } else {
                            let new_node = Node::new((*(*(*node).left).right).value);
                            (*(*node).left).right = Self::delete((*new_node).value, (*(*node).left).right);
                            (*new_node).left = (*node).left;
                            (*new_node).right = (*node).right;
                            return new_node;
                        }
                    } else {
                        println!("inorder successor");
                        if (*(*node).right).left.is_null() {
                            let new_node = Node::new((*(*node).right).value);
                            (*new_node).left = (*node).left;
                            (*new_node).right = (*(*node).right).right;
                            (*node).left = ptr::null_mut();
                            (*node).right = ptr::null_mut();
                            return new_node;
                        } else {
                            let new_node = Node::new((*(*(*node).right).left).value);
                            (*(*node).right).left = Self::delete((*new_node).value, (*(*node).right).left);
                            (*new_node).left = (*node).left;
                            (*new_node).right = (*node).right;
                            return new_node;
                        }
                    }
                }
            }

            if value < (*node).value {
                (*node).left = Self::delete(value, (*node).left);
            } else if value > (*node).value {
                println!("{}",(*node).value);
                (*node).right = Self::delete(value, (*node).right);
            }
        
            node
        }
    }
    pub fn inorder(topnode: *mut Node) {
        unsafe {
            if ! topnode.is_null() {
                Self::inorder((*topnode).left);
                print!("{} ",(*topnode).value);
                Self::inorder((*topnode).right);
            }
        }
    }
}