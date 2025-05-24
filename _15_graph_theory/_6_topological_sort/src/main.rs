use _1_graph::graph::Graph;
use topo::topo_sort;

// Topological Sorting
/*
    - A topo ordering is an ordering of nodes in a directed graph where for each directed edge from node A to 
    node B, node A appears before node B in the ordering.
    - The topo sort algo can find a topo ordering in O(V+E) time.
    - Topo ordering are not unique.
    - A graph that has cycle can't have a definite order. Only DAG(directed acyclic graph) can have valid order.
    - To detect cycle, one can use Tarjan's strongly connected algorithm to find cycles in a graph.
    Real world situations can be modelled as a directed graph where some events must occur before others.
    e.g.
        1. Program dependencies
        2. School class pre-requisite
        3. Event scheduling
        4. Assembly instructions
*/
mod topo;

fn main() {
    let mut g = Graph::new(_1_graph::edges::edge::Edge::DIRECTED_EDGE);
    g.add_vertex(0, "A".to_string());
    g.add_vertex(1, "B".to_string());
    g.add_vertex(2, "C".to_string());
    g.add_vertex(3, "D".to_string());
    g.add_vertex(4, "E".to_string());
    g.add_vertex(5, "F".to_string());
    g.add_vertex(6, "G".to_string());
    g.add_vertex(7, "H".to_string());
    g.add_vertex(8, "I".to_string());
    g.add_vertex(9, "J".to_string());
    g.add_vertex(10, "K".to_string());
    g.add_vertex(11, "L".to_string());
    g.add_vertex(12, "M".to_string());

    g.add_edge(0, 3, None);
    g.add_edge(1, 3, None);
    g.add_edge(2, 0, None);
    g.add_edge(2, 1, None);
    g.add_edge(4, 0, None);
    g.add_edge(4, 3, None);
    g.add_edge(4, 5, None);
    g.add_edge(3, 6, None);
    g.add_edge(3, 7, None);
    g.add_edge(6, 8, None);
    g.add_edge(5, 9, None);
    g.add_edge(5, 10, None);
    g.add_edge(7, 9, None);
    g.add_edge(7, 8, None);
    g.add_edge(9, 11, None);
    g.add_edge(9, 12, None);
    g.add_edge(8, 11, None);

    println!("{}",g);
    topo_sort(g);
    
}
