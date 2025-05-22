use _1_graph::graph::Graph;
use rooting::tree_center;

mod rooting;
fn main() {

    let mut t1_graph = Graph::new(_1_graph::edges::edge::Edge::UNDIRECTED_EDGE);
    t1_graph.add_vertex(0, "0".to_string());
    t1_graph.add_vertex(1, "1".to_string());
    t1_graph.add_vertex(2, "2".to_string());
    t1_graph.add_vertex(3, "3".to_string());
    t1_graph.add_vertex(4, "4".to_string());
    t1_graph.add_vertex(5, "5".to_string());
    t1_graph.add_vertex(6, "6".to_string());
    t1_graph.add_vertex(7, "7".to_string());
    t1_graph.add_vertex(8, "8".to_string());
    t1_graph.add_vertex(9, "9".to_string());
    
    t1_graph.add_edge(0, 1, None);
    t1_graph.add_edge(2, 1, None);
    t1_graph.add_edge(2, 3, None);
    t1_graph.add_edge(2, 9, None);
    t1_graph.add_edge(2, 6, None);
    t1_graph.add_edge(6, 7, None);
    t1_graph.add_edge(6, 8, None);
    t1_graph.add_edge(3, 4, None);
    t1_graph.add_edge(3, 5, None);

    println!("Graph 1");
    // println!("{}\n", t1_graph);
    tree_center(&mut t1_graph);
    // println!("{}",t1_graph);
    
    let mut t2_graph = Graph::new(_1_graph::edges::edge::Edge::UNDIRECTED_EDGE);
    t2_graph.add_vertex(0, "0".to_string());
    t2_graph.add_vertex(1, "1".to_string());
    t2_graph.add_vertex(2, "2".to_string());
    t2_graph.add_vertex(3, "3".to_string());
    t2_graph.add_vertex(4, "4".to_string());
    t2_graph.add_vertex(5, "5".to_string());
    t2_graph.add_vertex(6, "6".to_string());
    t2_graph.add_vertex(7, "7".to_string());
    t2_graph.add_vertex(8, "8".to_string());
    t2_graph.add_vertex(9, "9".to_string());

    t2_graph.add_edge(1, 0, None);
    t2_graph.add_edge(1, 3, None);
    t2_graph.add_edge(1, 4, None);
    t2_graph.add_edge(4, 5, None);
    t2_graph.add_edge(4, 8, None);
    t2_graph.add_edge(3, 7, None);
    t2_graph.add_edge(3, 6, None);
    t2_graph.add_edge(3, 2, None);
    t2_graph.add_edge(6, 9, None);

    println!("Graph 2");
    tree_center(&mut t2_graph);
}
