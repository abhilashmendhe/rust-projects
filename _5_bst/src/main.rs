use _5_bst::*;
fn main() {


    let mut tree = BST::new();
    tree.top = BST::insert(10, tree.top);
    tree.top = BST::insert(5,  tree.top);
    tree.top = BST::insert(15, tree.top);
    tree.top = BST::insert(20, tree.top);
    tree.top = BST::insert(1, tree.top);
    tree.top = BST::insert(12, tree.top);
    tree.top = BST::insert(8, tree.top);
    tree.top = BST::insert(25, tree.top);
    tree.top = BST::insert(30, tree.top);
    tree.top = BST::insert(35, tree.top);
    tree.top = BST::insert(6, tree.top);
    BST::inorder(tree.top);
    println!();

    println!("Total nodes: {}", BST::num_of_nodes(tree.top));
    println!("Height of tree: {}",BST::height(tree.top));

    tree.top = BST::delete(10, tree.top);

    BST::inorder(tree.top);
    println!();
}
