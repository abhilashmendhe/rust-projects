use std::collections::HashSet;

use _1_graph::{graph::Graph, vertex::Vertex};
pub fn dfs(vertex: &Vertex, g: &Graph, visited: &mut HashSet<u32>, topo_vec: &mut Vec<u32>) {
    if !visited.contains(&vertex.v_id) {
        visited.insert(vertex.v_id);
        for ve in &vertex.edges {
            let dest_id = *&ve.get_dest_id();
            let o = g.vertices.iter().find(|v| v.v_id == dest_id).unwrap();
            if visited.contains(&o.v_id) {
                continue;
            }
            dfs(o, &g, visited, topo_vec);
        }
        topo_vec.push(vertex.v_id);
    }
}

pub fn sssp(g: Graph) -> i64 {
    let mut visited = HashSet::new();
    // let first_v = &g.vertices[0];
    let mut topo_vec = vec![];
    for first_v in &g.vertices {
        dfs(first_v, &g, &mut visited, &mut topo_vec);
    }
    topo_vec.reverse();
    let mut weights = vec![i64::MAX; topo_vec.len()];
    weights[0] = 0;
    for i in 0..topo_vec.len() {
        let vid = topo_vec[i];
        if let Some(vert) = g.vertices.iter().find(|v| v.v_id == vid) {
            for edge in vert.get_edges() {
                // println!("{:?}", edge);
                let dest_id = edge.get_dest_id();
                let dest_w = edge.get_weight().unwrap();
                let d_pos = topo_vec.iter().position(|did| *did == dest_id).unwrap();
                // println!("dest_id pos: {}, t_w: {}", d_pos, weights[i] + dest_w);
                weights[d_pos] = weights[d_pos].min(weights[i] + dest_w);
            }
        }
        // println!();
    }
    // println!("{:?}", weights);
    // println!("{:?}", topo_vec);
    return weights[weights.len() - 1];
}
