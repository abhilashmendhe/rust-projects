use _1_graph::{edges, graph::Graph};
use bfs::bfs_search;
use grids::grid_bfs;
mod bfs;
mod grids;
/*
    BFS
    ---
    It is useful for finding shortest path on unweighted graphs.
*/

fn main() {
    let mut graph = Graph::new(edges::edge::Edge::UNDIRECTED_EDGE);
    graph.add_vertex(0, "Mumbai".to_string());
    graph.add_vertex(1, "Delhi".to_string());
    graph.add_vertex(2, "Kolkata".to_string());
    graph.add_vertex(3, "Chennai".to_string());
    graph.add_vertex(4, "Pune".to_string());
    graph.add_vertex(5, "Bengluru".to_string());
    graph.add_vertex(6, "Kochi".to_string());
    graph.add_vertex(7, "Chandigarh".to_string());
    graph.add_vertex(8, "Lucknow".to_string());
    graph.add_vertex(9, "Punjab".to_string());
    graph.add_vertex(10, "Dehradun".to_string());
    graph.add_vertex(11, "Jaipur".to_string());
    graph.add_vertex(12, "Darjeeling".to_string());

    graph.add_edge(0, 9, None);
    graph.add_edge(0, 7, None);
    graph.add_edge(0, 11, None);

    graph.add_edge(9, 10, None);
    graph.add_edge(9, 8, None);
    graph.add_edge(1, 10, None);
    graph.add_edge(1, 8, None);

    graph.add_edge(7, 11, None);
    graph.add_edge(7, 3, None);
    graph.add_edge(7, 6, None);
    graph.add_edge(5, 6, None);

    graph.add_edge(8, 12, None);
    graph.add_edge(12, 2, None);

    graph.add_edge(3, 2, None);
    graph.add_edge(3, 4, None);

    // println!("{}",graph);

    bfs_search(graph, 0);

    grid_bfs();
}

