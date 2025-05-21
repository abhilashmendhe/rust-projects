// use my_graph::*;
use _1_graph::{graph::Graph, *};
use connected_component::connected_component;
use dfs::dfs_search;

/*
    DFS - Depth First Search

    It is usefull when augmented to perform other tasks such as count connected components, determine
    connectivity, or find bridges/articulation points then DFS really shines.
    1. Compute graph minimum spanning tree
    2. Detect and find cycles in a graph
    3. Check if graph is bipartie
    4. Topo sort
    5. Find bridges and articulation points.
    6. Generate mazes.
    7. Find augmenting paths in a flow network.
*/
mod dfs;
mod connected_component;
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

    graph.add_edge(0, 1, None);
    graph.add_edge(0, 9, None);
    graph.add_edge(1, 8, None);
    graph.add_edge(9, 8, None);

    graph.add_edge(8, 7, None);
    graph.add_edge(7, 10, None);
    graph.add_edge(7, 11, None);
    graph.add_edge(10, 11, None);

    graph.add_edge(7, 6, None);
    graph.add_edge(7, 3, None);
    graph.add_edge(5, 6, None);
    graph.add_edge(5, 3, None);

    graph.add_edge(3, 2, None);
    graph.add_edge(3, 4, None);

    println!("---- Graph 1 ----");
    // println!("{}",graph);
    // For DFS
    println!("------ DFS -------");
    dfs_search(graph, 0);
    println!();

    let mut graph2 = Graph::new(edges::edge::Edge::UNDIRECTED_EDGE);
    graph2.add_vertex(0, "Mumbai".to_string());
    graph2.add_vertex(1, "Delhi".to_string());
    graph2.add_vertex(2, "Kolkata".to_string());
    graph2.add_vertex(3, "Chennai".to_string());
    graph2.add_vertex(4, "Pune".to_string());
    graph2.add_vertex(5, "Bengluru".to_string());
    graph2.add_vertex(6, "Kochi".to_string());
    graph2.add_vertex(7, "Chandigarh".to_string());
    graph2.add_vertex(8, "Lucknow".to_string());
    graph2.add_vertex(9, "Punjab".to_string());
    graph2.add_vertex(10, "Dehradun".to_string());
    graph2.add_vertex(11, "Jaipur".to_string());
    graph2.add_vertex(12, "Darjeeling".to_string());
    graph2.add_vertex(13, "Bihar".to_string());
    graph2.add_vertex(14, "Hyderabad".to_string());
    graph2.add_vertex(15, "Shimla".to_string());
    graph2.add_vertex(16, "Indore".to_string());
    graph2.add_vertex(17, "Raipur".to_string());

    graph2.add_edge(0, 4, None);
    graph2.add_edge(0, 8, None);
    graph2.add_edge(0, 13, None);
    graph2.add_edge(0, 14, None);
    graph2.add_edge(4, 8, None);
    graph2.add_edge(8, 14, None);
    graph2.add_edge(14, 13, None);

    graph2.add_edge(5, 1, None);
    graph2.add_edge(5, 17, None);
    graph2.add_edge(5, 16, None);

    graph2.add_edge(6, 7, None);
    graph2.add_edge(6, 11, None);
    graph2.add_edge(7, 11, None);

    graph2.add_edge(3, 9, None);
    graph2.add_edge(9, 15, None);
    graph2.add_edge(9, 2, None);
    graph2.add_edge(2, 15, None);
    graph2.add_edge(15, 10, None);

    println!("---- Graph 2 ----");
    // println!("{}", graph2);
    connected_component(graph2);
}
