use _1_graph::graph::Graph;
use _7_sssp::sssp;

fn main() {
    let mut g = Graph::new(_1_graph::edges::edge::Edge::DIRECTED_EDGE);

    // Add vertices
    g.add_vertex(0, "A".to_string()); // A
    g.add_vertex(1, "B".to_string()); // B
    g.add_vertex(2, "C".to_string()); // C
    g.add_vertex(3, "D".to_string()); // D
    g.add_vertex(4, "E".to_string()); // E
    g.add_vertex(5, "F".to_string()); // F
    g.add_vertex(6, "G".to_string()); // G
    g.add_vertex(7, "H".to_string()); // H

    // Add edges
    g.add_edge(0, 1, Some(3));
    g.add_edge(0, 2, Some(6));

    g.add_edge(1, 2, Some(4));
    g.add_edge(1, 3, Some(4));
    g.add_edge(1, 4, Some(11));

    g.add_edge(2, 3, Some(8));
    g.add_edge(2, 6, Some(11));

    g.add_edge(3, 4, Some(-4));
    g.add_edge(3, 5, Some(5));
    g.add_edge(3, 6, Some(2));

    g.add_edge(4, 7, Some(9));
    g.add_edge(5, 7, Some(1));
    g.add_edge(6, 7, Some(2));

    // g.add_vertex(8, "I".to_string());
    // g.add_edge(8, 7, Some(6));
    // println!("{}", g);
    println!("{}", sssp(g));
}
