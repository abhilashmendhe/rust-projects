use _1_graph::{edges::edge::Edge::DIRECTED_EDGE, graph::Graph};
use _9_bellman_ford::bellman_ford;

fn main() {
    let mut g = Graph::new(DIRECTED_EDGE);

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

    // From 0
    g.add_edge(0, 1, Some(5));

    // From 1
    g.add_edge(1, 2, Some(20));
    g.add_edge(1, 5, Some(30));
    g.add_edge(1, 6, Some(60));

    // From 2
    g.add_edge(2, 3, Some(10));
    g.add_edge(2, 4, Some(75));

    // From 3
    g.add_edge(3, 2, Some(-15));

    // From 4
    g.add_edge(4, 9, Some(100));

    // From 5
    g.add_edge(5, 4, Some(25));
    g.add_edge(5, 6, Some(5));
    g.add_edge(5, 8, Some(50));

    // From 6
    g.add_edge(6, 7, Some(-50));

    // From 7
    g.add_edge(7, 8, Some(-10));

    // println!("{}", g);
    let mut edge_list = vec![];
    for v in &g.vertices {
        for edge in &v.edges {
            edge_list.push(edge.clone());
        }
    }
    bellman_ford(&g, edge_list);
}
