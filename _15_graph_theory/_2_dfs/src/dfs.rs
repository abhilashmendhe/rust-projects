use _1_graph::{graph::Graph, vertex::Vertex};


pub fn dfs_search(g: Graph, start_id: usize) {

    let n_vertices = g.vertices.len();
    let vertices = g.vertices;

    // Our id are simply integer values starting from 0
    // Therefore every index in visited array represent the id of a vertex
    // To make it simple we chose such visited array to track the visit of every vertex/node.
    let mut visited = vec![false; n_vertices]; 

    // print!("{} ", start_id);
    fn dfs(visited:&mut Vec<bool>, start_id: usize, vertices: &Vec<Vertex>) {  
        if !visited[start_id] {
            // println!("No visited");
            
            visited[start_id] = true;

            if let Some(v) = vertices.iter().find(|v| v.get_vertex_id() == start_id as u32) {
                print!("{} ({}) -> ", start_id, v.get_name());
                for edge in v.get_edges() {
                    // print!("{} ", edge.get_dest_id());
                    dfs(visited, edge.get_dest_id() as usize, &vertices);
                }
            }
        }
    }
    dfs(&mut visited, start_id, &vertices);
    println!();
}