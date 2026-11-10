use _1_graph::graph::Graph;
use _12_kruskal_mst::union_find::compute_disjoint_set;
fn main() {
    let mut g = Graph::new(_1_graph::edges::edge::Edge::UNDIRECTED_EDGE);
    g.add_vertex(0, String::from("A"));
    g.add_vertex(1, String::from("B"));
    g.add_vertex(2, String::from("C"));
    g.add_vertex(3, String::from("D"));
    g.add_vertex(4, String::from("E"));
    g.add_vertex(5, String::from("F"));
    g.add_vertex(6, String::from("G"));
    g.add_vertex(7, String::from("H"));
    g.add_vertex(8, String::from("I"));
    g.add_vertex(9, String::from("J"));

    g.add_edge(0, 1, Some(5));
    g.add_edge(0, 3, Some(4));
    g.add_edge(0, 4, Some(1));

    g.add_edge(1, 2, Some(4));
    g.add_edge(1, 3, Some(2));

    g.add_edge(2, 7, Some(4));
    g.add_edge(2, 8, Some(1));
    g.add_edge(2, 9, Some(2));

    g.add_edge(3, 4, Some(2));
    g.add_edge(3, 5, Some(5));
    g.add_edge(3, 6, Some(11));
    g.add_edge(3, 7, Some(2));

    g.add_edge(4, 5, Some(1));

    g.add_edge(5, 6, Some(7));

    g.add_edge(6, 7, Some(1));
    g.add_edge(6, 8, Some(4));

    g.add_edge(8, 9, Some(0));
    g.add_edge(7, 8, Some(6));

    
    // let unique_sorted_edges = g.get_unique_edges_sorted();
    // // println!("{:?}", unique_sorted_edges);
    let unique_sorted_edges = [(0 as i64, 8 as u32, 9 as u32), (1, 0, 4), (1, 2, 8), (1, 4, 5), (1, 6, 7), (2, 3, 4), (2, 1, 3), (2, 2, 9), (2, 3, 7), (4, 2, 7), (4, 0, 3), (4, 6, 8), (4, 1, 2), (5, 3, 5), (5, 0, 1), (6, 7, 8), (7, 5, 6), (11, 3, 6)];
    // let unique_sorted_edges = [(3, 0, 2), (1, 2, 1), (2, 1, 3), (2, 3, 4), (6, 4, 5)];
    compute_disjoint_set(&g, unique_sorted_edges.to_vec());
}
