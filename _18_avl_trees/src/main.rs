use _18_avl_trees::AVLTree;

use _18_avl_trees::PrintTraversal;

fn main() {

    let mut avl_tree = AVLTree::<u8>::new();

    // avl_tree.insert(10);
    // avl_tree.insert(90);
    // avl_tree.insert(20);
    // avl_tree.insert(80);
    // avl_tree.insert(30);
    // avl_tree.insert(70);
    // avl_tree.insert(50);
    // avl_tree.insert(15);
    // avl_tree.insert(25);
    // avl_tree.insert(5);
    avl_tree.insert(30);
    avl_tree.insert(40);
    // avl_tree.insert(20);
    avl_tree.insert(10);
    avl_tree.insert(20);
    // avl_tree.insert(5);
    avl_tree.print_tree(PrintTraversal::BFS);

    // println!("Total nodes: {}",avl_tree.total_nodes());
    // println!("Height of tree: {}", avl_tree.height());

    let delete_elem = 40;
    avl_tree.delete(delete_elem);

    println!();
    avl_tree.print_tree(PrintTraversal::DFS_PREORDER);

    // println!("Total nodes: {}",avl_tree.total_nodes());
    // println!("Height of tree: {}", avl_tree.height());
}
