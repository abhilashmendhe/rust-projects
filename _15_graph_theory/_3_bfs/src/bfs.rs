use std::collections::VecDeque;

use _1_graph::graph::Graph;

pub fn bfs_search(g: Graph, start_id: u32) {

    let vertices = g.vertices;
    let mut visited = vec![false; vertices.len()];
    let mut queue = VecDeque::new();

    queue.push_back(start_id);
    visited[start_id as usize] = true;

    while !queue.is_empty() {
        let node_id = queue.pop_front().unwrap();
        if let Some(vertex_node) = vertices
                                            .iter()
                                            .find(|v| v.get_vertex_id() == node_id) {
            print!("{} ({}) - ", node_id, vertex_node.get_name());
            for e in &vertex_node.edges {
                // print!("{} - ", e.get_dest_id());
                if !visited[e.get_dest_id() as usize] {
                    // print!("{} - ", e.get_dest_id());
                    queue.push_back(e.get_dest_id());
                    visited[e.get_dest_id() as usize] = true;
                }
            }
            
        }
    }
    println!();
}