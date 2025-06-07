use _18_avl_trees::AVLTree;

use _18_avl_trees::PrintTraversal;

fn main() {

    let mut avl_tree = AVLTree::<u8>::new();

    avl_tree.insert(5);
    avl_tree.insert(2);
    avl_tree.insert(10);
    avl_tree.insert(3);
    avl_tree.insert(15);
    avl_tree.insert(1);
    avl_tree.insert(8);

    avl_tree.print_tree(PrintTraversal::PREORDER);

    println!("Total nodes: {}",avl_tree.total_nodes());
    println!("Height of tree: {}", avl_tree.height());
}
