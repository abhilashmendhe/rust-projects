use std::collections::LinkedList;

use _1_graph::{graph::Graph, vertex::Vertex};

pub fn topo_sort(g: Graph) {

    let vertices = &g.vertices;
    let n = vertices.len();
    let mut visited = vec![false; n];

    let mut order_list = LinkedList::new();
    for v in vertices {
        let start_id = v.get_vertex_id();
        dfs(&mut visited, start_id as usize, &vertices, &mut order_list);
    }
    
    println!("{:?}", order_list);
}

fn dfs(visited:&mut Vec<bool>, start_id: usize, vertices: &Vec<Vertex>, order_list: &mut LinkedList<(usize, String)>) {  
    if !visited[start_id] {
        
        visited[start_id] = true;

        if let Some(v) = vertices.iter().find(|v| v.get_vertex_id() == start_id as u32) {
            
            for edge in v.get_edges() {
                dfs(visited, edge.get_dest_id() as usize, &vertices, order_list);
            }

            // print!("{} ({}) -> ", start_id, v.get_name());  
            order_list.push_front((start_id, v.get_name().to_string()));
        }
    }
}