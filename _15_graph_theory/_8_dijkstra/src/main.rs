use _1_graph::graph::Graph;
use _8_dijkstra::lazy_dijkstra::lazy_dijk_shortest;

fn main() {
    let mut g = Graph::new(_1_graph::edges::edge::Edge::DIRECTED_EDGE);

    g.add_vertex(0, "".to_string());
    g.add_vertex(1, "".to_string());
    g.add_vertex(2, "".to_string());
    g.add_vertex(3, "".to_string());
    g.add_vertex(4, "".to_string());

    g.add_edge(0, 1, Some(4));
    g.add_edge(0, 2, Some(1));

    g.add_edge(1, 3, Some(1));

    g.add_edge(2, 1, Some(2));
    g.add_edge(2, 3, Some(5));

    g.add_edge(3, 4, Some(3));

    // println!("{}", g);
    lazy_dijk_shortest(&g);
}
