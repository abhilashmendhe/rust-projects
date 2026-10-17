use std::{cmp::Reverse, collections::BinaryHeap};

use _1_graph::graph::Graph;

pub fn lazy_dijk_shortest(g: &Graph) -> i64 {
    let mut distv = vec![i64::MAX; g.vertices.len()];
    distv[0] = 0; // assign start node value to zero which is `0`
    let mut pq = BinaryHeap::new();
    pq.push((Reverse(0), 0 as u32)); // (dist, index)
    while let Some((_dist, index)) = pq.pop() {
        let vertex = g.vertices.iter().find(|v| v.v_id as u32 == index).unwrap();
        for edge in &vertex.edges {
            let dest_id = edge.get_dest_id();
            let weight = edge.get_weight().unwrap();
            distv[dest_id as usize] =
                distv[dest_id as usize].min(distv[vertex.v_id as usize] + weight);
            pq.push((Reverse(weight), dest_id)); // (dist, index)
        }
    }
    distv[distv.len() - 1]
}
