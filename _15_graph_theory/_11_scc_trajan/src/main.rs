use _1_graph::graph::Graph;
use _11_scc_trajan::scc;

fn main() {
    let mut g = Graph::new(_1_graph::edges::edge::Edge::DIRECTED_EDGE);

    g.add_vertex(0, String::new());
    g.add_vertex(1, String::new());
    g.add_vertex(2, String::new());
    g.add_vertex(3, String::new());
    g.add_vertex(4, String::new());
    g.add_vertex(5, String::new());
    g.add_vertex(6, String::new());
    g.add_vertex(7, String::new());

    g.add_edge(0, 1, None);
    g.add_edge(0, 2, None);

    g.add_edge(1, 0, None);
    g.add_edge(1, 3, None);

    g.add_edge(2, 3, None);

    g.add_edge(3, 4, None);
    g.add_edge(3, 5, None);

    g.add_edge(4, 2, None);
    g.add_edge(4, 5, None);
    g.add_edge(4, 6, None);

    g.add_edge(5, 7, None);

    g.add_edge(6, 5, None);

    g.add_edge(7, 6, None);

    let low = scc(&mut g);
    println!("{:?}", low);
}
