use _1_graph::graph::Graph;
use encode_tree::encode_tree;
mod encode_tree;
// Isomorphic Trees
// Two graphs or trees are isomorhic is asking whether they are structurally the same.
fn main() {

    let mut g = Graph::new(_1_graph::edges::edge::Edge::UNDIRECTED_EDGE);
    g.add_vertex(0, "0".to_string());
    g.add_vertex(1, "1".to_string());
    g.add_vertex(2, "2".to_string());
    g.add_vertex(3, "3".to_string());
    g.add_vertex(4, "4".to_string());
    g.add_vertex(5, "5".to_string());
    g.add_vertex(6, "6".to_string());
    g.add_vertex(7, "7".to_string());
    g.add_vertex(8, "8".to_string());
    g.add_vertex(9, "9".to_string());

    g.add_edge(0, 2, None);
    g.add_edge(0, 1, None);
    g.add_edge(0, 3, None);

    g.add_edge(2, 6, None);
    g.add_edge(2, 7, None);

    g.add_edge(1, 4, None);
    g.add_edge(1, 5, None);

    g.add_edge(5, 9, None);
    g.add_edge(3, 8, None);

    // println!("{}",g);
    let fisrt_tree_encode = encode_tree(&mut g);

    let mut g = Graph::new(_1_graph::edges::edge::Edge::UNDIRECTED_EDGE);
    g.add_vertex(0, "0".to_string());
    g.add_vertex(1, "1".to_string());
    g.add_vertex(2, "2".to_string());
    g.add_vertex(3, "3".to_string());
    g.add_vertex(4, "4".to_string());
    g.add_vertex(5, "5".to_string());
    g.add_vertex(6, "6".to_string());
    g.add_vertex(7, "7".to_string());
    g.add_vertex(8, "8".to_string());
    g.add_vertex(9, "9".to_string());

    g.add_edge(0, 2, None);
    g.add_edge(0, 1, None);
    g.add_edge(0, 3, None);

    g.add_edge(3, 6, None);
    g.add_edge(3, 7, None);

    g.add_edge(1, 4, None);
    g.add_edge(1, 5, None);

    g.add_edge(5, 9, None);
    g.add_edge(2, 8, None);

    // println!("{}",g);
    let second_tree_encode = encode_tree(&mut g);
    // "(()())(())", "(())()"
    // "(()())(())", "(())()"
    // println!("First tree encode: {}", fisrt_tree_encode);
    // println!("Second tree encode: {}", second_tree_encode);

    if fisrt_tree_encode == second_tree_encode {
        println!("Both the trees are isomorphic!"); 
    } else {
        println!("Bot the trees are not isomorphic!");
    }
}
