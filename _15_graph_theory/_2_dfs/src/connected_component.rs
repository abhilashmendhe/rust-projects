use _1_graph::{graph::Graph, vertex::Vertex};


pub fn connected_component(g: Graph) {
    
    let vertices = g.vertices;
    let mut visited = vec![false; vertices.len()];

    for v in &vertices {
        let v_id = v.get_vertex_id();
        if !visited[v_id as usize] {
             println!();
        }
        dfs(&mut visited, v_id as usize, &vertices);
         
    }
    println!();
}   

fn dfs(visited:&mut Vec<bool>, start_id: usize, vertices: &Vec<Vertex>) {
    
    if !visited[start_id] {
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