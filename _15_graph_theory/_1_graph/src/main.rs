use edges::edge::Edge::*;
use graph::Graph;

pub mod graph;
pub mod vertex;
pub mod edges;
fn main() {

    // Create a graph
    let mut g = Graph::new(UNDIRECTED_EDGE);

    // Add vertex to the graph
    g.add_vertex(1, "Mumbai".to_string());
    g.add_vertex(2, "Kolkata".to_string());
    g.add_vertex(3, "Bengaluru".to_string());
    g.add_vertex(4, "Hyderabad".to_string());
    g.add_vertex(5, "Delhi".to_string());
    g.add_vertex(6, "Chennai".to_string());
    g.add_vertex(7, "Lucknow".to_string());
    g.add_vertex(8, "Pune".to_string());
    g.add_vertex(9, "Guwhati".to_string());
    g.add_vertex(10, "Kochi".to_string());
    g.add_vertex(11, "Chandigarh".to_string());

    // Create an edge between 2 vertices
    g.add_edge(1, 2, None);
    g.add_edge(1, 3, None);
    g.add_edge(1, 5, None);
    g.add_edge(1, 6, None);
    g.add_edge(1, 11, None);
    
    g.add_edge(2, 7, None);
    g.add_edge(2, 9, None);
    g.add_edge(2, 10, None);
    g.add_edge(2, 5, None);
    
    g.add_edge(5, 4, None);
    g.add_edge(5, 6, None);
    g.add_edge(5, 7, None);
    g.add_edge(5, 8, None);
    g.add_edge(5, 11, None);
    g.add_edge(5, 10, None);
    
    g.add_edge(10, 1, None);
    g.add_edge(10, 8, None);
    g.add_edge(10, 3, None);
    g.add_edge(10, 6, None);
    
    // println!("{}",g);
    // Delete a vertex by id
    g.delete_vertex(11);

    // Delete an edge or 2-way edge (from src id to dest id)
    g.delete_edge(5, 1);

    // Check if 2 vertices are neighbors (src id and dest id)
    g.check_neighbors(2, 1);

    // Print all neigbors of a vertex (pass vertex id)
    g.print_neigbors(2);

    // println!("{}",g);

    /*  
        io::stdout().flush().unwrap();
        // user input 
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        let input = input.trim().parse::<u8>().unwrap();
    */
}

